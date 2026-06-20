#![allow(missing_docs, clippy::expect_used)]

use std::fmt::Write as _;
use std::time::{Duration, Instant};

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::gaeb_xml;
use boq_core::model::GaebDocument;

const SMOKE_ITEMS: usize = 250;
const PARSE_SMOKE_BUDGET: Duration = Duration::from_secs(2);
const ADAPTER_SMOKE_BUDGET: Duration = Duration::from_secs(2);
const LARGE_X81_URI: &str = "gaeb/bvbs/gaeb_xml_3_3/ava/x81/synthetic-large.X81";

#[test]
fn test_large_boq_fixture_generator_is_deterministic() {
    let first = generate_large_gaeb_xml(SMOKE_ITEMS);
    let second = generate_large_gaeb_xml(SMOKE_ITEMS);

    assert_eq!(first, second);
    assert!(first.contains(r"<Project><Name>Synthetic Large BoQ</Name>"));
    assert_eq!(first.matches("<Item ").count(), SMOKE_ITEMS);
}

#[test]
fn bench_parse_large_gaeb_xml_under_budget() {
    let source = generate_large_gaeb_xml(SMOKE_ITEMS);
    let started = Instant::now();
    let document = parse_large_document(&source);
    let elapsed = started.elapsed();

    assert_eq!(document.boq.nodes[0].children.len(), SMOKE_ITEMS);
    assert!(
        elapsed <= PARSE_SMOKE_BUDGET,
        "large BoQ parse smoke took {elapsed:?}, budget {PARSE_SMOKE_BUDGET:?}"
    );
}

#[test]
fn bench_adapter_conversion_under_budget() {
    let source = generate_large_gaeb_xml(SMOKE_ITEMS);
    let document = parse_large_document(&source);

    let started = Instant::now();
    let adapted = ObraImportDocument::try_from_gaeb(&document)
        .expect("supported synthetic AVA X81 fixture should adapt to Obra DTO");
    let elapsed = started.elapsed();

    assert_eq!(adapted.line_items.len(), SMOKE_ITEMS);
    assert!(
        elapsed <= ADAPTER_SMOKE_BUDGET,
        "large BoQ adapter smoke took {elapsed:?}, budget {ADAPTER_SMOKE_BUDGET:?}"
    );
}

#[test]
fn test_benchmark_docs_include_machine_context() {
    let docs = include_str!("../docs/benchmarks/large-boq-baseline.md");
    for required in [
        "Machine context",
        "CPU",
        "Memory",
        "Rust toolchain",
        "Smoke budgets",
        "Synthetic large BoQ",
        "License-safe fixture path",
    ] {
        assert!(
            docs.contains(required),
            "missing benchmark doc anchor: {required}"
        );
    }
}

#[test]
fn benchmark_harness_includes_license_safe_fixture_path() {
    let fixture = include_str!("fixtures/synthetic/minimal_ava.x81");
    let document = gaeb_xml::parse_str(
        fixture,
        Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
    )
    .expect("license-safe synthetic fixture path should remain parseable");

    assert_eq!(document.boq.nodes.len(), 1);
}

fn parse_large_document(source: &str) -> GaebDocument {
    gaeb_xml::parse_str(source, Some(LARGE_X81_URI.to_owned()))
        .expect("synthetic large AVA X81 fixture should parse")
}

fn generate_large_gaeb_xml(items: usize) -> String {
    let mut source = String::from(
        r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Synthetic Large BoQ</Name><BoQ><BoQBody><BoQCtgy ID="001" RNoPart="001">"#,
    );
    for index in 1..=items {
        let ordinal = format!("001.{index:04}");
        let rno_part = format!("{index:04}");
        let qty = format!("{}.{:03}", 1 + (index % 17), index % 1_000);
        let unit = if index % 3 == 0 { "m3" } else { "m2" };
        write!(
            source,
            r#"<Item ID="{ordinal}" RNoPart="{rno_part}"><Qty>{qty}</Qty><QU>{unit}</QU><Description><CompleteText><DetailTxt><Text><p>Generated line item {index:04} for deterministic large BoQ smoke benchmark</p></Text></DetailTxt></CompleteText></Description></Item>"#
        )
        .expect("writing to String cannot fail");
    }
    source.push_str("</BoQCtgy></BoQBody></BoQ></Project></GAEB>");
    source
}
