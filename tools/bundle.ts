import { rolldown } from "rolldown";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const projectRoot = resolve(__dirname, "..");

async function bundle(): Promise<void> {
  const bundler = await rolldown({
    input: resolve(projectRoot, "crates/lib_vue_compiler_sfc_sys/ffi/js/vue_compiler_sfc_bridge.js"),
    platform: "browser",
    resolve: {
      conditionNames: ["module", "import", "browser", "default"],
      // Look for node_modules in tools directory
      modules: [resolve(__dirname, "node_modules"), "node_modules"],
    },
  });

  await bundler.write({
    file: resolve(projectRoot, "dist/vue-compiler.js"),
    format: "iife",
    name: "VueCompiler",
  });

  console.log("Bundle created: dist/vue-compiler.js");
}

bundle().catch((err) => {
  console.error("Bundle failed:", err);
  process.exit(1);
});
