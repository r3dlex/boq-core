//! Command-line service contract adapter for boq-core.

use std::path::PathBuf;

use boq_core::service_contract::{AnalyzeFormatHint, AnalyzeInput, analyze_bytes};
use boq_core::service_obra_import::{ObraImportInput, convert_bytes_to_obra_import};
use boq_core::service_support_manifest::export_embedded_support_manifest;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let command = args.next().ok_or_else(usage)?;
    if command == "capabilities" {
        if args.next().is_some() {
            return Err(usage());
        }
        let report = export_embedded_support_manifest().map_err(|error| error.to_string())?;
        let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;
        println!("{json}");
        return Ok(());
    }
    if command != "analyze" && command != "obra-import" {
        return Err(usage());
    }
    let path = PathBuf::from(args.next().ok_or_else(usage)?);
    let mut format_hint = None;
    while let Some(arg) = args.next() {
        if arg == "--format" {
            let value = args.next().ok_or_else(usage)?;
            format_hint = AnalyzeFormatHint::parse(&value);
            if format_hint.is_none() {
                return Err(format!("unsupported --format value: {value}"));
            }
        } else {
            return Err(format!("unknown argument: {arg}"));
        }
    }
    let bytes = std::fs::read(&path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let source_uri = Some(path.display().to_string());
    let json = if command == "analyze" {
        let report = analyze_bytes(&AnalyzeInput {
            bytes: &bytes,
            source_uri,
            format_hint,
        });
        serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?
    } else {
        let report = convert_bytes_to_obra_import(&ObraImportInput {
            bytes: &bytes,
            source_uri,
            format_hint,
        });
        serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?
    };
    println!("{json}");
    Ok(())
}

fn usage() -> String {
    "usage: boq-core-service analyze <path> [--format gaeb-xml|gaeb90] | boq-core-service obra-import <path> [--format gaeb-xml|gaeb90] | boq-core-service capabilities".to_owned()
}
