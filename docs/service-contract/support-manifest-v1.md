# boq-core support manifest contract v1

`boq-core.support-manifest.v1` is the stable JSON export of manifest-backed support status and capability policy for service consumers such as `boq-service /capabilities` and `/validate`.

## Contract files

- Schema: `docs/service-contract/support-manifest-v1.schema.json`
- Golden support manifest report: `tests/fixtures/service_contract/support_manifest.capabilities.json`

## Required fields

Every report contains:

- `schema_version`: always `boq-core.support-manifest.v1`
- `crate_version`: producing `boq-core` crate version
- `support_vocabulary`: exactly `supported`, `supported_parse_only`, `future_track`, `reference_only`
- `entries`: fixture/source policy rows sorted by `fixture_id`
- `production_ready`: always `false`
- `certification_claims`: always empty

Each entry contains:

- `fixture_id`, `source_family`, `process_domain`, `gaeb_version`, `phase`, and `target_dir`
- `support_status`: exact manifest vocabulary, never extended with production/certification language
- `capabilities`: `detect`, `parse`, `validate`, `adapt_to_obra`, `export`, `roundtrip`, `certification`, and `reference_only`
- `source_policy`: CI policy, license note, checksum presence, and whether service export requires external downloads
- `test_mapping`: manifest tests that back supported claims

## Support-honesty rules

- `certification`: always `false`; official paid certification is outside this export.
- `service_export_requires_external_download`: always `false`; services can consume the export without fetching paid or external standards data.
- `future_track` and `reference_only` rows must not claim parse, validate, Obra adapter, export, roundtrip, or certification capability.
- `supported_parse_only` rows must not imply validation, export, roundtrip, production, or certification support.
