import { Vector2 } from '../../js/vector';
import { BasicNoise } from './basic.ts'
import { DDDNoise } from './3dnoise.ts'
import { WireframeTerrain } from './terrain';

const WIDTH = 1280;
const HEIGHT = 860;

async function main() {
  const basicCanvas = document.querySelector<HTMLCanvasElement>('#basic-canvas');
  const dddCanvas = document.querySelector<HTMLCanvasElement>('#ddd-canvas');
  const terrainCanvas = document.querySelector<HTMLCanvasElement>('#terrain-canvas');
  // const wallCanvas = document.querySelector<HTMLCanvasElement>('#wall-canvas');

  if (!basicCanvas || !dddCanvas || !terrainCanvas) {
    throw new Error('Canvas elements not found');
  }

  basicCanvas.width = dddCanvas.width = terrainCanvas.width = WIDTH;
  basicCanvas.height = dddCanvas.height = terrainCanvas.height = HEIGHT;

  // const animator = new WebGPURaycaster(gpuCanvas, moverCanvas, wallCanvas);
  // await animator.init();
  let basicNoise = new BasicNoise(basicCanvas);
  let dddNoise = new DDDNoise(dddCanvas);
  let terrain = new WireframeTerrain(terrainCanvas, {
      cols: 100,
      rows: 200,
      noiseScale: 8,
      amplitude: 250,
      color: '#000000ff'
  });
  basicNoise.draw(0);
  dddNoise.start();
  terrain.start();
}

main().catch(err => {
  console.error(err);
  alert('An error occurred. Check the console for details.');
});
