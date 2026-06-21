# AVA rich text and schema-version handling

AVA XML descriptions now use the same loss-aware rich-description path as the
Texterstellung parser when the source URI is under `gaeb/.../ava/...`.

## Contract

- Plain text remains available as `BoqItem.short_text` for adapter consumers.
- Markup-rich descriptions are preserved in `BoqItem.long_text` as
  `RichText::XhtmlFragment` or `RichText::Mixed`.
- Tables are surfaced as `RichTextFragment::Table` when table markup is present.
- Layout/style-sensitive markup emits a recoverable finding instead of being
  silently dropped.
- Unknown AVA item children are preserved under `gaeb.unsupported.<Tag>` node
  metadata and emit `gaeb_xml_unsupported_item_field` findings.
- Namespace-derived XML versions are recorded in `summary.version`,
  `source.gaeb_version`, and `boq.metadata["gaeb.namespace"]` without promoting
  unsupported schema versions.

This is readiness evidence only; it is not official BVBS certification.
