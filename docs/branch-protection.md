# Branch Protection Plan

Target branch: `main`.

Required rules for the future `r3dlex/boq-core` repository:

- Require pull requests before merging.
- Require all CI checks to pass before merge.
- Require the `Rust quality gates` workflow.
- Require branch to be up to date before merge.
- Disallow force pushes.
- Disallow branch deletion.
- Restrict bypasses to repository administrators only when explicitly approved.
- Treat paid BVBS submission and official certification claims as user-confirmation-gated external actions.

This document is a local plan; applying GitHub branch protection is an external repository action.
