# GAEB XML 3.4 beta impact notes

Issue: #38
ADR: ARCH-007
Status: reference_only impact tracking; not production parser support.

## Source rows

| Manifest id | Purpose | Status | CI policy |
| --- | --- | --- | --- |
| `official_gaeb_xml34_beta_schema` | Official GAEB XML 3.4 beta schema package reference | `reference_only` | `download_on_demand` |
| `official_gaeb_xml34_beta_changelog` | Official GAEB XML 3.4 beta-vs-3.3 changelog reference | `reference_only` | `download_on_demand` |

## Model extension points to monitor

The beta changelog/schema track is expected to affect metadata rather than stable import behavior until a stable GAEB XML 3.4 release is verified.

| Impact area | Potential boq-core model location | Current handling |
| --- | --- | --- |
| Sustainability descriptors | `BoqItem.metadata["gaeb.beta.sustainability"]` or a future typed extension | Record as structured findings/metadata only after stable evidence. |
| Lifecycle descriptors | `BoqItem.metadata["gaeb.beta.lifecycle"]` or future planning module | No support claim; monitor for follow-up stable schema issue. |
| Carbon / CO2 descriptors | `BoqItem.metadata["gaeb.beta.carbon"]` or future environmental data extension | Reference-only; no calculation or export semantics. |
| Changelog schema deltas | `GaebDocument.boq.metadata["gaeb.xml34_beta_impact"]` | Planning artifact only until tests and stable fixtures exist. |

## Support boundary

GAEB XML 3.4 beta remains `reference_only`. It is not a BVBS certification fixture, is not production supported, and must not be promoted by namespace detection alone. A future implementation must add failing tests, stable fixture checksums, and explicit review evidence before support status can change.
