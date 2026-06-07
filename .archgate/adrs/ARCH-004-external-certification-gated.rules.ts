/// <reference path="../rules.d.ts" />

export default {
  rules: {
    "docs-do-not-claim-official-certification": {
      description: "Docs and CI may claim certification readiness, not official BVBS certification.",
      severity: "error",
      async check(ctx) {
        const files = await ctx.glob("**/*.{md,yml,yaml,toml}");
        const forbidden = /officially certified|BVBS certified|certified by BVBS/i;
        for (const file of files.filter((name) => !name.startsWith("target/") && !name.startsWith("gaeb/developer_examples/"))) {
          const matches = await ctx.grep(file, forbidden);
          for (const match of matches) {
            ctx.report.violation({
              message: "Text appears to claim official BVBS certification instead of readiness.",
              file: match.file,
              line: match.line,
              fix: "Use certification-path readiness unless official certification has been obtained.",
            });
          }
        }
      },
    },
  },
} satisfies RuleSet;
