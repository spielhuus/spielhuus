import { Canvas1 } from './canvas1';
import { Vector2 } from '../../js/vector';
import { DrawOptions } from './utils';

const canvas1 = document.querySelector<HTMLCanvasElement>('#canvas1');
const ctx1 = canvas1?.getContext('2d');

if (!canvas1 || !ctx1) {
  throw new Error('Failed to find one or more canvas elements or contexts');
}

// Set dimensions for both canvases
// TODO: respect window size
canvas1.width = 800;
canvas1.height = 600;
let drawOptions = new DrawOptions();

const ray_canvas = new Canvas1(canvas1, ctx1, drawOptions);

let isMouseInside = false;
const mousePosition = { x: 0, y: 0 };

function updateMousePosition(event: MouseEvent) {
  const canvas = event.target as HTMLCanvasElement;
  const rect = canvas.getBoundingClientRect();
  mousePosition.x = event.clientX - rect.left;
  mousePosition.y = event.clientY - rect.top;
  requestAnimationFrame(draw);
}

function handleMouseEnter() {
  isMouseInside = true;
  requestAnimationFrame(draw);
}

function handleMouseLeave() {
  isMouseInside = false;
  requestAnimationFrame(draw);
}


// canvas1.addEventListener('focus', () => {
//   ray_canvas.isActive = true;
//   requestAnimationFrame((time) => ray_canvas.draw(time));
// });
//
// canvas1.addEventListener('blur', () => {
//   ray_canvas.isActive = false;
// });
//
// requestAnimationFrame((time) => walls_canvas.draw(time));
// requestAnimationFrame((time) => final_canvas.draw(time));

// Unified draw function that updates both canvases
function draw() {
  if (!ctx1 || !canvas1) {
    return;
  }

  ray_canvas.draw(0);
}
// const ray_canvas = new Canvas1(canvas1, ctx1, drawOptions);
// Initial draw
draw();
