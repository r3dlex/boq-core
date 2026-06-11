//! GAEB XML writer/export boundary for supported in-crate roundtrip evidence.

use std::fmt::Write as _;

use crate::error::ValidationFinding;
use crate::model::{BoqNode, BoqNodeKind, GaebDocument, RichText};

/// Returns schema-validation readiness findings for exported GAEB XML.
///
/// The repository does not currently ship a local, checksummed GAEB XML schema
/// validator. Supported export paths therefore record an explicit readiness gap
/// instead of silently claiming external checker validation.
#[must_use]
pub fn schema_validation_findings(document: &GaebDocument) -> Vec<ValidationFinding> {
    if document.capabilities.export {
        vec![ValidationFinding::warning(
            "gaeb_xml_schema_validation_tooling_unavailable",
            "local checksummed GAEB XML schema validation is not configured; export is parser-roundtrip checked only",
        )]
    } else {
        Vec::new()
    }
}

/// Serializes a supported [`GaebDocument`] into a deterministic GAEB XML string.
///
/// The writer intentionally covers the loss-aware model fields currently parsed by
/// `boq-core`: hierarchy ordinals, quantities, units, prices, totals, and plain
/// long text. It refuses documents whose support capabilities do not explicitly
/// allow export, keeping export/roundtrip separate from parse-only support.
///
/// # Errors
///
/// Returns a validation finding when `document.capabilities.export` is false.
pub fn write_string(document: &GaebDocument) -> Result<String, ValidationFinding> {
    if !document.capabilities.export {
        return Err(ValidationFinding::warning(
            "gaeb_xml_export_not_supported",
            "document support capabilities do not allow GAEB XML export",
        ));
    }

    let phase_code = document
        .summary
        .phase
        .as_ref()
        .or(document.source.phase.as_ref())
        .map_or("81", |phase| phase.code.as_str());
    let version = document
        .summary
        .version
        .as_deref()
        .or(document.source.gaeb_version.as_deref())
        .unwrap_or("3.3");
    let title = document
        .summary
        .project_name
        .as_deref()
        .or(document.summary.title.as_deref())
        .unwrap_or(document.boq.title.as_str());

    let mut output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    let _ = writeln!(
        output,
        "<GAEB xmlns=\"http://www.gaeb.de/GAEB_DA_XML/DA{phase_code}/{version}\">"
    );
    let _ = writeln!(
        output,
        "  <GAEBInfo><Version>{}</Version></GAEBInfo>",
        escape_text(version)
    );
    output.push_str("  <PrjInfo>");
    let _ = write!(output, "<NamePrj>{}</NamePrj>", escape_text(title));
    output.push_str("<Cur>EUR</Cur><CurLbl>Euro</CurLbl>");
    output.push_str("</PrjInfo>\n");
    let _ = writeln!(output, "  <Award><DP>{phase_code}</DP><BoQ>");
    output.push_str("    <BoQBody>\n");
    for node in &document.boq.nodes {
        write_node(node, 6, &mut output)?;
    }
    output.push_str("    </BoQBody>\n");
    output.push_str("  </BoQ></Award>\n");
    output.push_str("</GAEB>\n");
    Ok(output)
}

fn write_node(node: &BoqNode, indent: usize, output: &mut String) -> Result<(), ValidationFinding> {
    match node.kind {
        BoqNodeKind::Chapter => write_chapter(node, indent, output),
        BoqNodeKind::Item | BoqNodeKind::Resource | BoqNodeKind::Assembly => {
            write_item(node, indent, output)
        }
    }
}

fn write_chapter(
    node: &BoqNode,
    indent: usize,
    output: &mut String,
) -> Result<(), ValidationFinding> {
    let pad = " ".repeat(indent);
    let _ = writeln!(
        output,
        "{pad}<BoQCtgy ID=\"{}\" RNoPart=\"{}\">",
        escape_attr(&node.ordinal),
        escape_attr(&node.title)
    );
    if !node.title.is_empty() {
        let _ = writeln!(
            output,
            "{pad}  <LblTx><p><span>{}</span></p></LblTx>",
            escape_text(&node.title)
        );
    }
    output.push_str(&pad);
    output.push_str("  <BoQBody><Itemlist>\n");
    for child in &node.children {
        write_node(child, indent + 4, output)?;
    }
    output.push_str(&pad);
    output.push_str("  </Itemlist></BoQBody>\n");
    let _ = writeln!(output, "{pad}</BoQCtgy>");
    Ok(())
}

fn write_item(node: &BoqNode, indent: usize, output: &mut String) -> Result<(), ValidationFinding> {
    let pad = " ".repeat(indent);
    let rno = node
        .metadata
        .get("gaeb.rno_part")
        .and_then(serde_json::Value::as_str)
        .unwrap_or(node.title.as_str());
    let _ = writeln!(
        output,
        "{pad}<Item ID=\"{}\" RNoPart=\"{}\">",
        escape_attr(&node.ordinal),
        escape_attr(rno)
    );
    if let Some(item) = &node.item {
        let _ = writeln!(output, "{pad}  <Qty>{}</Qty>", item.quantity.normalize());
        if !item.unit.is_empty() {
            let _ = writeln!(output, "{pad}  <QU>{}</QU>", escape_text(&item.unit));
        }
        if let Some(unit_price) = item.unit_price {
            let _ = writeln!(output, "{pad}  <UP>{}</UP>", unit_price.normalize());
        }
        if let Some(total_price) = item.total_price {
            let _ = writeln!(output, "{pad}  <IT>{}</IT>", total_price.normalize());
        }
        let description = item
            .long_text
            .as_ref()
            .map_or(Ok(item.short_text.as_str()), rich_text_plain)?;
        if !description.is_empty() {
            let _ = writeln!(
                output,
                "{pad}  <Description><CompleteText><DetailTxt><Text><p>{}</p></Text></DetailTxt></CompleteText></Description>",
                escape_text(description)
            );
        }
    }
    let _ = writeln!(output, "{pad}</Item>");
    Ok(())
}

fn rich_text_plain(rich_text: &RichText) -> Result<&str, ValidationFinding> {
    match rich_text {
        RichText::Plain(text) | RichText::XhtmlFragment(text) => Ok(text),
        RichText::Mixed(_) => Err(ValidationFinding::warning(
            "gaeb_xml_mixed_rich_text_export_not_supported",
            "mixed rich-text export is not supported without explicit loss handling",
        )),
    }
}

fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(value: &str) -> String {
    escape_text(value).replace('"', "&quot;")
}
