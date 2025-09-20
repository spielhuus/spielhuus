import init from "./game_of_life/game_of_life.js";

async function run() {
  const self = document.currentScript;
  const wasmUrl = (self as HTMLScriptElement)?.dataset.wasmUrl;
  if (!wasmUrl) {
    console.error("The data-wasm-url attribute is missing on the script tag.");
    return;
  }
  await init(wasmUrl);
}

run();
