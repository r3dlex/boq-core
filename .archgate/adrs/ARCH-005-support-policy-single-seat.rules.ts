/// <reference path="../rules.d.ts" />

export default {
  rules: {
    "manifest-toml-included-only-from-support-module": {
      description: "Only src/support/ may include the embedded GAEB fixture manifest TOML.",
      severity: "error",
      async check(ctx) {
        const files = await ctx.glob("src/**/*.rs");
        for (const file of files.filter((name) => !name.startsWith("src/support/"))) {
          const matches = await ctx.grep(file, /include_str!\([^)]*manifest\.toml/);
          for (const match of matches) {
            ctx.report.violation({
              message: "include_str!(\"...manifest.toml\") outside src/support/ duplicates the manifest seat.",
              file: match.file,
              line: match.line,
              fix: "Load the manifest through boq_core::support::manifest::parse instead.",
            });
          }
        }
      },
    },
    "supported-status-must-come-from-policy": {
      description: "Parser modules must not construct SupportStatus::Supported directly; ask the policy.",
      severity: "error",
      async check(ctx) {
        const files = await ctx.glob("src/gaeb_xml/**/*.rs");
        files.push(...(await ctx.glob("src/gaeb90.rs")));
        // Match struct-field construction like `support_status: SupportStatus::Supported,`
        // used when building documents. Explicit comparisons (assert_eq!) and
        // rebinding inside test blocks legitimately mention the variant and are
        // not flagged.
        const pattern = /:\s*SupportStatus::Supported\s*,/;
        for (const file of files) {
          const matches = await ctx.grep(file, pattern);
          for (const match of matches) {
            ctx.report.violation({
              message: "Parser module constructs SupportStatus::Supported directly instead of consulting the support policy.",
              file: match.file,
              line: match.line,
              fix: "Call boq_core::support::default_policy().decide(...) and use the returned status/capabilities.",
            });
          }
        }
      },
    },
  },
} satisfies RuleSet;
