# Market overlay readiness contract v1

Schema version: `boq-core.market-overlays.v1`

The market overlay readiness report is a service-facing matrix for the
fixture-backed overlay modules that can enrich parsed BoQ items with market
evidence. It is intentionally a readiness and evidence contract, not a catalog
distribution mechanism.

## Command

```sh
boq-core-service market-overlays
```

## Rows

The report currently includes:

- `sinapi-bdi` — SINAPI catalog and BDI evidence from
  `tests/fixtures/synthetic/sinapi_catalog.json`.
- `prezzario-computo` — Italian Prezzario and Computo Metrico evidence from
  `tests/fixtures/synthetic/prezzario_computo.json`.
- `catalogo-cuadro` — Catálogo de Conceptos and Cuadro de Precios evidence from
  `tests/fixtures/synthetic/catalogo_cuadro.json`.
- `stabu-raw` — STABU and RAW exchange evidence from
  `tests/fixtures/synthetic/stabu_raw.json`.
- `dqe-quantity` — DQE quantity-estimate evidence from
  `tests/fixtures/synthetic/dqe_quantity.json`.

Each row lists the owning Rust module, deterministic fixture, service contracts
where the evidence can appear, metadata keys carried when present, loss-finding
semantics, and the current support boundary.

## Support honesty

- `production_ready`: always `false`.
- `certification_claims`: always empty.
- `external_catalog_download_required`: always `false`.
- `promotes_support_status`: always `false`.
- `grants_adapter_support_to_parse_only`: always `false`.
- `complete_market_coverage_claimed`: always `false`.

Applying an overlay may add annotations, provenance, and loss findings to a
parsed document. It must not change the document `support_status`, must not
grant Obra adapter support to parse-only inputs, and must not claim complete
market coverage or certification.
