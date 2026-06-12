#![allow(clippy::print_stdout, clippy::print_stderr, missing_docs)]

use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::fs::{self, File};
use std::io::{self, Read, Write as IoWrite};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use boq_core::support::manifest::{self, FixtureEntry, FixtureManifest};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use zip::ZipArchive;
#[cfg(test)]
use zip::ZipWriter;
#[cfg(test)]
use zip::write::SimpleFileOptions;

const MANIFEST_PATH: &str = "gaeb/manifest.toml";
const DOWNLOAD_DIR: &str = "gaeb/.cache/downloads";
const MAX_UNCOMPRESSED_ENTRY_BYTES: u64 = 256 * 1024 * 1024;
const MAX_UNCOMPRESSED_ARCHIVE_BYTES: u64 = 1024 * 1024 * 1024;
const MAX_ARCHIVE_ENTRIES: usize = 20_000;

#[derive(Debug, Default, Deserialize)]
struct Lockfile {
    entries: Option<Vec<LockEntry>>,
}

#[derive(Debug, Deserialize)]
struct LockEntry {
    id: String,
    archive_sha256: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, action] if command == "fixtures" => run_fixtures(action),
        _ => {
            eprintln!("usage: cargo run --bin xtask -- fixtures <download|unpack|manifest|verify>");
            Err("invalid xtask command".into())
        }
    }
}

fn run_fixtures(action: &str) -> Result<(), Box<dyn Error>> {
    match action {
        "download" => download_fixtures(),
        "unpack" => unpack_fixtures(),
        "manifest" => write_lockfile(),
        "verify" => verify_manifest(),
        _ => Err(format!("unknown fixtures action: {action}").into()),
    }
}

fn load_manifest() -> Result<FixtureManifest, Box<dyn Error>> {
    let text = fs::read_to_string(MANIFEST_PATH)?;
    manifest::parse(&text).map_err(Into::into)
}

fn load_lockfile() -> Result<Lockfile, Box<dyn Error>> {
    match fs::read_to_string("gaeb/fixtures.lock") {
        Ok(text) => Ok(toml::from_str(&text)?),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Lockfile::default()),
        Err(error) => Err(error.into()),
    }
}

fn verify_manifest() -> Result<(), Box<dyn Error>> {
    let manifest = load_manifest()?;
    let lockfile = load_lockfile()?;
    let lock_entries = lockfile
        .entries
        .unwrap_or_default()
        .into_iter()
        .map(|entry| (entry.id, entry.archive_sha256))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut failures = Vec::new();

    if let Err(issues) = manifest::validate(&manifest) {
        for issue in issues {
            failures.push(issue.to_string());
        }
    }

    for fixture in &manifest.fixtures {
        if !Path::new(&fixture.target_dir).is_dir() {
            failures.push(format!(
                "target_dir missing for {}: {}",
                fixture.id, fixture.target_dir
            ));
        }
        let expected_checksum = fixture
            .archive_sha256
            .as_ref()
            .or_else(|| lock_entries.get(&fixture.id));
        let archive = archive_path(fixture);
        if archive.exists() {
            let Some(expected) = expected_checksum else {
                failures.push(format!(
                    "downloaded archive lacks locked checksum: {}",
                    fixture.id
                ));
                continue;
            };
            let actual = sha256_file(&archive)?;
            if &actual != expected {
                failures.push(format!("checksum mismatch for {}", fixture.id));
            }
        }
    }

    if failures.is_empty() {
        println!(
            "fixture manifest verified: {} entries",
            manifest.fixtures.len()
        );
        Ok(())
    } else {
        for failure in failures {
            eprintln!("fixture verification error: {failure}");
        }
        Err("fixture manifest verification failed".into())
    }
}

fn download_fixtures() -> Result<(), Box<dyn Error>> {
    let manifest = load_manifest()?;
    let selected = selected_fixture_ids();
    fs::create_dir_all(DOWNLOAD_DIR)?;

    for fixture in manifest
        .fixtures
        .iter()
        .filter(|fixture| should_process(fixture, selected.as_ref()))
    {
        validate_fixture_identity(fixture)?;
        validate_url(&fixture.normalized_url)?;
        let output = archive_path(fixture);
        if output.exists() {
            verify_download_checksum(fixture, &output)?;
            println!("skip existing {} -> {}", fixture.id, output.display());
            continue;
        }
        let temp_output = output.with_extension("download");
        let _ = fs::remove_file(&temp_output);
        println!(
            "download {} -> {}",
            fixture.normalized_url,
            output.display()
        );
        let status = Command::new("curl")
            .arg("--fail")
            .arg("--location")
            .arg("--proto")
            .arg("=https")
            .arg("--max-redirs")
            .arg("5")
            .arg("--connect-timeout")
            .arg("15")
            .arg("--max-time")
            .arg("300")
            .arg("--silent")
            .arg("--show-error")
            .arg("--output")
            .arg(&temp_output)
            .arg(&fixture.normalized_url)
            .status()?;
        if !status.success() {
            let _ = fs::remove_file(&temp_output);
            return Err(format!("curl failed for {}", fixture.id).into());
        }
        verify_download_checksum(fixture, &temp_output)?;
        fs::rename(&temp_output, &output)?;
    }

    Ok(())
}

fn unpack_fixtures() -> Result<(), Box<dyn Error>> {
    let manifest = load_manifest()?;
    let selected = selected_fixture_ids();
    for fixture in manifest
        .fixtures
        .iter()
        .filter(|fixture| should_process(fixture, selected.as_ref()))
    {
        validate_fixture_identity(fixture)?;
        let archive = archive_path(fixture);
        if !archive.exists()
            || archive
                .extension()
                .is_none_or(|ext| !ext.eq_ignore_ascii_case("zip"))
        {
            continue;
        }
        verify_download_checksum(fixture, &archive)?;
        safe_unzip(&archive, Path::new(&fixture.target_dir))?;
    }
    Ok(())
}

fn write_lockfile() -> Result<(), Box<dyn Error>> {
    let manifest = load_manifest()?;
    let selected = selected_fixture_ids();
    let mut output = String::from("version = 1\nresolved_at = \"manual\"\n\n");
    for fixture in manifest
        .fixtures
        .iter()
        .filter(|fixture| should_process(fixture, selected.as_ref()))
    {
        let archive = archive_path(fixture);
        if archive.exists() {
            let checksum = sha256_file(&archive)?;
            output.push_str("[[entries]]\n");
            writeln!(output, "id = \"{}\"", fixture.id)?;
            writeln!(output, "archive = \"{}\"", archive.display())?;
            writeln!(output, "archive_sha256 = \"{checksum}\"\n")?;
        }
    }
    fs::write("gaeb/fixtures.lock", output)?;
    Ok(())
}

fn selected_fixture_ids() -> Option<BTreeSet<String>> {
    env::var("BOQ_FIXTURE_IDS").ok().map(|value| {
        value
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    })
}

fn should_process(fixture: &FixtureEntry, selected: Option<&BTreeSet<String>>) -> bool {
    selected.is_none_or(|ids| ids.contains(&fixture.id))
}

fn archive_path(fixture: &FixtureEntry) -> PathBuf {
    let suffix = Path::new(&fixture.normalized_url)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("download.bin");
    Path::new(DOWNLOAD_DIR).join(format!("{}__{}", fixture.id, suffix))
}

fn validate_fixture_identity(fixture: &FixtureEntry) -> Result<(), Box<dyn Error>> {
    manifest::validate_identity(fixture).map_err(|message| -> Box<dyn Error> { message.into() })?;
    // Belt-and-braces: reject unsafe archive extraction paths via the
    // path-component check even when string-level validation already passed.
    reject_unsafe_path(Path::new(&fixture.target_dir))?;
    Ok(())
}

fn validate_url(url: &str) -> Result<(), Box<dyn Error>> {
    if !url.starts_with("https://") {
        return Err(format!("fixture URL must use https: {url}").into());
    }
    let host = url
        .strip_prefix("https://")
        .and_then(|rest| rest.split('/').next())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let allowed = [
        "www.bvbs.de",
        "www.gaeb.de",
        "github.com",
        "gist.github.com",
        "www.gaeb-online.de",
    ];
    if !allowed.contains(&host.as_str()) {
        return Err(format!("fixture host is not allowlisted: {host}").into());
    }
    Ok(())
}

fn verify_download_checksum(fixture: &FixtureEntry, archive: &Path) -> Result<(), Box<dyn Error>> {
    let expected = match &fixture.archive_sha256 {
        Some(checksum) => Some(checksum.clone()),
        None => locked_checksum(&fixture.id)?,
    };
    let Some(expected) = expected else {
        return Err(format!(
            "missing checksum for {}; run `cargo run --bin xtask -- fixtures manifest` after reviewing source terms",
            fixture.id
        )
        .into());
    };
    let actual = sha256_file(archive)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("checksum mismatch for {}", fixture.id).into())
    }
}

fn locked_checksum(id: &str) -> Result<Option<String>, Box<dyn Error>> {
    Ok(load_lockfile()?
        .entries
        .unwrap_or_default()
        .into_iter()
        .find(|entry| entry.id == id)
        .map(|entry| entry.archive_sha256))
}

fn safe_unzip(archive: &Path, target_dir: &Path) -> Result<(), Box<dyn Error>> {
    reject_unsafe_path(target_dir)?;
    if target_dir.is_absolute() || !target_dir.starts_with("gaeb") {
        return Err(format!("target_dir must stay under gaeb/: {}", target_dir.display()).into());
    }
    fs::create_dir_all(target_dir)?;
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file)?;
    if zip.len() > MAX_ARCHIVE_ENTRIES {
        return Err(format!("zip contains too many entries: {}", zip.len()).into());
    }
    let mut total_uncompressed = 0_u64;

    for index in 0..zip.len() {
        let mut entry = zip.by_index(index)?;
        let enclosed = entry
            .enclosed_name()
            .ok_or_else(|| format!("unsafe zip path in {}", archive.display()))?;
        reject_unsafe_path(&enclosed)?;

        if entry.is_dir() {
            fs::create_dir_all(target_dir.join(enclosed))?;
            continue;
        }
        if entry.size() > MAX_UNCOMPRESSED_ENTRY_BYTES {
            return Err(format!("zip entry too large: {}", entry.name()).into());
        }
        total_uncompressed = total_uncompressed.saturating_add(entry.size());
        if total_uncompressed > MAX_UNCOMPRESSED_ARCHIVE_BYTES {
            return Err(format!("zip archive too large: {}", archive.display()).into());
        }

        let output_path = target_dir.join(enclosed);
        if is_executable_payload(&output_path) {
            let quarantine = target_dir.join("_quarantine");
            fs::create_dir_all(&quarantine)?;
            let file_name = output_path
                .file_name()
                .ok_or("missing executable file name")?;
            copy_entry(&mut entry, &quarantine.join(file_name))?;
        } else {
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            copy_entry(&mut entry, &output_path)?;
        }
    }

    Ok(())
}

fn is_executable_payload(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "exe" | "bat" | "cmd" | "com" | "ps1" | "sh" | "msi"
            )
        })
}

fn reject_unsafe_path(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        Err(format!("unsafe archive path: {}", path.display()).into())
    } else {
        Ok(())
    }
}

fn copy_entry<R: Read>(reader: &mut R, output_path: &Path) -> io::Result<()> {
    let mut output = File::create(output_path)?;
    io::copy(reader, &mut output)?;
    output.flush()
}

fn sha256_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 8192];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    let digest = hasher.finalize();
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(output, "{byte:02x}")?;
    }
    Ok(output)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static CWD_LOCK: Mutex<()> = Mutex::new(());

    fn fixture(id: &str, target_dir: &str, archive_sha256: Option<String>) -> FixtureEntry {
        FixtureEntry {
            id: id.to_owned(),
            source_url: "https://www.bvbs.de/example.zip".to_owned(),
            normalized_url: "https://www.bvbs.de/example.zip".to_owned(),
            source_family: "bvbs".to_owned(),
            process_domain: "ava".to_owned(),
            gaeb_version: "gaeb_xml_3_3".to_owned(),
            phase: "x81".to_owned(),
            target_dir: target_dir.to_owned(),
            support_status: "supported".to_owned(),
            ci_policy: "download_on_demand".to_owned(),
            license_note: "test".to_owned(),
            test_mapping: vec!["test".to_owned()],
            archive_sha256,
        }
    }

    #[test]
    fn rejects_invalid_fixture_id_and_unsafe_target_dir() {
        let bad_id = fixture("Bad-ID", "gaeb/test", None);
        assert!(validate_fixture_identity(&bad_id).is_err());

        let bad_path = fixture("valid_id", "../outside", None);
        assert!(validate_fixture_identity(&bad_path).is_err());
    }

    #[test]
    fn validates_allowlisted_https_urls() {
        assert!(validate_url("https://www.bvbs.de/file.zip").is_ok());
        assert!(validate_url("http://www.bvbs.de/file.zip").is_err());
        assert!(validate_url("https://evil.example/file.zip").is_err());
    }

    #[test]
    fn missing_checksum_is_hard_error_for_existing_archive() {
        let temp =
            std::env::temp_dir().join(format!("boq-core-xtask-checksum-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&temp);
        std::fs::create_dir_all(&temp).expect("temp dir");
        let archive = temp.join("archive.zip");
        std::fs::write(&archive, b"zip bytes").expect("archive bytes");
        let fixture = fixture("unpinned_fixture", "gaeb/test", None);

        let error =
            verify_download_checksum(&fixture, &archive).expect_err("unpinned archive must fail");
        assert!(error.to_string().contains("missing checksum"));
        let _ = std::fs::remove_dir_all(temp);
    }

    #[test]
    fn executable_payload_detection_is_case_insensitive() {
        assert!(is_executable_payload(Path::new("tool.EXE")));
        assert!(is_executable_payload(Path::new("script.Ps1")));
        assert!(!is_executable_payload(Path::new("sample.x81")));
    }

    #[test]
    fn safe_unzip_quarantines_executable_payloads() {
        let temp = test_temp_dir("quarantine");
        let archive = temp.join("payload.zip");
        create_zip(
            &archive,
            &[
                ("folder/sample.x81", &b"<GAEB/>"[..]),
                ("tools/setup.EXE", &b"binary"[..]),
            ],
        );
        let target = temp.join("gaeb/quarantine");
        let _guard = CWD_LOCK.lock().expect("cwd lock");
        let previous_dir = std::env::current_dir().expect("current dir");
        std::env::set_current_dir(&temp).expect("change current dir");

        safe_unzip(Path::new("payload.zip"), Path::new("gaeb/quarantine"))
            .expect("safe unzip should succeed");

        assert!(target.join("folder/sample.x81").exists());
        assert!(!target.join("tools/setup.EXE").exists());
        assert!(target.join("_quarantine/setup.EXE").exists());
        std::env::set_current_dir(previous_dir).expect("restore current dir");
        let _ = std::fs::remove_dir_all(temp);
    }

    #[test]
    fn safe_unzip_rejects_zip_path_traversal() {
        let temp = test_temp_dir("traversal");
        let archive = temp.join("traversal.zip");
        create_zip(&archive, &[("../evil.x81", &b"evil"[..])]);
        let _guard = CWD_LOCK.lock().expect("cwd lock");
        let previous_dir = std::env::current_dir().expect("current dir");
        std::env::set_current_dir(&temp).expect("change current dir");

        let error = safe_unzip(Path::new("traversal.zip"), Path::new("gaeb/traversal"))
            .expect_err("path traversal should fail");

        assert!(error.to_string().contains("unsafe zip path"));
        std::env::set_current_dir(previous_dir).expect("restore current dir");
        let _ = std::fs::remove_dir_all(temp);
    }

    fn test_temp_dir(name: &str) -> PathBuf {
        let temp =
            std::env::temp_dir().join(format!("boq-core-xtask-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&temp);
        std::fs::create_dir_all(&temp).expect("temp dir");
        temp
    }

    fn create_zip(path: &Path, entries: &[(&str, &[u8])]) {
        let file = File::create(path).expect("zip file");
        let mut zip = ZipWriter::new(file);
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        for (name, bytes) in entries {
            zip.start_file(*name, options).expect("start zip file");
            zip.write_all(bytes).expect("write zip entry");
        }
        zip.finish().expect("finish zip");
    }
}
