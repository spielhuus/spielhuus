import init from "./ahoi_wgpu/ahoi_wgpu.js";

async function run() {
  const self = document.currentScript;
  const wasmUrl = (self as HTMLScriptElement)?.dataset.wasmUrl;
  if (!wasmUrl) {
    console.error("The data-wasm-url attribute is missing on the script tag.");
    return;
  }
  await init({module_or_path: wasmUrl});
}

run();
