#![allow(missing_docs, clippy::expect_used)]

use std::path::Path;

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::gaeb_xml;

#[test]
fn ava_x81_imports_to_rich_model_and_obra_snapshot() {
    assert_real_bvbs_ava_fixture_parses("gaeb/bvbs/gaeb_xml_3_3/ava/x81");
}

#[test]
fn ava_x84_imports_priced_bid_snapshot() {
    assert_real_bvbs_ava_fixture_parses("gaeb/bvbs/gaeb_xml_3_3/ava/x84");
}

#[test]
fn ava_x86_imports_contract_award_snapshot() {
    assert_real_bvbs_ava_fixture_parses("gaeb/bvbs/gaeb_xml_3_3/ava/x86");
}

fn assert_real_bvbs_ava_fixture_parses(dir: &str) {
    let path = first_fixture_file(dir);
    let document = gaeb_xml::parse_file(&path).expect("BVBS AVA fixture should parse");
    assert_eq!(document.summary.version.as_deref(), Some("3.3"));
    assert!(
        !document.boq.nodes.is_empty(),
        "expected parsed nodes for {}",
        path.display()
    );
    assert!(
        document
            .boq
            .nodes
            .iter()
            .any(|node| !node.children.is_empty()),
        "expected preserved GAEB hierarchy for {}",
        path.display()
    );

    let adapted = ObraImportDocument::from_gaeb(&document)
        .expect("supported BVBS AVA fixture should adapt to Obra");
    assert!(adapted.wbs_nodes.len() >= document.boq.nodes.len());
    assert!(!adapted.boq.deterministic_key.is_empty());
}

fn first_fixture_file(dir: &str) -> std::path::PathBuf {
    std::fs::read_dir(Path::new(dir))
        .expect("fixture dir exists")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| path.is_file())
        .expect("fixture file exists")
}

#[derive(serde::Serialize)]
struct AvaGoldenReport {
    schema_version: u8,
    fixture_id: &'static str,
    source_path: String,
    source_checksum_sha256: String,
    parser_version: &'static str,
    support_status: String,
    capabilities: serde_json::Value,
    summary: serde_json::Value,
    hierarchy: HierarchySummary,
    item_samples: Vec<ItemSummary>,
    findings: Vec<serde_json::Value>,
    certification_claim: bool,
}

#[derive(serde::Serialize)]
struct HierarchySummary {
    root_count: usize,
    total_node_count: usize,
    total_item_count: usize,
    root_ordinals: Vec<String>,
}

#[derive(serde::Serialize)]
struct ItemSummary {
    ordinal: String,
    title: String,
    quantity: String,
    unit: String,
    unit_price: Option<String>,
    total_price: Option<String>,
    long_text_kind: &'static str,
}

#[test]
fn test_bvbs_ava_x81_golden_report_matches() {
    assert_bvbs_ava_golden_report_matches(
        "bvbs_xml33_ava_x81",
        "gaeb/bvbs/gaeb_xml_3_3/ava/x81",
        "gaeb/golden/bvbs_ava/x81-report.json",
    );
}

#[test]
fn test_bvbs_ava_x84_golden_report_matches() {
    assert_bvbs_ava_golden_report_matches(
        "bvbs_xml33_ava_x84",
        "gaeb/bvbs/gaeb_xml_3_3/ava/x84",
        "gaeb/golden/bvbs_ava/x84-report.json",
    );
}

#[test]
fn test_bvbs_ava_x86_golden_report_matches() {
    assert_bvbs_ava_golden_report_matches(
        "bvbs_xml33_ava_x86",
        "gaeb/bvbs/gaeb_xml_3_3/ava/x86",
        "gaeb/golden/bvbs_ava/x86-report.json",
    );
}

#[test]
fn test_golden_reports_capture_support_status_and_findings() {
    for report_path in [
        "gaeb/golden/bvbs_ava/x81-report.json",
        "gaeb/golden/bvbs_ava/x84-report.json",
        "gaeb/golden/bvbs_ava/x86-report.json",
    ] {
        let report = std::fs::read_to_string(report_path).expect("golden report exists");
        let value: serde_json::Value = serde_json::from_str(&report).expect("golden report parses");
        assert_eq!(value["schema_version"], 1, "{report_path}");
        assert_eq!(value["support_status"], "supported", "{report_path}");
        assert_eq!(value["certification_claim"], false, "{report_path}");
        assert!(
            value["source_checksum_sha256"]
                .as_str()
                .is_some_and(|checksum| checksum.len() == 64)
        );
        assert!(value["findings"].as_array().is_some(), "{report_path}");
        assert!(
            value["hierarchy"]["total_item_count"]
                .as_u64()
                .unwrap_or_default()
                > 0,
            "{report_path}"
        );
    }
}

fn assert_bvbs_ava_golden_report_matches(fixture_id: &'static str, dir: &str, report_path: &str) {
    let report = build_bvbs_ava_golden_report(fixture_id, dir);
    let actual = serde_json::to_string_pretty(&report).expect("golden report serializes") + "\n";

    if std::env::var_os("UPDATE_BVBS_AVA_GOLDEN").is_some() {
        std::fs::create_dir_all("gaeb/golden/bvbs_ava").expect("golden dir created");
        std::fs::write(report_path, actual).expect("golden report written");
        return;
    }

    let expected = std::fs::read_to_string(report_path).expect("golden report exists");
    assert_eq!(
        actual, expected,
        "golden drift in {report_path}; review parser changes before refreshing with UPDATE_BVBS_AVA_GOLDEN=1 cargo test --test bvbs_ava_integration"
    );
}

fn build_bvbs_ava_golden_report(fixture_id: &'static str, dir: &str) -> AvaGoldenReport {
    let path = first_fixture_file(dir);
    let document = gaeb_xml::parse_file(&path).expect("BVBS AVA fixture should parse");
    let mut item_samples = Vec::new();
    collect_item_samples(&document.boq.nodes, &mut item_samples);
    item_samples.truncate(8);

    AvaGoldenReport {
        schema_version: 1,
        fixture_id,
        source_path: path.to_string_lossy().replace('\\', "/"),
        source_checksum_sha256: document
            .source
            .checksum
            .clone()
            .expect("parsed fixture has checksum"),
        parser_version: env!("CARGO_PKG_VERSION"),
        support_status: serde_json::to_value(document.support_status)
            .expect("support status serializes")
            .as_str()
            .expect("support status is string")
            .to_owned(),
        capabilities: serde_json::to_value(document.capabilities).expect("capabilities serialize"),
        summary: serde_json::json!({
            "format": document.summary.format,
            "version": document.summary.version,
            "phase": document.summary.phase,
            "title": document.summary.title,
            "project_name": document.summary.project_name,
            "currency": document.boq.currency,
        }),
        hierarchy: HierarchySummary {
            root_count: document.boq.nodes.len(),
            total_node_count: count_nodes(&document.boq.nodes),
            total_item_count: count_items(&document.boq.nodes),
            root_ordinals: document
                .boq
                .nodes
                .iter()
                .map(|node| node.ordinal.clone())
                .collect(),
        },
        item_samples,
        findings: document
            .findings
            .iter()
            .map(|finding| serde_json::to_value(finding).expect("finding serializes"))
            .collect(),
        certification_claim: false,
    }
}

fn count_nodes(nodes: &[boq_core::model::BoqNode]) -> usize {
    nodes
        .iter()
        .map(|node| 1 + count_nodes(&node.children))
        .sum()
}

fn count_items(nodes: &[boq_core::model::BoqNode]) -> usize {
    nodes
        .iter()
        .map(|node| usize::from(node.item.is_some()) + count_items(&node.children))
        .sum()
}

fn collect_item_samples(nodes: &[boq_core::model::BoqNode], items: &mut Vec<ItemSummary>) {
    for node in nodes {
        if let Some(item) = &node.item {
            items.push(ItemSummary {
                ordinal: node.ordinal.clone(),
                title: node.title.clone(),
                quantity: item.quantity.to_string(),
                unit: item.unit.clone(),
                unit_price: item.unit_price.map(|price| price.to_string()),
                total_price: item.total_price.map(|price| price.to_string()),
                long_text_kind: match item.long_text {
                    Some(boq_core::model::RichText::Plain(_)) => "plain",
                    Some(boq_core::model::RichText::XhtmlFragment(_)) => "xhtml_fragment",
                    Some(boq_core::model::RichText::Mixed(_)) => "mixed",
                    None => "none",
                },
            });
        }
        collect_item_samples(&node.children, items);
    }
}
