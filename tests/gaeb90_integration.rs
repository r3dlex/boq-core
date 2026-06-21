#![allow(missing_docs, clippy::expect_used)]

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::gaeb90;
use boq_core::support::SupportStatus;

#[test]
fn dangl_gaeb90_d83_parses_as_fixture_backed_adapter_compatible() {
    let document =
        gaeb90::parse_file("gaeb/developer_examples/dangl_ava_examples/gaeb90/d83/gaeb90.d83")
            .expect("Dangl GAEB 90 d83 should parse");

    assert_eq!(document.support_status, SupportStatus::Supported);
    assert!(document.capabilities.adapt_to_obra);
    assert_eq!(
        document
            .source
            .phase
            .as_ref()
            .map(|phase| phase.code.as_str()),
        Some("83")
    );
    assert!(document.boq.nodes.len() >= 3);
    assert!(
        document
            .boq
            .nodes
            .iter()
            .any(|node| node.title.contains("Excavation"))
    );

    let import = ObraImportDocument::try_from_gaeb(&document)
        .expect("fixture-backed GAEB 90 d83 should convert to Obra DTO");
    assert!(
        import
            .line_items
            .iter()
            .any(|item| item.description == "Excavation")
    );
}
