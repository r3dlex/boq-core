/// <reference path="../rules.d.ts" />

export default {
  rules: {
    "boq-core-does-not-reference-obra-backend": {
      description: "boq-core MVP must not couple to or modify the Obra backend.",
      severity: "error",
      async check(ctx) {
        const files = await ctx.glob("**/*.{rs,toml,md,yml,yaml}");
        for (const file of files.filter((name) => !name.startsWith("target/") && !name.startsWith(".archgate/") && !name.startsWith("gaeb/developer_examples/"))) {
          const matches = await ctx.grep(file, /\.\.\/obra\/backend|obra\/backend|Obra\.Repo|Ecto\./);
          for (const match of matches) {
            ctx.report.violation({
              message: "MVP references Obra backend integration.",
              file: match.file,
              line: match.line,
              fix: "Keep backend integration as a later story; emit adapter DTOs only.",
            });
          }
        }
      },
    },
  },
} satisfies RuleSet;
