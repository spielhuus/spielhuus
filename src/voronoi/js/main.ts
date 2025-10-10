import { JsVoronoi } from "./js_voronoi";
import { JfaVoronoi } from "./jfa_voronoi";
import { GpuVoronoi, DisplayMode } from "./gpu_voronoi";

const canvas1 = document.querySelector<HTMLCanvasElement>('#raw-canvas');
const ctx1 = canvas1?.getContext('2d');

const canvas2 = document.querySelector<HTMLCanvasElement>('#jfa-canvas');
const ctx2 = canvas2?.getContext('2d');

const canvas3 = document.querySelector<HTMLCanvasElement>('#jfa-gpu');

if (!canvas1 || !ctx1 || !canvas2 || !ctx2 || !canvas3) {
  throw new Error('Failed to find one or more canvas elements or contexts');
}

// TODO: respect window size
canvas1.width = 800;
canvas1.height = 600;
canvas2.width = 800;
canvas2.height = 600;
canvas3.width = 800;
canvas3.height = 600;

const raw_canvas = new JsVoronoi(canvas1, ctx1);
const jfa_canvas = new JfaVoronoi(canvas2, ctx2);
const gpu_canvas = new GpuVoronoi(canvas3);

document.getElementById('btn-cells')?.addEventListener('click', () => {
    gpu_canvas.setDisplayMode(DisplayMode.CELLS);
});
document.getElementById('btn-grayscale')?.addEventListener('click', () => {
    gpu_canvas.setDisplayMode(DisplayMode.GRAYSCALE);
});
document.getElementById('btn-heatmap')?.addEventListener('click', () => {
    gpu_canvas.setDisplayMode(DisplayMode.HEATMAP);
});
document.getElementById('btn-contours')?.addEventListener('click', () => {
    gpu_canvas.setDisplayMode(DisplayMode.CONTOURS);
});



async function run() {
  raw_canvas.draw();
  jfa_canvas.draw();

  const res = await gpu_canvas.init();
  if (res) {
    gpu_canvas.draw();
  } else {
    document.body.innerHTML = "WebGPU is not supported on this browser.";
  }
}

run();
