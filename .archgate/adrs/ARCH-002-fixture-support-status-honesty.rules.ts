/// <reference path="../rules.d.ts" />

export default {
  rules: {
    "manifest-support-claims-have-tests": {
      description: "Supported fixtures must map to tests; reference-only fixtures must not claim test mappings.",
      severity: "error",
      async check(ctx) {
        const file = "gaeb/manifest.toml";
        const text = await ctx.readFile(file).catch(() => "");
        if (text.length === 0) {
          return;
        }
        const blocks = text.split(/\n\[\[fixtures\]\]\n/).slice(1);
        for (const block of blocks) {
          const id = block.match(/id = "([^"]+)"/)?.[1] ?? "unknown";
          const status = block.match(/support_status = "([^"]+)"/)?.[1] ?? "";
          const mapping = block.match(/test_mapping = \[(.*)\]/)?.[1] ?? "";
          const hasMapping = mapping.trim().length > 0;
          if ((status === "supported" || status === "supported_parse_only") && !hasMapping) {
            ctx.report.violation({ message: `Supported fixture lacks test mapping: ${id}`, file });
          }
          if (status === "reference_only" && hasMapping) {
            ctx.report.violation({ message: `Reference-only fixture claims test mapping: ${id}`, file });
          }
        }
      },
    },
    "reference-executables-not-supported": {
      description: "Executable payloads must stay reference-only.",
      severity: "error",
      async check(ctx) {
        const file = "gaeb/manifest.toml";
        const text = await ctx.readFile(file).catch(() => "");
        if (text.length === 0) {
          return;
        }
        const blocks = text.split(/\n\[\[fixtures\]\]\n/).slice(1);
        for (const block of blocks) {
          const id = block.match(/id = "([^"]+)"/)?.[1] ?? "unknown";
          const status = block.match(/support_status = "([^"]+)"/)?.[1] ?? "";
          const phase = block.match(/phase = "([^"]+)"/)?.[1] ?? "";
          const url = block.match(/normalized_url = "([^"]+)"/)?.[1] ?? "";
          if ((phase === "exe" || /\.(exe|bat|cmd|com|ps1|sh|msi)$/i.test(url)) && status !== "reference_only") {
            ctx.report.violation({ message: `Executable fixture is not reference-only: ${id}`, file });
          }
        }
      },
    },
  },
} satisfies RuleSet;
