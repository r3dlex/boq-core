# Coverage Policy

Target: **95% minimum for all three selected Rust coverage measurements**:

- line coverage
- function coverage
- region coverage

Enforced command:

```bash
cargo llvm-cov --all-features --summary-only \
  --ignore-filename-regex 'src/bin/xtask.rs' \
  --fail-under-lines 95 \
  --fail-under-functions 95 \
  --fail-under-regions 95
```

The `xtask` binary is excluded from the library coverage denominator because it is an external fixture-acquisition CLI. It is still covered by dedicated unit tests for URL allowlisting, unsafe paths, checksum requirements, and executable quarantine, plus the offline manifest test and explicit `cargo run --bin xtask -- fixtures verify` quality gate. Parser, model, adapter, validation, support-status, criteria, and fixture-catalog library paths are included.

Downloaded fixture payloads, caches, and generated coverage artifacts are excluded from git and from the coverage denominator.
