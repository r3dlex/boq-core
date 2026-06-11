//! GAEB DA XML parser foundation.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::OnceLock;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::VERSION;
use crate::checksum::sha256_hex;
use crate::error::{ParseError, ValidationFinding};
use crate::format::detect_path;
use crate::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, GaebDocument, GaebDocumentSummary, GaebFormat, GaebPhase,
    Metadata, RichText, SourceProvenance,
};
use crate::support::{SupportCapabilities, SupportStatus};

pub mod bau;
pub mod writer;

pub use writer::{schema_validation_findings, write_string};

/// Parses GAEB XML text into a loss-aware document.
///
/// # Errors
///
/// Returns a parse error when XML cannot be read or decoded.
pub fn parse_str(source: &str, source_uri: Option<String>) -> Result<GaebDocument, ParseError> {
    let detected = source_uri.as_deref().map_or_else(
        || crate::format::DetectedFormat {
            format: GaebFormat::GaebXml,
            phase: None,
            extension: None,
        },
        detect_path,
    );
    let mut parser = XmlParser::new(source, detected.phase);
    parser.parse(source_uri, Some(sha256_hex(source.as_bytes())))
}

/// Parses a GAEB XML file from disk.
///
/// # Errors
///
/// Returns a parse error when the file cannot be read or XML cannot be parsed.
pub fn parse_file(path: impl AsRef<Path>) -> Result<GaebDocument, ParseError> {
    let path_ref = path.as_ref();
    let bytes = std::fs::read(path_ref).map_err(|error| ParseError {
        code: "xml_read_failed".to_owned(),
        message: error.to_string(),
        location: Some(path_ref.display().to_string()),
    })?;
    let source = String::from_utf8(bytes).map_err(|error| ParseError {
        code: "xml_decode_failed".to_owned(),
        message: error.to_string(),
        location: Some(path_ref.display().to_string()),
    })?;
    parse_str(&source, Some(path_ref.display().to_string()))
}

struct XmlParser<'a> {
    reader: Reader<&'a [u8]>,
    buffer: Vec<u8>,
    phase: Option<GaebPhase>,
    version: Option<String>,
    namespace: Option<String>,
    title: Option<String>,
    project_name: Option<String>,
    nodes: Vec<BoqNode>,
    findings: Vec<ValidationFinding>,
}

impl<'a> XmlParser<'a> {
    fn new(source: &'a str, phase: Option<GaebPhase>) -> Self {
        let mut reader = Reader::from_str(source);
        reader.config_mut().trim_text(true);
        Self {
            reader,
            buffer: Vec::new(),
            phase,
            version: None,
            namespace: None,
            title: None,
            project_name: None,
            nodes: Vec::new(),
            findings: Vec::new(),
        }
    }

    fn parse(
        &mut self,
        source_uri: Option<String>,
        checksum: Option<String>,
    ) -> Result<GaebDocument, ParseError> {
        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(start)) => {
                    let owned = start.into_owned();
                    self.handle_start(&owned, false)?;
                }
                Ok(Event::Empty(start)) => {
                    let owned = start.into_owned();
                    self.handle_start(&owned, true)?;
                }
                Ok(Event::Eof) => break,
                Err(error) => {
                    return Err(ParseError {
                        code: "xml_parse_failed".to_owned(),
                        message: error.to_string(),
                        location: None,
                    });
                }
                _ => {}
            }
            self.buffer.clear();
        }

        let title = self.title.clone().unwrap_or_else(|| "GAEB BoQ".to_owned());
        let summary = GaebDocumentSummary {
            format: GaebFormat::GaebXml,
            version: self.version.clone(),
            phase: self.phase.clone(),
            title: Some(title.clone()),
            project_name: self.project_name.clone(),
        };
        let support = support_policy(
            self.version.as_deref(),
            self.phase.as_ref(),
            source_uri.as_deref(),
        );
        let source = SourceProvenance {
            source_uri,
            source_format: GaebFormat::GaebXml,
            gaeb_version: self.version.clone(),
            phase: self.phase.clone(),
            checksum,
            parser_version: VERSION.to_owned(),
        };
        let mut metadata = Metadata::new();
        if let Some(namespace) = &self.namespace {
            metadata.insert("gaeb.namespace".to_owned(), serde_json::json!(namespace));
        }
        metadata.insert(
            "gaeb.support_policy".to_owned(),
            serde_json::json!({
                "status": support.status,
                "reason": support.reason,
            }),
        );

        Ok(GaebDocument {
            source,
            summary,
            boq: Boq {
                title,
                nodes: self.nodes.clone(),
                currency: None,
                metadata,
            },
            capabilities: support.capabilities,
            support_status: support.status,
            findings: self.findings.clone(),
            metadata: BTreeMap::new(),
        })
    }

    fn handle_start(&mut self, start: &BytesStart<'_>, is_empty: bool) -> Result<(), ParseError> {
        let local = local_name(start.name());
        match local.as_str() {
            "GAEB" => self.capture_root(start),
            "Version" => self.version = self.read_text_for(start.name())?,
            "Name" if self.project_name.is_none() => {
                self.project_name = self.read_text_for(start.name())?;
            }
            "BoQInfo" | "BoQ" if self.title.is_none() => {
                self.title = Some(
                    self.project_name
                        .clone()
                        .unwrap_or_else(|| "GAEB BoQ".to_owned()),
                );
            }
            "Item" => {
                let node = self.parse_item(
                    start,
                    is_empty,
                    i32::try_from(self.nodes.len()).unwrap_or(i32::MAX),
                )?;
                self.nodes.push(node);
            }
            "Section" | "BoQCtgy" => {
                let sort_order = i32::try_from(self.nodes.len()).unwrap_or(i32::MAX);
                let node = if is_empty {
                    Self::make_section(start, sort_order)
                } else {
                    self.parse_section(start, sort_order)?
                };
                self.nodes.push(node);
            }
            _ => {}
        }
        Ok(())
    }

    fn capture_root(&mut self, start: &BytesStart<'_>) {
        for attr in start.attributes().flatten() {
            if attr.key.as_ref() == b"xmlns" {
                self.namespace = Some(String::from_utf8_lossy(attr.value.as_ref()).to_string());
            }
        }
    }

    fn make_section(start: &BytesStart<'_>, sort_order: i32) -> BoqNode {
        let ordinal = attr_value(start, b"ID").unwrap_or_else(|| format!("section_{sort_order}"));
        let title = attr_value(start, b"RNoPart").unwrap_or_else(|| ordinal.clone());
        BoqNode {
            ordinal,
            title,
            kind: BoqNodeKind::Chapter,
            children: Vec::new(),
            item: None,
            sort_order,
            metadata: BTreeMap::new(),
        }
    }

    fn parse_section(
        &mut self,
        start: &BytesStart<'_>,
        sort_order: i32,
    ) -> Result<BoqNode, ParseError> {
        let end_name = local_name(start.name());
        let mut section = Self::make_section(start, sort_order);
        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(child)) => {
                    let owned = child.into_owned();
                    let local = local_name(owned.name());
                    match local.as_str() {
                        "Item" => {
                            let child = self.parse_item(
                                &owned,
                                false,
                                i32::try_from(section.children.len()).unwrap_or(i32::MAX),
                            )?;
                            section.children.push(child);
                        }
                        "Section" | "BoQCtgy" => {
                            let child = self.parse_section(
                                &owned,
                                i32::try_from(section.children.len()).unwrap_or(i32::MAX),
                            )?;
                            section.children.push(child);
                        }
                        _ => {}
                    }
                }
                Ok(Event::Empty(child)) => {
                    let owned = child.into_owned();
                    let local = local_name(owned.name());
                    match local.as_str() {
                        "Item" => {
                            let child = self.parse_item(
                                &owned,
                                true,
                                i32::try_from(section.children.len()).unwrap_or(i32::MAX),
                            )?;
                            section.children.push(child);
                        }
                        "Section" | "BoQCtgy" => {
                            let child = Self::make_section(
                                &owned,
                                i32::try_from(section.children.len()).unwrap_or(i32::MAX),
                            );
                            section.children.push(child);
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(end)) if local_name(end.name()) == end_name => break,
                Ok(Event::Eof) => {
                    return Err(ParseError {
                        code: "xml_unclosed_section".to_owned(),
                        message: format!("section ended before closing {end_name} tag"),
                        location: Some(section.ordinal.clone()),
                    });
                }
                Err(error) => {
                    return Err(ParseError {
                        code: "xml_parse_failed".to_owned(),
                        message: error.to_string(),
                        location: Some(section.ordinal.clone()),
                    });
                }
                _ => {}
            }
            self.buffer.clear();
        }
        Ok(section)
    }

    fn parse_item(
        &mut self,
        start: &BytesStart<'_>,
        is_empty: bool,
        sort_order: i32,
    ) -> Result<BoqNode, ParseError> {
        let ordinal = attr_value(start, b"ID").unwrap_or_else(|| format!("item_{sort_order}"));
        let rno_part = attr_value(start, b"RNoPart").unwrap_or_else(|| ordinal.clone());
        let mut title = rno_part.clone();
        let mut item = BoqItem {
            short_text: title.clone(),
            long_text: None,
            quantity: Decimal::ZERO,
            unit: String::new(),
            unit_price: None,
            total_price: None,
            notes: None,
            metadata: BTreeMap::new(),
        };
        let mut metadata = BTreeMap::new();
        metadata.insert("gaeb.rno_part".to_owned(), serde_json::json!(rno_part));

        if is_empty {
            self.findings.push(
                ValidationFinding::warning(
                    "gaeb_xml_empty_item",
                    "item had no child payload; title and ordinal were derived from attributes",
                )
                .at(ordinal.clone()),
            );
        } else {
            self.read_item_body(&mut item, &mut title, &mut metadata)?;
        }

        Ok(BoqNode {
            ordinal,
            title: title.clone(),
            kind: BoqNodeKind::Item,
            children: Vec::new(),
            item: Some(item),
            sort_order,
            metadata,
        })
    }

    fn read_item_body(
        &mut self,
        item: &mut BoqItem,
        title: &mut String,
        metadata: &mut Metadata,
    ) -> Result<(), ParseError> {
        let mut depth = 1_usize;
        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(start)) => {
                    let owned = start.into_owned();
                    let local = local_name(owned.name());
                    match local.as_str() {
                        "Qty" => item.quantity = self.read_decimal_for(owned.name())?,
                        "QU" => item.unit = self.read_text_for(owned.name())?.unwrap_or_default(),
                        "UP" => item.unit_price = self.read_optional_decimal_for(owned.name())?,
                        "IT" => item.total_price = self.read_optional_decimal_for(owned.name())?,
                        "Description" => {
                            let description = self.read_description_text()?;
                            if !description.is_empty() {
                                title.clone_from(&description);
                                item.short_text.clone_from(&description);
                                item.long_text = Some(RichText::Plain(description));
                            }
                        }
                        "LumpSumItem" => {
                            let value = self.read_text_for(owned.name())?.unwrap_or_default();
                            metadata
                                .insert("gaeb.lump_sum_item".to_owned(), serde_json::json!(value));
                        }
                        _ => depth = depth.saturating_add(1),
                    }
                }
                Ok(Event::Empty(start)) => {
                    let local = local_name(start.name());
                    metadata.insert(format!("gaeb.empty.{local}"), serde_json::json!(true));
                }
                Ok(Event::End(end)) => {
                    if local_name(end.name()) == "Item" {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            break;
                        }
                    } else {
                        depth = depth.saturating_sub(1).max(1);
                    }
                }
                Ok(Event::Eof) => {
                    return Err(ParseError {
                        code: "xml_unclosed_item".to_owned(),
                        message: "item ended before closing Item tag".to_owned(),
                        location: None,
                    });
                }
                Err(error) => {
                    return Err(ParseError {
                        code: "xml_parse_failed".to_owned(),
                        message: error.to_string(),
                        location: None,
                    });
                }
                _ => {}
            }
            self.buffer.clear();
        }
        Ok(())
    }

    fn read_description_text(&mut self) -> Result<String, ParseError> {
        let mut depth = 1_usize;
        let mut parts = Vec::new();
        let mut saw_markup = false;
        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(_)) => {
                    saw_markup = true;
                    depth = depth.saturating_add(1);
                }
                Ok(Event::Empty(_)) => saw_markup = true,
                Ok(Event::Text(text)) => {
                    let decoded = text.decode().map_err(|error| ParseError {
                        code: "xml_text_decode_failed".to_owned(),
                        message: error.to_string(),
                        location: Some("Description".to_owned()),
                    })?;
                    let trimmed = decoded.trim();
                    if !trimmed.is_empty() {
                        parts.push(trimmed.to_owned());
                    }
                }
                Ok(Event::End(_)) => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        break;
                    }
                }
                Ok(Event::Eof) => {
                    return Err(ParseError {
                        code: "xml_unclosed_description".to_owned(),
                        message: "description ended before closing Description tag".to_owned(),
                        location: None,
                    });
                }
                Err(error) => {
                    return Err(ParseError {
                        code: "xml_parse_failed".to_owned(),
                        message: error.to_string(),
                        location: None,
                    });
                }
                _ => {}
            }
            self.buffer.clear();
        }
        if saw_markup && !parts.is_empty() {
            self.findings.push(ValidationFinding::warning(
                "gaeb_xml_description_plain_text_only",
                "description rich markup was normalized to plain text; original XML is not yet roundtripped",
            ));
        }
        Ok(parts.join("\n"))
    }

    fn read_decimal_for(&mut self, end: QName<'_>) -> Result<Decimal, ParseError> {
        self.read_optional_decimal_for(end)
            .map(|value| value.unwrap_or(Decimal::ZERO))
    }

    fn read_optional_decimal_for(&mut self, end: QName<'_>) -> Result<Option<Decimal>, ParseError> {
        let Some(text) = self.read_text_for(end)? else {
            return Ok(None);
        };
        let normalized = text.trim().replace(',', ".");
        if normalized.is_empty() {
            return Ok(None);
        }
        Decimal::from_str_exact(&normalized)
            .map(Some)
            .map_err(|error| ParseError {
                code: "xml_decimal_parse_failed".to_owned(),
                message: error.to_string(),
                location: Some(String::from_utf8_lossy(end.as_ref()).to_string()),
            })
    }

    fn read_text_for(&mut self, end: QName<'_>) -> Result<Option<String>, ParseError> {
        match self.reader.read_text(end) {
            Ok(text) => text
                .decode()
                .map(|decoded| Some(decoded.into_owned()))
                .map_err(|error| ParseError {
                    code: "xml_text_decode_failed".to_owned(),
                    message: error.to_string(),
                    location: Some(String::from_utf8_lossy(end.as_ref()).to_string()),
                }),
            Err(error) => Err(ParseError {
                code: "xml_text_read_failed".to_owned(),
                message: error.to_string(),
                location: Some(String::from_utf8_lossy(end.as_ref()).to_string()),
            }),
        }
    }
}

#[derive(Debug, Clone)]
struct SupportPolicy {
    status: SupportStatus,
    capabilities: SupportCapabilities,
    reason: String,
}

const FIXTURE_MANIFEST_TOML: &str = include_str!("../../gaeb/manifest.toml");
static FIXTURE_SUPPORT_REGISTRY: OnceLock<Result<Vec<FixtureSupportEntry>, String>> =
    OnceLock::new();

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    fixtures: Vec<FixtureManifestRow>,
}

#[derive(Debug, Deserialize)]
struct FixtureManifestRow {
    id: String,
    process_domain: String,
    gaeb_version: String,
    phase: String,
    target_dir: String,
    support_status: String,
}

#[derive(Debug)]
struct FixtureSupportEntry {
    id: String,
    process_domain: String,
    gaeb_version: String,
    phase_code: String,
    target_dir: String,
    support_status: String,
}

fn support_policy(
    version: Option<&str>,
    phase: Option<&GaebPhase>,
    source_uri: Option<&str>,
) -> SupportPolicy {
    if let Some(source_uri) = source_uri {
        match fixture_support_entry(source_uri) {
            Ok(Some(entry)) => {
                if let Some(policy) = support_policy_from_fixture_entry(version, phase, entry) {
                    return policy;
                }
            }
            Ok(None) => {}
            Err(error) => {
                return SupportPolicy {
                    status: SupportStatus::SupportedParseOnly,
                    capabilities: SupportCapabilities::parse_only(),
                    reason: format!(
                        "embedded GAEB fixture manifest failed to parse; support registry disabled: {error}"
                    ),
                };
            }
        }
    }

    SupportPolicy {
        status: SupportStatus::SupportedParseOnly,
        capabilities: SupportCapabilities::parse_only(),
        reason: "GAEB XML parsed outside manifest-backed support registry".to_owned(),
    }
}

fn support_policy_from_fixture_entry(
    version: Option<&str>,
    phase: Option<&GaebPhase>,
    entry: &FixtureSupportEntry,
) -> Option<SupportPolicy> {
    let phase_code = phase.map(|phase| phase.code.as_str());
    if version != Some(entry.gaeb_version.as_str()) || phase_code != Some(entry.phase_code.as_str())
    {
        return None;
    }

    let (status, capabilities, summary) = match entry.support_status.as_str() {
        "supported" if entry.process_domain == "ava" => (
            SupportStatus::Supported,
            SupportCapabilities::supported_import(),
            "supported AVA import fixture",
        ),
        "supported_parse_only" => (
            SupportStatus::SupportedParseOnly,
            SupportCapabilities::parse_only(),
            "supported parse-only fixture",
        ),
        "future_track" => (
            SupportStatus::SupportedParseOnly,
            SupportCapabilities::parse_only(),
            "future-track fixture parsed without adapter/export promotion",
        ),
        "reference_only" => (
            SupportStatus::SupportedParseOnly,
            SupportCapabilities::parse_only(),
            "reference-only fixture parsed without support promotion",
        ),
        _ => (
            SupportStatus::SupportedParseOnly,
            SupportCapabilities::parse_only(),
            "manifest fixture parsed without support promotion",
        ),
    };

    Some(SupportPolicy {
        status,
        capabilities,
        reason: format!("manifest fixture {}: {summary}", entry.id),
    })
}

fn fixture_support_entry(
    source_uri: &str,
) -> Result<Option<&'static FixtureSupportEntry>, &'static str> {
    let normalized = normalize_source_path(source_uri);
    Ok(fixture_support_registry()?
        .iter()
        .find(|entry| source_path_matches_target_dir(&normalized, &entry.target_dir)))
}

fn source_path_matches_target_dir(source_path: &str, target_dir: &str) -> bool {
    source_path == target_dir
        || source_path.starts_with(&format!("{target_dir}/"))
        || source_path.ends_with(&format!("/{target_dir}"))
        || source_path.contains(&format!("/{target_dir}/"))
}

fn fixture_support_registry() -> Result<&'static [FixtureSupportEntry], &'static str> {
    match FIXTURE_SUPPORT_REGISTRY.get_or_init(|| {
        toml::from_str::<FixtureManifest>(FIXTURE_MANIFEST_TOML)
            .map(|manifest| {
                manifest
                    .fixtures
                    .into_iter()
                    .filter_map(FixtureSupportEntry::from_manifest_row)
                    .collect()
            })
            .map_err(|error| error.to_string())
    }) {
        Ok(entries) => Ok(entries.as_slice()),
        Err(error) => Err(error.as_str()),
    }
}

impl FixtureSupportEntry {
    fn from_manifest_row(row: FixtureManifestRow) -> Option<Self> {
        let gaeb_version = gaeb_xml_version(&row.gaeb_version)?;
        let phase_code = phase_code(&row.phase)?;
        Some(Self {
            id: row.id,
            process_domain: row.process_domain,
            gaeb_version,
            phase_code,
            target_dir: normalize_source_path(&row.target_dir),
            support_status: row.support_status,
        })
    }
}

fn gaeb_xml_version(value: &str) -> Option<String> {
    value
        .strip_prefix("gaeb_xml_3_")
        .map(|suffix| format!("3.{suffix}"))
}

fn phase_code(value: &str) -> Option<String> {
    value
        .strip_prefix('x')
        .filter(|code| !code.is_empty() && code.chars().all(|ch| ch.is_ascii_digit()))
        .map(ToOwned::to_owned)
}

fn normalize_source_path(value: &str) -> String {
    value
        .replace('\\', "/")
        .trim_start_matches("./")
        .trim_end_matches('/')
        .to_ascii_lowercase()
}

fn local_name(name: QName<'_>) -> String {
    let raw = name.as_ref();
    let after_prefix = raw.rsplit(|byte| *byte == b':').next().unwrap_or(raw);
    String::from_utf8_lossy(after_prefix).to_string()
}

fn attr_value(start: &BytesStart<'_>, key: &[u8]) -> Option<String> {
    start
        .attributes()
        .flatten()
        .find(|attr| attr.key.as_ref() == key)
        .map(|attr| String::from_utf8_lossy(attr.value.as_ref()).to_string())
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    #[test]
    fn parses_minimal_ava_x81_document() {
        let document = parse_str(
            include_str!("../../tests/fixtures/synthetic/minimal_ava.x81"),
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
        )
        .expect("synthetic AVA XML should parse");

        assert_eq!(document.summary.version.as_deref(), Some("3.3"));
        assert_eq!(
            document
                .source
                .phase
                .as_ref()
                .map(|phase| phase.code.as_str()),
            Some("81")
        );
        assert_eq!(document.boq.nodes.len(), 1);
        assert_eq!(document.support_status, SupportStatus::Supported);
        assert!(document.source.checksum.is_some());

        let item = document.boq.nodes[0]
            .item
            .as_ref()
            .expect("item payload should exist");
        assert_eq!(item.quantity, Decimal::new(1250, 2));
        assert_eq!(item.unit, "m2");
        assert_eq!(item.unit_price, None);
        assert!(item.short_text.contains("Concrete slab"));
    }

    #[test]
    fn parses_sections_and_fallback_item_fields_without_source_uri_as_parse_only() {
        let document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQInfo/><BoQBody><Section ID="01"/><Item/></BoQBody></BoQ></Project></GAEB>"#,
            None,
        )
        .expect("fallback XML should parse");

        assert_eq!(document.boq.title, "GAEB BoQ");
        assert!(document.source.source_uri.is_none());
        assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
        assert_eq!(document.boq.nodes.len(), 2);
        assert_eq!(
            document.boq.nodes[0].kind,
            crate::model::BoqNodeKind::Chapter
        );
        assert_eq!(document.boq.nodes[0].ordinal, "01");
        assert_eq!(document.boq.nodes[1].ordinal, "item_1");
    }

    #[test]
    fn preserves_nested_sections_items_and_empty_children() {
        let document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><BoQCtgy ID="01" RNoPart="01"><Item ID="01.001" RNoPart="001"><Qty>1</Qty><QU>m</QU><Marker/></Item><Section ID="01.01" RNoPart="01"/><BoQCtgy ID="01.02" RNoPart="02"><Item ID="01.002" RNoPart="002"><Qty>2</Qty><QU>m2</QU></Item></BoQCtgy><Item ID="01.003"/></BoQCtgy></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/nested.X81".to_owned()),
        )
        .expect("nested hierarchy should parse");

        let root = &document.boq.nodes[0];
        assert_eq!(root.kind, BoqNodeKind::Chapter);
        assert_eq!(root.children.len(), 4);
        assert_eq!(root.children[0].ordinal, "01.001");
        assert_eq!(
            root.children[0].metadata.get("gaeb.empty.Marker"),
            Some(&serde_json::json!(true))
        );
        assert_eq!(root.children[1].kind, BoqNodeKind::Chapter);
        assert_eq!(root.children[2].children[0].ordinal, "01.002");
        assert_eq!(root.children[3].ordinal, "01.003");
        assert!(
            document
                .findings
                .iter()
                .any(|finding| finding.code == "gaeb_xml_empty_item")
        );
    }

    #[test]
    fn parses_priced_item_fields() {
        let document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>AVA Test</Name><BoQ><BoQBody><Item ID="A" RNoPart="10"><Qty>2.500</Qty><QU>m</QU><UP>3.200</UP><IT>8.00</IT><Description><CompleteText><DetailTxt><Text><p>Pipe trench</p></Text></DetailTxt></CompleteText></Description></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x86/test.X86".to_owned()),
        )
        .expect("priced item should parse");
        let item = document.boq.nodes[0].item.as_ref().expect("item payload");
        assert_eq!(item.quantity, Decimal::new(2500, 3));
        assert_eq!(item.unit, "m");
        assert_eq!(item.unit_price, Some(Decimal::new(3200, 3)));
        assert_eq!(item.total_price, Some(Decimal::new(800, 2)));
        assert_eq!(item.short_text, "Pipe trench");
        assert_eq!(
            document.boq.nodes[0].metadata.get("gaeb.rno_part"),
            Some(&serde_json::json!("10"))
        );
        assert_eq!(document.support_status, SupportStatus::Supported);
        assert_eq!(
            document.findings[0].code,
            "gaeb_xml_description_plain_text_only"
        );
    }

    #[test]
    fn fixture_support_registry_loads_manifest_rows() {
        let registry = fixture_support_registry().expect("embedded fixture manifest should parse");

        assert!(registry.iter().any(|entry| entry.id == "bvbs_xml33_ava_x81"
            && entry.support_status == "supported"
            && entry.process_domain == "ava"));
        assert!(registry.iter().any(|entry| entry.id == "bvbs_xml33_bau_x83"
            && entry.support_status == "future_track"
            && entry.process_domain == "construction_execution"));
    }

    #[test]
    fn support_policy_matches_absolute_fixture_paths() {
        let source_path = std::env::current_dir()
            .expect("current directory")
            .join("gaeb/bvbs/gaeb_xml_3_3/ava/x81/test.X81");
        let document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some(source_path.display().to_string()),
        )
        .expect("document should parse");

        assert_eq!(document.support_status, SupportStatus::Supported);
        assert!(document.capabilities.adapt_to_obra);
    }

    #[test]
    fn support_policy_rejects_non_ava_paths_containing_ava_text() {
        let document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/not_ava_but_has_ava_text/x81/test.X81".to_owned()),
        )
        .expect("document should parse");
        assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
        assert!(!document.capabilities.adapt_to_obra);
    }

    #[test]
    fn support_policy_keeps_xml33_bau_x83_x84_parse_only_until_fixtures_are_locked() {
        for (path, phase) in [
            (
                "gaeb/bvbs/gaeb_xml_3_3/construction_execution/x83/test.X83",
                "83",
            ),
            (
                "gaeb/bvbs/gaeb_xml_3_3/construction_execution/x84/test.X84",
                "84",
            ),
        ] {
            let document = parse_str(
                r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
                Some(path.to_owned()),
            )
            .expect("Bau document should parse");

            assert_eq!(
                document
                    .source
                    .phase
                    .as_ref()
                    .map(|phase| phase.code.as_str()),
                Some(phase)
            );
            assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
            assert_eq!(
                document.boq.metadata.get("gaeb.support_policy"),
                Some(&serde_json::json!({
                    "status": SupportStatus::SupportedParseOnly,
                    "reason": format!(
                        "manifest fixture bvbs_xml33_bau_x{}: future-track fixture parsed without adapter/export promotion",
                        phase
                    ),
                }))
            );
            assert!(!document.capabilities.adapt_to_obra);
            assert!(!document.capabilities.export);
            assert!(!document.capabilities.roundtrip);
        }
    }

    #[test]
    fn support_policy_rejects_unsupported_ava_phase() {
        let document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x83/test.X83".to_owned()),
        )
        .expect("document should parse");

        assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
        assert!(!document.capabilities.adapt_to_obra);
    }

    #[test]
    fn xml33_bau_roundtrip_writer_preserves_semantic_item_fields() {
        let mut document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Bau Test</Name><BoQ><BoQBody><BoQCtgy ID="001" RNoPart="001"><Item ID="001.0010" RNoPart="10"><Qty>2.500</Qty><QU>m</QU><UP>3.200</UP><IT>8.00</IT><Description><CompleteText><DetailTxt><Text><p>Pipe trench &amp; bedding</p></Text></DetailTxt></CompleteText></Description></Item></BoQCtgy></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x84/test.X84".to_owned()),
        )
        .expect("Bau X84 should parse");
        document.support_status = SupportStatus::Supported;
        document.capabilities = SupportCapabilities::roundtrip_without_schema_validation();

        assert!(!document.capabilities.validate);
        let exported = write_string(&document).expect("Bau X84 export should be allowed");
        assert!(exported.contains(r#"RNoPart="10""#));
        assert!(!exported.contains(r#"RNoPart="Pipe trench""#));
        let reparsed = parse_str(
            &exported,
            Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x84/exported.X84".to_owned()),
        )
        .expect("exported Bau X84 should parse");

        let original = document.boq.nodes[0].children[0]
            .item
            .as_ref()
            .expect("original item");
        let roundtripped = reparsed.boq.nodes[0].children[0]
            .item
            .as_ref()
            .expect("roundtripped item");
        assert_eq!(roundtripped.quantity, original.quantity);
        assert_eq!(roundtripped.unit, original.unit);
        assert_eq!(roundtripped.unit_price, original.unit_price);
        assert_eq!(roundtripped.total_price, original.total_price);
        assert!(roundtripped.short_text.contains("Pipe trench"));
        assert_eq!(reparsed.support_status, SupportStatus::SupportedParseOnly);
    }

    #[test]
    fn parse_only_xml_export_is_rejected() {
        let document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/quantity_takeoff/x31/test.X31".to_owned()),
        )
        .expect("parse-only document should parse");

        let error = write_string(&document).expect_err("parse-only document must not export");
        assert_eq!(error.code, "gaeb_xml_export_not_supported");
    }

    #[test]
    fn bau_x84_baseline_merge_overlays_prices_without_losing_x83_text() {
        let baseline = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="001.0010" RNoPart="10"><Qty>1</Qty><QU>m</QU><Description><CompleteText><DetailTxt><Text><p>Baseline text</p></Text></DetailTxt></CompleteText></Description></Item><Item ID="001.0020" RNoPart="20"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x83/base.X83".to_owned()),
        )
        .expect("baseline should parse");
        let offer = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="001.0010" RNoPart="10"><Qty>1</Qty><QU>m</QU><UP>4.20</UP><IT>4.20</IT><Description>Offer text</Description></Item><Item ID="009.9999"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x84/offer.X84".to_owned()),
        )
        .expect("offer should parse");

        let merged = bau::merge_x84_offer_into_x83_baseline(&baseline, &offer);
        let item = merged.boq.nodes[0].item.as_ref().expect("merged item");
        assert_eq!(item.unit_price, Some(Decimal::new(420, 2)));
        assert_eq!(item.total_price, Some(Decimal::new(420, 2)));
        assert!(item.short_text.contains("Baseline text"));
        assert!(
            merged
                .findings
                .iter()
                .any(|finding| finding.code == "gaeb_xml_bau_x84_missing_ordinal")
        );
        assert!(
            merged
                .findings
                .iter()
                .any(|finding| finding.code == "gaeb_xml_bau_x84_extra_ordinal")
        );
    }

    #[test]
    fn bau_x84_baseline_merge_reports_duplicate_offer_ordinals() {
        let baseline = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="001.0010" RNoPart="10"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x83/base.X83".to_owned()),
        )
        .expect("baseline should parse");
        let offer = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="001.0010" RNoPart="10"><Qty>1</Qty><UP>1.00</UP></Item><Item ID="001.0010" RNoPart="10"><Qty>1</Qty><UP>2.00</UP></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x84/offer.X84".to_owned()),
        )
        .expect("offer should parse");

        let merged = bau::merge_x84_offer_into_x83_baseline(&baseline, &offer);
        assert!(
            merged
                .findings
                .iter()
                .any(|finding| finding.code == "gaeb_xml_bau_x84_duplicate_ordinal")
        );
    }

    #[test]
    fn invalid_utf8_file_reports_decode_error() {
        let path = std::env::temp_dir().join("boq-core-invalid-utf8.x81");
        std::fs::write(&path, [0xff, 0xfe, 0xfd]).expect("write temp invalid utf8");
        let error = parse_file(&path).expect_err("invalid utf8 should fail");
        let _ = std::fs::remove_file(path);
        assert_eq!(error.code, "xml_decode_failed");
    }

    #[test]
    fn plain_description_does_not_emit_rich_markup_loss() {
        let document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Description>Plain only</Description></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/plain.X81".to_owned()),
        )
        .expect("plain description should parse");
        assert!(
            !document
                .findings
                .iter()
                .any(|finding| finding.code == "gaeb_xml_description_plain_text_only")
        );
    }

    #[test]
    fn invalid_decimal_reports_parse_error() {
        let error = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Qty>not-a-number</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/bad.X81".to_owned()),
        )
        .expect_err("invalid decimal should fail");
        assert_eq!(error.code, "xml_decimal_parse_failed");
    }

    #[test]
    fn empty_decimal_fields_are_none_or_zero() {
        let document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Qty></Qty><UP></UP><IT></IT></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x86/empty.X86".to_owned()),
        )
        .expect("empty decimal fields should be tolerated");
        let item = document.boq.nodes[0].item.as_ref().expect("item payload");
        assert_eq!(item.quantity, Decimal::ZERO);
        assert_eq!(item.unit_price, None);
        assert_eq!(item.total_price, None);
    }

    #[test]
    fn unclosed_item_reports_item_error() {
        let error = parse_str(
            r#"<GAEB><Project><BoQ><BoQBody><Item ID="A"><Qty>1</Qty>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/unclosed.X81".to_owned()),
        )
        .expect_err("unclosed item should fail");
        assert!(matches!(
            error.code.as_str(),
            "xml_unclosed_item" | "xml_parse_failed"
        ));
    }

    #[test]
    fn unclosed_section_reports_section_error() {
        let error = parse_str(
            r#"<GAEB><Project><BoQ><BoQBody><BoQCtgy ID="S"><Item ID="A"/>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/unclosed-section.X81".to_owned()),
        )
        .expect_err("unclosed section should fail");

        assert_eq!(error.code, "xml_unclosed_section");
        assert_eq!(error.location.as_deref(), Some("S"));
    }

    #[test]
    fn unclosed_description_reports_description_error() {
        let error = parse_str(
            r#"<GAEB><Project><BoQ><BoQBody><Item ID="A"><Description><p>text"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/unclosed-description.X81".to_owned()),
        )
        .expect_err("unclosed description should fail");

        assert_eq!(error.code, "xml_unclosed_description");
    }

    #[test]
    fn malformed_top_level_xml_reports_parse_error() {
        let error = parse_str("<GAEB><", Some("bad-top-level.x81".to_owned()))
            .expect_err("top-level malformed XML should fail");
        assert_eq!(error.code, "xml_parse_failed");
    }

    #[test]
    fn malformed_item_body_reports_parse_error() {
        let error = parse_str(
            r#"<GAEB><Project><BoQ><BoQBody><Item ID="A"><Unknown><</Unknown></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/bad-item.X81".to_owned()),
        )
        .expect_err("malformed item body should fail");

        assert_eq!(error.code, "xml_parse_failed");
    }

    #[test]
    fn malformed_xml_reports_parse_error() {
        let error = parse_str("<GAEB><Version>3.3</GAEB>", Some("bad.x81".to_owned()))
            .expect_err("malformed XML should fail");
        assert!(matches!(
            error.code.as_str(),
            "xml_parse_failed" | "xml_text_read_failed"
        ));
    }

    #[test]
    fn missing_xml_file_reports_read_error() {
        let error = parse_file("tests/fixtures/synthetic/does-not-exist.x81")
            .expect_err("missing XML file should fail");
        assert_eq!(error.code, "xml_read_failed");
    }
}
