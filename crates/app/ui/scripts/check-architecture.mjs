// Enforces the folder rules from docs/plan/implementation-plan.md, part C3, and one
// layout rule that no test in jsdom can see.
import { readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";

const root = resolve(import.meta.dirname, "..", "src");
const generated = join(root, "ipc", "generated");
const contractTypes = new Set(
  readdirSync(generated).filter((f) => f.endsWith(".ts")).map((f) => f.replace(/\.ts$/, "")),
);

function* files(dir) {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) yield* files(path);
    else if (/\.(ts|tsx)$/.test(name)) yield path;
  }
}

// Resolves an import specifier to a path under src, or null for packages.
function target(file, spec) {
  if (spec.startsWith("@/")) return join(root, spec.slice(2));
  if (spec.startsWith(".")) return resolve(dirname(file), spec);
  return null;
}

const area = (path) => relative(root, path).split(sep);
const problems = [];
for (const file of files(root)) {
  if (file.startsWith(generated)) continue;
  const source = readFileSync(file, "utf8");
  const [top, feature] = area(file);
  const at = relative(resolve(root, ".."), file);
  for (const [, spec] of source.matchAll(/(?:from|import)\s*\(?\s*["']([^"']+)["']/g)) {
    if (spec.startsWith("@tauri-apps/") && top !== "ipc") {
      problems.push(`${at}: only src/ipc may import ${spec}`);
    }
    const to = target(file, spec);
    if (!to) continue;
    const [toTop, toFeature] = area(to);
    if (top === "features" && toTop === "features" && toFeature !== feature) {
      problems.push(`${at}: feature "${feature}" imports feature "${toFeature}"; share through design/ or ipc/`);
    }
    if ((top === "design" || top === "ipc") && (toTop === "features" || toTop === "shell")) {
      problems.push(`${at}: ${top}/ must not depend on ${toTop}/`);
    }
  }
  // A scrolling box must be positioned. Visually hidden text (`sr-only`) is absolutely
  // positioned, so inside an unpositioned scroller it stays put while the content
  // scrolls and stretches whatever does contain it: the window scrolled past the end
  // of the Network tab this way.
  for (const [, classes] of source.matchAll(/["'`]([^"'`\n]*\boverflow-(?:[xy]-)?(?:auto|scroll)\b[^"'`\n]*)["'`]/g)) {
    if (!/(?:^|\s)(?:relative|absolute|fixed|sticky)(?:\s|$)/.test(classes)) {
      problems.push(`${at}: scroll container "${classes.trim()}" needs "relative" so hidden labels scroll with it`);
    }
  }
  // Declarations only (`type X =`, `interface X`), not `type X` inside an import list.
  const declarations = /(?:^|\n)\s*(?:export\s+)?(?:type\s+([A-Z]\w*)\s*(?:<[^>]*>)?\s*=|interface\s+([A-Z]\w*)\b)/g;
  for (const [, typeName, interfaceName] of source.matchAll(declarations)) {
    const name = typeName ?? interfaceName;
    if (contractTypes.has(name)) {
      problems.push(`${at}: hand-written contract type ${name}; import it from src/ipc/generated`);
    }
  }
}
if (problems.length) {
  console.error(problems.join("\n"));
  process.exit(1);
}
console.log(`Architecture rules hold for ${[...files(root)].length} files.`);
