#![allow(missing_docs, clippy::expect_used)]

use boq_core::gaeb_xml;
use boq_core::model::{RichText, RichTextFragment};
use boq_core::support::SupportStatus;
use rust_decimal::Decimal;

const TEXT_X81_RICH: &str = r#"
<GAEB>
  <GAEBInfo><Version>3.3</Version></GAEBInfo>
  <Project>
    <Name>Texterstellung X81</Name>
    <BoQ>
      <BoQBody>
        <Item ID="001.0010" RNoPart="10">
          <Qty>1.000</Qty>
          <QU>St</QU>
          <Description>
            <CompleteText>
              <DetailTxt>
                <Text>
                  <p style="text-align:left;"><span style="font-weight:bold;">Formatierter Text</span></p>
                </Text>
              </DetailTxt>
            </CompleteText>
          </Description>
        </Item>
      </BoQBody>
    </BoQ>
  </Project>
</GAEB>
"#;

const TEXT_X81_TABLE: &str = r#"
<GAEB>
  <GAEBInfo><Version>3.3</Version></GAEBInfo>
  <Project>
    <Name>Texterstellung X81 Tables</Name>
    <BoQ>
      <BoQBody>
        <Item ID="001.0020" RNoPart="20">
          <Qty>1.000</Qty>
          <QU>St</QU>
          <Description>
            <CompleteText>
              <DetailTxt>
                <Text>
                  <p><span>Tabelle mit 3 Spalten</span></p>
                  <table cellpadding="6" style="border-left:1px solid;">
                    <tr><td style="text-align:right;"><p><span>Spalte 1</span></p></td><td><p><span>Spalte 2</span></p></td></tr>
                    <tr><td><p><span>rechts</span></p></td><td><p><span>links</span></p></td></tr>
                  </table>
                </Text>
              </DetailTxt>
            </CompleteText>
          </Description>
        </Item>
      </BoQBody>
    </BoQ>
  </Project>
</GAEB>
"#;

const TEXT_X82_ESTIMATE: &str = r#"
<GAEB>
  <GAEBInfo><Version>3.3</Version></GAEBInfo>
  <Project>
    <Name>Texterstellung X82</Name>
    <BoQ>
      <BoQBody>
        <Item ID="001.0030" RNoPart="30">
          <Qty>2.000</Qty>
          <QU>m2</QU>
          <UP>12.340</UP>
          <IT>24.68</IT>
          <Description>
            <CompleteText>
              <DetailTxt>
                <Text>
                  <p><span>Kostenansatz</span></p>
                  <TextComplement Kind="Owner" MarkLbl="1">
                    <ComplCaption><span>Breite</span></ComplCaption>
                    <ComplBody><span>80</span></ComplBody>
                    <ComplTail><span>cm</span></ComplTail>
                  </TextComplement>
                </Text>
              </DetailTxt>
            </CompleteText>
          </Description>
        </Item>
      </BoQBody>
    </BoQ>
  </Project>
</GAEB>
"#;

#[test]
fn test_text_x81_rich_text_blocks_preserved() {
    let document = gaeb_xml::parse_str(
        TEXT_X81_RICH,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x81/rich.X81".to_owned()),
    )
    .expect("Texterstellung X81 rich text should parse");

    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    assert!(document.capabilities.parse);
    assert!(!document.capabilities.adapt_to_obra);

    let item = document.boq.nodes[0].item.as_ref().expect("item payload");
    assert_eq!(item.short_text, "Formatierter Text");
    assert!(matches!(
        item.long_text.as_ref(),
        Some(RichText::XhtmlFragment(fragment))
            if fragment.contains("<p") && fragment.contains("font-weight:bold")
    ));
    assert!(document.findings.iter().any(|finding| {
        finding.code == "gaeb_xml_texterstellung_layout_preserved_not_rendered"
    }));
}

#[test]
fn test_text_tables_normalize_to_document_blocks() {
    let document = gaeb_xml::parse_str(
        TEXT_X81_TABLE,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x81/table.X81".to_owned()),
    )
    .expect("Texterstellung table should parse");

    let item = document.boq.nodes[0].item.as_ref().expect("item payload");
    let Some(rich_text) = item.long_text.as_ref() else {
        assert!(item.long_text.is_some(), "expected item rich text");
        return;
    };
    let RichText::Mixed(fragments) = rich_text else {
        assert!(
            matches!(rich_text, RichText::Mixed(_)),
            "expected mixed rich text with table fragment"
        );
        return;
    };

    assert!(fragments.iter().any(|fragment| matches!(
        fragment,
        RichTextFragment::Text(text)
            if text.contains("Tabelle mit 3 Spalten") && text.contains("Spalte 1")
    )));
    assert!(fragments.iter().any(|fragment| matches!(
        fragment,
        RichTextFragment::Table(markup)
            if markup.contains("<table") && markup.contains("Spalte 2")
    )));
}

#[test]
fn test_text_x82_cost_estimate_metadata_preserved() {
    let document = gaeb_xml::parse_str(
        TEXT_X82_ESTIMATE,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x82/estimate.X82".to_owned()),
    )
    .expect("Texterstellung X82 should parse");

    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    let item = document.boq.nodes[0].item.as_ref().expect("item payload");
    assert_eq!(item.quantity, Decimal::new(2000, 3));
    assert_eq!(item.unit, "m2");
    assert_eq!(item.unit_price, Some(Decimal::new(12340, 3)));
    assert_eq!(item.total_price, Some(Decimal::new(2468, 2)));
    assert!(matches!(
        item.long_text.as_ref(),
        Some(RichText::XhtmlFragment(fragment))
            if fragment.contains("TextComplement") && fragment.contains("Kostenansatz")
    ));
}

#[test]
fn test_text_unsupported_layout_emits_findings() {
    let document = gaeb_xml::parse_str(
        TEXT_X82_ESTIMATE,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x82/layout.X82".to_owned()),
    )
    .expect("Texterstellung X82 should parse with findings");

    let codes = document
        .findings
        .iter()
        .map(|finding| finding.code.as_str())
        .collect::<Vec<_>>();
    assert!(codes.contains(&"gaeb_xml_texterstellung_layout_preserved_not_rendered"));
    assert!(codes.contains(&"gaeb_xml_texterstellung_text_complement_preserved_as_markup"));
}

#[test]
fn test_text_plain_description_without_markup_stays_plain() {
    let document = gaeb_xml::parse_str(
        r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="P" RNoPart="Plain"><Description>Nur Text</Description></Item></BoQBody></BoQ></Project></GAEB>"#,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x81/plain.X81".to_owned()),
    )
    .expect("plain Texterstellung description should parse");

    let item = document.boq.nodes[0].item.as_ref().expect("item payload");
    assert_eq!(item.short_text, "Nur Text");
    assert_eq!(
        item.long_text.as_ref(),
        Some(&RichText::Plain("Nur Text".to_owned()))
    );
}

#[test]
fn test_text_empty_description_keeps_attribute_title() {
    let document = gaeb_xml::parse_str(
        r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="E" RNoPart="Empty title"><Description></Description></Item></BoQBody></BoQ></Project></GAEB>"#,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x81/empty.X81".to_owned()),
    )
    .expect("empty Texterstellung description should parse");

    let node = &document.boq.nodes[0];
    let item = node.item.as_ref().expect("item payload");
    assert_eq!(node.title, "Empty title");
    assert_eq!(item.short_text, "Empty title");
    assert_eq!(item.long_text, None);
}

#[test]
fn test_text_empty_table_and_empty_text_complement_are_preserved() {
    let document = gaeb_xml::parse_str(
        r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="T" RNoPart="T"><Description><CompleteText><DetailTxt><Text><table cellpadding="0"/><TextComplement Kind="Owner"/></Text></DetailTxt></CompleteText></Description></Item></BoQBody></BoQ></Project></GAEB>"#,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x81/empty-table.X81".to_owned()),
    )
    .expect("empty table should parse");

    let item = document.boq.nodes[0].item.as_ref().expect("item payload");
    let Some(RichText::Mixed(fragments)) = item.long_text.as_ref() else {
        assert!(
            matches!(item.long_text.as_ref(), Some(RichText::Mixed(_))),
            "expected mixed rich text"
        );
        return;
    };
    assert!(fragments.iter().any(|fragment| matches!(
        fragment,
        RichTextFragment::Table(markup) if markup.contains("<table")
    )));
    assert!(document.findings.iter().any(|finding| {
        finding.code == "gaeb_xml_texterstellung_text_complement_preserved_as_markup"
    }));
}

#[test]
fn test_text_nested_table_cdata_and_break_markup_are_preserved() {
    let document = gaeb_xml::parse_str(
        r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="N" RNoPart="N"><Description><CompleteText><DetailTxt><Text><table><tr><td><p><span>Alpha</span><br/><![CDATA[Beta]]></p></td></tr></table></Text></DetailTxt></CompleteText></Description></Item></BoQBody></BoQ></Project></GAEB>"#,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x81/nested-table.X81".to_owned()),
    )
    .expect("nested table and CDATA should parse");

    let item = document.boq.nodes[0].item.as_ref().expect("item payload");
    let Some(RichText::Mixed(fragments)) = item.long_text.as_ref() else {
        assert!(
            matches!(item.long_text.as_ref(), Some(RichText::Mixed(_))),
            "expected mixed rich text"
        );
        return;
    };
    assert!(fragments.iter().any(|fragment| matches!(
        fragment,
        RichTextFragment::Table(markup)
            if markup.contains("<tr>") && markup.contains("<![CDATA[Beta]]>")
    )));
    assert!(document.findings.iter().any(|finding| {
        finding.code == "gaeb_xml_texterstellung_layout_preserved_not_rendered"
    }));
}

#[test]
fn test_text_unclosed_description_reports_error() {
    let error = gaeb_xml::parse_str(
        r#"<GAEB><Project><BoQ><BoQBody><Item ID="U"><Description><CompleteText>"#,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x81/unclosed.X81".to_owned()),
    )
    .expect_err("unclosed Texterstellung description should fail");

    assert_eq!(error.code, "xml_unclosed_description");
}

#[test]
fn test_text_malformed_description_reports_parse_error() {
    let error = gaeb_xml::parse_str(
        r#"<GAEB><Project><BoQ><BoQBody><Item ID="M"><Description><Text><</Text></Description></Item></BoQBody></BoQ></Project></GAEB>"#,
        Some("gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x81/malformed.X81".to_owned()),
    )
    .expect_err("malformed Texterstellung description should fail");

    assert_eq!(error.code, "xml_parse_failed");
}

#[test]
fn test_texterstellung_support_promotion_requires_rich_text_evidence() {
    let manifest_text =
        std::fs::read_to_string("gaeb/manifest.toml").expect("manifest should be readable");
    let manifest: toml::Value = toml::from_str(&manifest_text).expect("manifest should parse");
    let fixtures = manifest
        .get("fixtures")
        .and_then(toml::Value::as_array)
        .expect("fixtures array");

    for (id, required_tests) in [
        (
            "bvbs_xml33_text_x81",
            [
                "test_text_x81_rich_text_blocks_preserved",
                "test_text_tables_normalize_to_document_blocks",
                "test_text_unsupported_layout_emits_findings",
            ],
        ),
        (
            "bvbs_xml33_text_x82",
            [
                "test_text_x82_cost_estimate_metadata_preserved",
                "test_text_unsupported_layout_emits_findings",
                "test_text_tables_normalize_to_document_blocks",
            ],
        ),
    ] {
        let fixture = fixtures
            .iter()
            .filter_map(toml::Value::as_table)
            .find(|fixture| fixture.get("id").and_then(toml::Value::as_str) == Some(id))
            .expect("missing manifest fixture");
        assert_eq!(
            fixture.get("support_status").and_then(toml::Value::as_str),
            Some("supported_parse_only")
        );
        let mapping = fixture
            .get("test_mapping")
            .and_then(toml::Value::as_array)
            .expect("test mapping")
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<Vec<_>>();
        for required_test in required_tests {
            assert!(
                mapping.contains(&required_test),
                "{id} missing test mapping {required_test}"
            );
        }
    }
}
