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
