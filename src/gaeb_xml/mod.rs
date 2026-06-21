//! GAEB DA XML parser foundation.
//!
//! This module is the public XML parse/export boundary. Use [`parse_str`] or
//! [`parse_file`] to create a loss-aware [`crate::model::GaebDocument`], then
//! inspect [`crate::model::GaebDocument::support_status`] and
//! [`crate::model::GaebDocument::capabilities`] before validation, adapter, or
//! export work.
//!
//! ```
//! let source = include_str!("../../tests/fixtures/synthetic/minimal_ava.x81");
//! let document = boq_core::gaeb_xml::parse_str(
//!     source,
//!     Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
//! )?;
//!
//! assert_eq!(document.summary.version.as_deref(), Some("3.3"));
//! assert!(document.capabilities.adapt_to_obra);
//! # Ok::<(), boq_core::error::ParseError>(())
//! ```

use std::collections::BTreeMap;
use std::path::Path;

use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::{Reader, Writer};
use rust_decimal::Decimal;

use crate::VERSION;
use crate::checksum::sha256_hex;
use crate::error::{ParseError, ValidationFinding};
use crate::format::detect_path;
use crate::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, GaebDocument, GaebDocumentSummary, GaebFormat, GaebPhase,
    Metadata, RichText, RichTextFragment, SourceProvenance,
};
use crate::support::SupportQuery;

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
    preserve_rich_descriptions: bool,
}

struct ParsedDescription {
    plain_text: String,
    rich_text: Option<RichText>,
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
            preserve_rich_descriptions: false,
        }
    }

    fn parse(
        &mut self,
        source_uri: Option<String>,
        checksum: Option<String>,
    ) -> Result<GaebDocument, ParseError> {
        self.preserve_rich_descriptions = source_uri.as_deref().is_some_and(|uri| {
            uri.contains("/ava/")
                || uri.contains("\\ava\\")
                || uri.contains("specification_authoring")
                || uri.contains("texterstellung")
                || uri.contains("text_x81")
                || uri.contains("text_x82")
        });

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
        if let Some(version) = self.version.clone() {
            self.record_version_compatibility_finding(&version);
        }
        let support = crate::support::default_policy().decide(SupportQuery {
            format: GaebFormat::GaebXml,
            version: self.version.as_deref(),
            phase: self.phase.as_ref(),
            source_uri: source_uri.as_deref(),
        });
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
            if legacy_xml_version_from_namespace(namespace).is_some() {
                metadata.insert(
                    "gaeb.xml_version_inferred_from_namespace".to_owned(),
                    serde_json::json!(true),
                );
            }
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
                let namespace = String::from_utf8_lossy(attr.value.as_ref()).to_string();
                if self.version.is_none() {
                    self.version = legacy_xml_version_from_namespace(&namespace).map(str::to_owned);
                }
                self.namespace = Some(namespace);
            }
        }
    }

    fn record_version_compatibility_finding(&mut self, version: &str) {
        if matches!(version, "3.1" | "3.2") {
            self.findings.push(ValidationFinding::warning(
                "gaeb_xml_legacy_version_compatibility",
                format!(
                    "GAEB XML {version} compatibility is detected but remains gated by fixture support status; data is not silently coerced to GAEB XML 3.3"
                ),
            ));
        } else if version == "3.4" {
            self.findings.push(ValidationFinding::warning(
                "gaeb_xml_beta_version_reference_only",
                "GAEB XML 3.4 beta is detected as reference-only impact tracking, not production parser support",
            ));
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
        let raw_ordinal = attr_value(start, b"ID");
        let ordinal = usable_item_ordinal(raw_ordinal.as_deref(), sort_order);
        if raw_ordinal.as_deref().is_some_and(is_malformed_xml_ordinal) {
            self.findings.push(
                ValidationFinding::warning(
                    "gaeb_xml_malformed_ordinal",
                    "item ordinal attribute was blank or contained whitespace; a stable fallback ordinal was used",
                )
                .at(ordinal.clone()),
            );
        }
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
            self.read_item_body(&ordinal, &mut item, &mut title, &mut metadata)?;
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
        ordinal: &str,
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
                            let description = self.read_description(ordinal)?;
                            if self.is_phase("84") && !description.plain_text.is_empty() {
                                self.findings.push(
                                    ValidationFinding::warning(
                                        "gaeb_xml_bau_x84_tender_description_not_authoritative",
                                        "X84 description text is bid payload text; X83 baseline descriptions remain authoritative for tender wording",
                                    )
                                    .at(format!("{ordinal}/Description")),
                                );
                            }
                            if !description.plain_text.is_empty() {
                                title.clone_from(&description.plain_text);
                                item.short_text.clone_from(&description.plain_text);
                            }
                            if description.rich_text.is_some() {
                                item.long_text = description.rich_text;
                            }
                        }
                        "BidderRemark" | "BieterBemerkung" | "Bieterangabe" | "Remark" => {
                            self.capture_bidder_remark(&local, item, metadata)?;
                        }
                        "LumpSumItem" => {
                            let value = self.read_text_for(owned.name())?.unwrap_or_default();
                            metadata
                                .insert("gaeb.lump_sum_item".to_owned(), serde_json::json!(value));
                        }
                        _ => {
                            let value = self.read_unsupported_item_field(&local)?;
                            let metadata_value = value.filter(|text| !text.is_empty()).map_or_else(
                                || serde_json::json!(true),
                                |text| serde_json::json!(text),
                            );
                            metadata.insert(format!("gaeb.unsupported.{local}"), metadata_value);
                            self.findings.push(
                                ValidationFinding::warning(
                                    "gaeb_xml_unsupported_item_field",
                                    format!(
                                        "unsupported GAEB XML item field {local} was preserved as metadata"
                                    ),
                                )
                                .at(format!("{ordinal}/{local}")),
                            );
                        }
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

    fn capture_bidder_remark(
        &mut self,
        field: &str,
        item: &mut BoqItem,
        metadata: &mut Metadata,
    ) -> Result<(), ParseError> {
        let value = self.read_unsupported_item_field(field)?.unwrap_or_default();
        if !value.is_empty() {
            item.notes = Some(value.clone());
            item.metadata.insert(
                "gaeb.bau_x84.bidder_remark".to_owned(),
                serde_json::json!(value),
            );
            metadata.insert(
                "gaeb.bau_x84.bidder_remark".to_owned(),
                serde_json::json!(true),
            );
        }
        Ok(())
    }

    fn is_phase(&self, code: &str) -> bool {
        self.phase.as_ref().is_some_and(|phase| phase.code == code)
    }

    fn read_unsupported_item_field(&mut self, field: &str) -> Result<Option<String>, ParseError> {
        let mut depth = 1_usize;
        let mut parts = Vec::new();
        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(_)) => depth = depth.saturating_add(1),
                Ok(Event::Text(text)) => {
                    let decoded = text.decode().map_err(|error| ParseError {
                        code: "xml_text_decode_failed".to_owned(),
                        message: error.to_string(),
                        location: Some(field.to_owned()),
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
                        code: "xml_unclosed_unsupported_item_field".to_owned(),
                        message: format!(
                            "unsupported item field {field} ended before its closing tag"
                        ),
                        location: Some(field.to_owned()),
                    });
                }
                Err(error) => {
                    return Err(ParseError {
                        code: "xml_parse_failed".to_owned(),
                        message: error.to_string(),
                        location: Some(field.to_owned()),
                    });
                }
                _ => {}
            }
            self.buffer.clear();
        }
        let text = parts.join(" ");
        Ok((!text.is_empty()).then_some(text))
    }

    fn read_description(&mut self, ordinal: &str) -> Result<ParsedDescription, ParseError> {
        if self.preserve_rich_descriptions {
            self.read_rich_description(ordinal)
        } else {
            let plain_text = self.read_description_text()?;
            let rich_text = (!plain_text.is_empty()).then(|| RichText::Plain(plain_text.clone()));
            Ok(ParsedDescription {
                plain_text,
                rich_text,
            })
        }
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

    #[allow(clippy::too_many_lines)]
    fn read_rich_description(&mut self, ordinal: &str) -> Result<ParsedDescription, ParseError> {
        let mut depth = 1_usize;
        let mut plain_parts = Vec::new();
        let mut inner_writer = Writer::new(Vec::new());
        let mut table_writer: Option<Writer<Vec<u8>>> = None;
        let mut table_depth = 0_usize;
        let mut table_fragments = Vec::new();
        let mut saw_markup = false;
        let mut saw_layout_sensitive_markup = false;
        let mut saw_text_complement = false;

        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(start)) => {
                    let owned = start.into_owned();
                    let local = local_name(owned.name());
                    saw_markup = true;
                    if local == "TextComplement" {
                        saw_text_complement = true;
                    }
                    if has_layout_sensitive_attrs(&owned) || is_layout_sensitive_tag(&local) {
                        saw_layout_sensitive_markup = true;
                    }

                    write_xml_event(
                        &mut inner_writer,
                        Event::Start(owned.clone()),
                        "Description",
                    )?;
                    if let Some(writer) = table_writer.as_mut() {
                        table_depth = table_depth.saturating_add(1);
                        write_xml_event(writer, Event::Start(owned), "table")?;
                    } else if local == "table" {
                        let mut writer = Writer::new(Vec::new());
                        table_depth = 1;
                        write_xml_event(&mut writer, Event::Start(owned), "table")?;
                        table_writer = Some(writer);
                    }
                    depth = depth.saturating_add(1);
                }
                Ok(Event::Empty(start)) => {
                    let owned = start.into_owned();
                    let local = local_name(owned.name());
                    saw_markup = true;
                    if local == "TextComplement" {
                        saw_text_complement = true;
                    }
                    if has_layout_sensitive_attrs(&owned) || is_layout_sensitive_tag(&local) {
                        saw_layout_sensitive_markup = true;
                    }

                    write_xml_event(
                        &mut inner_writer,
                        Event::Empty(owned.clone()),
                        "Description",
                    )?;
                    if let Some(writer) = table_writer.as_mut() {
                        write_xml_event(writer, Event::Empty(owned), "table")?;
                    } else if local == "table" {
                        let mut writer = Writer::new(Vec::new());
                        write_xml_event(&mut writer, Event::Empty(owned), "table")?;
                        table_fragments.push(writer_to_string(writer, "table")?);
                    }
                }
                Ok(Event::Text(text)) => {
                    let owned = text.into_owned();
                    let decoded = owned.decode().map_err(|error| ParseError {
                        code: "xml_text_decode_failed".to_owned(),
                        message: error.to_string(),
                        location: Some("Description".to_owned()),
                    })?;
                    let trimmed = decoded.trim();
                    if !trimmed.is_empty() {
                        plain_parts.push(trimmed.to_owned());
                    }
                    write_xml_event(&mut inner_writer, Event::Text(owned.clone()), "Description")?;
                    if let Some(writer) = table_writer.as_mut() {
                        write_xml_event(writer, Event::Text(owned), "table")?;
                    }
                }
                Ok(Event::CData(cdata)) => {
                    let owned = cdata.into_owned();
                    saw_markup = true;
                    write_xml_event(
                        &mut inner_writer,
                        Event::CData(owned.clone()),
                        "Description",
                    )?;
                    if let Some(writer) = table_writer.as_mut() {
                        write_xml_event(writer, Event::CData(owned), "table")?;
                    }
                }
                Ok(Event::End(end)) => {
                    let owned = end.into_owned();
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        break;
                    }
                    write_xml_event(&mut inner_writer, Event::End(owned.clone()), "Description")?;
                    if let Some(writer) = table_writer.as_mut() {
                        write_xml_event(writer, Event::End(owned), "table")?;
                        table_depth = table_depth.saturating_sub(1);
                        if table_depth == 0 {
                            let Some(writer) = table_writer.take() else {
                                return Err(ParseError {
                                    code: "xml_fragment_capture_failed".to_owned(),
                                    message: "table capture ended without an active writer"
                                        .to_owned(),
                                    location: Some("table".to_owned()),
                                });
                            };
                            table_fragments.push(writer_to_string(writer, "table")?);
                        }
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

        let plain_text = plain_parts.join("\n");
        let inner_xml = writer_to_string(inner_writer, "Description")?;
        let rich_text = if plain_text.is_empty() && inner_xml.is_empty() {
            None
        } else if table_fragments.is_empty() && saw_markup {
            Some(RichText::XhtmlFragment(inner_xml))
        } else if table_fragments.is_empty() {
            Some(RichText::Plain(plain_text.clone()))
        } else {
            let mut fragments = Vec::with_capacity(table_fragments.len() + 2);
            if !inner_xml.is_empty() {
                fragments.push(RichTextFragment::Unknown(inner_xml));
            }
            if !plain_text.is_empty() {
                fragments.push(RichTextFragment::Text(plain_text.clone()));
            }
            fragments.extend(table_fragments.into_iter().map(RichTextFragment::Table));
            Some(RichText::Mixed(fragments))
        };

        if saw_layout_sensitive_markup {
            self.findings.push(
                ValidationFinding::warning(
                    "gaeb_xml_texterstellung_layout_preserved_not_rendered",
                    "Texterstellung layout/style markup was preserved in rich text but is not rendered or certified",
                )
                .at(format!("{ordinal}/Description")),
            );
        }
        if saw_text_complement {
            self.findings.push(
                ValidationFinding::warning(
                    "gaeb_xml_texterstellung_text_complement_preserved_as_markup",
                    "Texterstellung text-complement markup was preserved as rich XML; no completion semantics are claimed",
                )
                .at(format!("{ordinal}/Description/TextComplement")),
            );
        }

        Ok(ParsedDescription {
            plain_text,
            rich_text,
        })
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

fn legacy_xml_version_from_namespace(namespace: &str) -> Option<&'static str> {
    let normalized = namespace.replace('_', ".");
    if normalized.contains("3.1") {
        Some("3.1")
    } else if normalized.contains("3.2") {
        Some("3.2")
    } else if normalized.contains("3.3") {
        Some("3.3")
    } else if normalized.contains("3.4") {
        Some("3.4")
    } else {
        None
    }
}

fn local_name(name: QName<'_>) -> String {
    let raw = name.as_ref();
    let after_prefix = raw.rsplit(|byte| *byte == b':').next().unwrap_or(raw);
    String::from_utf8_lossy(after_prefix).to_string()
}

fn usable_item_ordinal(raw: Option<&str>, sort_order: i32) -> String {
    raw.map(str::trim)
        .filter(|value| !value.is_empty() && !value.chars().any(char::is_whitespace))
        .map_or_else(|| format!("item_{sort_order}"), ToOwned::to_owned)
}

fn is_malformed_xml_ordinal(raw: &str) -> bool {
    let trimmed = raw.trim();
    trimmed.is_empty() || trimmed.chars().any(char::is_whitespace)
}

fn write_xml_event(
    writer: &mut Writer<Vec<u8>>,
    event: Event<'_>,
    location: &str,
) -> Result<(), ParseError> {
    writer.write_event(event).map_err(|error| ParseError {
        code: "xml_fragment_capture_failed".to_owned(),
        message: error.to_string(),
        location: Some(location.to_owned()),
    })
}

fn writer_to_string(writer: Writer<Vec<u8>>, location: &str) -> Result<String, ParseError> {
    String::from_utf8(writer.into_inner()).map_err(|error| ParseError {
        code: "xml_fragment_decode_failed".to_owned(),
        message: error.to_string(),
        location: Some(location.to_owned()),
    })
}

fn has_layout_sensitive_attrs(start: &BytesStart<'_>) -> bool {
    start.attributes().flatten().any(|attr| {
        matches!(
            attr.key.as_ref(),
            b"style" | b"valign" | b"align" | b"cellpadding" | b"cellspacing" | b"width"
        )
    })
}

fn is_layout_sensitive_tag(local: &str) -> bool {
    matches!(
        local,
        "br" | "TextComplement" | "ComplCaption" | "ComplBody" | "ComplTail"
    )
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
    use crate::support::{SupportCapabilities, SupportStatus};
    use quick_xml::events::BytesText;
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
        assert!(matches!(
            item.long_text.as_ref(),
            Some(RichText::XhtmlFragment(fragment))
                if fragment.contains("<p>Pipe trench</p>")
        ));
        assert!(
            !document
                .findings
                .iter()
                .any(|finding| finding.code == "gaeb_xml_description_plain_text_only")
        );
    }

    #[test]
    fn embedded_support_policy_promotes_ava_and_keeps_bau_parse_only() {
        let ava = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/probe.X81".to_owned()),
        )
        .expect("AVA probe should parse");
        assert_eq!(ava.support_status, SupportStatus::Supported);

        let bau = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x83/probe.X83".to_owned()),
        )
        .expect("Bau probe should parse");
        assert_eq!(bau.support_status, SupportStatus::SupportedParseOnly);
        assert_eq!(
            bau.boq.metadata.get("gaeb.support_policy"),
            Some(&serde_json::json!({
                "status": SupportStatus::SupportedParseOnly,
                "reason":
                    "manifest fixture bvbs_xml33_bau_x83: supported parse-only fixture",
            }))
        );
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
    fn support_policy_promotes_xml33_bau_x83_and_x84_parse_only() {
        for (path, phase, reason) in [
            (
                "gaeb/bvbs/gaeb_xml_3_3/construction_execution/x83/test.X83",
                "83",
                "manifest fixture bvbs_xml33_bau_x83: supported parse-only fixture".to_owned(),
            ),
            (
                "gaeb/bvbs/gaeb_xml_3_3/construction_execution/x84/test.X84",
                "84",
                "manifest fixture bvbs_xml33_bau_x84: supported parse-only fixture".to_owned(),
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
                    "reason": reason,
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

    #[test]
    fn xml_fragment_helpers_preserve_text_and_layout_flags() {
        let mut writer = Writer::new(Vec::new());
        write_xml_event(
            &mut writer,
            Event::Text(BytesText::new("fragment text")),
            "helper-test",
        )
        .expect("fragment text writes");
        assert_eq!(
            writer_to_string(writer, "helper-test").expect("fragment decodes"),
            "fragment text"
        );

        let mut styled = BytesStart::new("span");
        styled.push_attribute(("style", "font-weight:bold"));
        assert!(has_layout_sensitive_attrs(&styled));
        assert!(is_layout_sensitive_tag("TextComplement"));
        assert!(!is_layout_sensitive_tag("p"));
    }

    #[test]
    fn make_section_uses_stable_fallback_without_attributes() {
        let start = BytesStart::new("BoQCtgy");
        let section = XmlParser::make_section(&start, 7);
        assert_eq!(section.ordinal, "section_7");
        assert_eq!(section.title, "section_7");
        assert_eq!(section.kind, BoqNodeKind::Chapter);
    }

    #[test]
    fn schema_validation_findings_record_export_tooling_gap() {
        let mut document = parse_str(
            r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="A"/></BoQBody></BoQ></Project></GAEB>"#,
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/schema-gap.X81".to_owned()),
        )
        .expect("document parses");
        document.capabilities = SupportCapabilities::roundtrip_without_schema_validation();

        let findings = schema_validation_findings(&document);
        assert_eq!(findings.len(), 1);
        assert_eq!(
            findings[0].code,
            "gaeb_xml_schema_validation_tooling_unavailable"
        );
    }
}
