/// <reference path="../rules.d.ts" />

export default {
  rules: {
    "parser-modules-do-not-import-obra-adapter": {
      description: "Parser modules must parse into the GAEB domain model, not Obra adapter DTOs.",
      severity: "error",
      async check(ctx) {
        const files = await ctx.glob("src/gaeb*/**/*.rs");
        for (const file of files) {
          const matches = await ctx.grep(file, /crate::adapter|ObraImportDocument|ObraLineItem|ObraWbsNodeCandidate/);
          for (const match of matches) {
            ctx.report.violation({
              message: "Parser module imports or references Obra adapter DTOs.",
              file: match.file,
              line: match.line,
              fix: "Parse into the GAEB domain model first; keep Obra mapping in src/adapter/.",
            });
          }
        }
      },
    },
    "adapter-reports-document-findings": {
      description: "The Obra adapter must propagate parser findings into its loss report.",
      severity: "error",
      async check(ctx) {
        const file = "src/adapter/obra.rs";
        const text = await ctx.readFile(file).catch(() => "");
        if (text.length === 0) {
          return;
        }
        if (!text.includes("warnings: document.findings.clone()")) {
          ctx.report.violation({
            message: "Adapter loss report does not propagate parser findings.",
            file,
            fix: "Set loss_report.warnings from document.findings.",
          });
        }
      },
    },
  },
} satisfies RuleSet;
