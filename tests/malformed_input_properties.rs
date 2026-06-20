#![allow(missing_docs, clippy::expect_used)]

use boq_core::{gaeb_xml, gaeb90};

#[derive(Debug, Clone, Copy)]
enum Entrypoint {
    Gaeb90,
    Xml,
}

#[derive(Debug, Clone, Copy)]
struct MalformedCase {
    label: &'static str,
    entrypoint: Entrypoint,
    input: &'static [u8],
}

const XML_UNKNOWN_EMPTY: &str = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Malformed Corpus</Name><BoQ><BoQBody><Item ID="001.0010" RNoPart="10"><Qty>1</Qty><QU>m</QU><UnknownPayload/><Description><CompleteText><DetailTxt><Text><p>Keep this text</p></Text></DetailTxt></CompleteText></Description></Item></BoQBody></BoQ></Project></GAEB>"#;
const XML_BLANK_ORDINAL: &str = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="" RNoPart="10"><Qty>1</Qty><QU>m</QU></Item></BoQBody></BoQ></Project></GAEB>"#;
const XML_TRUNCATED: &str = r#"<GAEB><Project><BoQ><BoQBody><Item ID="001"><Qty>1</Qty>"#;

const REGRESSION_CORPUS: &[MalformedCase] = &[
    MalformedCase {
        label: "gaeb90-blank-ordinal-short-line",
        entrypoint: Entrypoint::Gaeb90,
        input: b"21\r\n25short text\r\n",
    },
    MalformedCase {
        label: "gaeb90-long-line-windows-byte",
        entrypoint: Entrypoint::Gaeb90,
        input: b"210010020  10000m\x801234567890123456789012345678901234567890123456789012345678901234567890\r\n",
    },
    MalformedCase {
        label: "xml-unknown-empty-child",
        entrypoint: Entrypoint::Xml,
        input: XML_UNKNOWN_EMPTY.as_bytes(),
    },
    MalformedCase {
        label: "xml-blank-ordinal",
        entrypoint: Entrypoint::Xml,
        input: XML_BLANK_ORDINAL.as_bytes(),
    },
    MalformedCase {
        label: "xml-truncated-item",
        entrypoint: Entrypoint::Xml,
        input: XML_TRUNCATED.as_bytes(),
    },
];

#[test]
fn test_gaeb90_random_line_lengths_never_panic() {
    for line_length in [1_usize, 2, 3, 10, 79, 80, 81, 160] {
        let mut line = String::from("21");
        line.extend(std::iter::repeat_n('A', line_length.saturating_sub(2)));
        line.push('\n');

        let parsed = gaeb90::parse_bytes(
            line.as_bytes(),
            Some(format!("generated-{line_length}.d83")),
        );
        match parsed {
            Ok(document) => {
                assert_eq!(document.boq.nodes.len(), 1, "line length {line_length}");
                if line_length != 80 {
                    assert!(
                        document
                            .findings
                            .iter()
                            .any(|finding| finding.code == "gaeb90_line_length"),
                        "line length {line_length} must be a recoverable finding"
                    );
                }
            }
            Err(error) => {
                assert!(!error.code.is_empty(), "line length {line_length}");
                assert!(!error.message.is_empty(), "line length {line_length}");
            }
        }
    }
}

#[test]
fn test_xml_unknown_elements_never_silent_drop() {
    let document = gaeb_xml::parse_str(XML_UNKNOWN_EMPTY, Some("unknown.X81".to_owned()))
        .expect("unknown empty child should be recoverable");
    let item = document.boq.nodes[0]
        .item
        .as_ref()
        .expect("known Item payload must survive unknown children");

    assert_eq!(document.boq.nodes[0].ordinal, "001.0010");
    assert_eq!(item.unit, "m");
    assert_eq!(item.quantity.to_string(), "1");
    assert!(item.short_text.contains("Keep this text"));
    assert_eq!(
        document.boq.nodes[0]
            .metadata
            .get("gaeb.empty.UnknownPayload"),
        Some(&serde_json::json!(true)),
        "unknown empty payload must be represented as loss-aware metadata"
    );
}

#[test]
fn test_malformed_ordinal_numbers_emit_findings() {
    let gaeb90_doc = gaeb90::parse_bytes(b"21\r\n25short text\r\n", Some("blank.d83".to_owned()))
        .expect("blank GAEB 90 item ordinal should recover");
    assert_eq!(gaeb90_doc.boq.nodes[0].ordinal, "item_1");
    assert!(
        gaeb90_doc
            .findings
            .iter()
            .any(|finding| finding.code == "gaeb90_malformed_ordinal"),
        "blank GAEB 90 item ordinals must be recoverable findings"
    );

    let xml_doc = gaeb_xml::parse_str(XML_BLANK_ORDINAL, Some("blank.X81".to_owned()))
        .expect("blank XML item ordinal should recover");
    assert_eq!(xml_doc.boq.nodes[0].ordinal, "item_0");
    assert!(
        xml_doc
            .findings
            .iter()
            .any(|finding| finding.code == "gaeb_xml_malformed_ordinal"),
        "blank XML item ordinals must be recoverable findings"
    );
}

#[test]
fn test_fuzz_corpus_minimizes_regressions() {
    let mut labels = REGRESSION_CORPUS
        .iter()
        .map(|case| case.label)
        .collect::<Vec<_>>();
    labels.sort_unstable();
    labels.dedup();

    assert_eq!(
        labels.len(),
        REGRESSION_CORPUS.len(),
        "regression corpus labels must stay unique for minimization"
    );

    for case in REGRESSION_CORPUS {
        assert!(
            case.input.len() <= 512,
            "{} exceeds the bounded corpus size",
            case.label
        );
        match case.entrypoint {
            Entrypoint::Gaeb90 => {
                let _ = gaeb90::parse_bytes(case.input, Some(format!("{}.d83", case.label)));
            }
            Entrypoint::Xml => {
                let text = std::str::from_utf8(case.input).expect("XML corpus cases are UTF-8");
                let _ = gaeb_xml::parse_str(text, Some(format!("{}.X81", case.label)));
            }
        }
    }
}
