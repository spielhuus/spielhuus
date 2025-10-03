import { Canvas1 } from './canvas1';
import { Canvas2 } from './canvas2';
import { Canvas3 } from './canvas3';
import { Canvas4 } from './canvas4';
import { Vector2 } from '../../js/vector';
import { DrawOptions } from './utils';

const canvas1 = document.querySelector<HTMLCanvasElement>('#grid-canvas');
const ctx1 = canvas1?.getContext('2d');

const canvas2 = document.querySelector<HTMLCanvasElement>('#view-canvas');
const ctx2 = canvas2?.getContext('2d');

const canvas3 = document.querySelector<HTMLCanvasElement>('#wall-canvas');
const ctx3 = canvas3?.getContext('2d');

const canvas4 = document.querySelector<HTMLCanvasElement>('#final-canvas');
const ctx4 = canvas4?.getContext('2d');

if (!canvas1 || !ctx1 || !canvas2 || !ctx2 || !canvas3 || !ctx3 || !canvas4 || !ctx4) {
  throw new Error('Failed to find one or more canvas elements or contexts');
}

// Set dimensions for both canvases
// TODO: respect window size
canvas1.width = 800;
canvas1.height = 600;
canvas2.width = 800;
canvas2.height = 600;
canvas3.width = 1280;
canvas3.height = 960;
canvas4.width = 1280;
canvas4.height = 960;

let drawOptions = new DrawOptions();

const ray_canvas = new Canvas1();
const proj_canvas = new Canvas2();
const walls_canvas = new Canvas3(canvas3, ctx3, drawOptions);
const final_canvas = new Canvas4(canvas4, ctx4, drawOptions);
// Initialize the raycaster, and *then* start the game.
final_canvas.initialize().then(() => {
    // Optional: Add a click listener to start the game after loading
    // canvas4.addEventListener('click', () => {
    //     if (!raycaster.isActive) {
    //         startGame();
    //     }
    // }, { once: true }); // { once: true } makes the listener fire only once

    // For now, let's just show a "Ready" message
    ctx4.fillStyle = 'black';
    ctx4.fillRect(0, 0, canvas4.width, canvas4.height);
    ctx4.font = '30px Arial';
    ctx4.fillStyle = 'white';
    ctx4.textAlign = 'center';
    ctx4.fillText('Textures Loaded. Click to Start!', canvas4.width / 2, canvas4.height / 2);

}).catch(error => {
    console.error("Failed to start the raycaster:", error);
});

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

// Add event listeners to both canvases
[canvas1, canvas2].forEach(canvas => {
  canvas.addEventListener('mousemove', updateMousePosition);
  canvas.addEventListener('mouseenter', handleMouseEnter);
  canvas.addEventListener('mouseleave', handleMouseLeave);
});

canvas3.addEventListener('focus', () => {
  walls_canvas.isActive = true;
  requestAnimationFrame((time) => walls_canvas.draw(time));
});

canvas3.addEventListener('blur', () => {
  walls_canvas.isActive = false;
});

canvas4.addEventListener('focus', () => {
  final_canvas.isActive = true;
  requestAnimationFrame((time) => final_canvas.draw(time));
});

canvas4.addEventListener('blur', () => {
  final_canvas.isActive = false;
});

requestAnimationFrame((time) => walls_canvas.draw(time));
requestAnimationFrame((time) => final_canvas.draw(time));

// Unified draw function that updates both canvases
function draw() {
  if (!ctx1 || !canvas1 || !ctx2 || !canvas2 || !ctx3 || !canvas3) {
    return;
  }

  ray_canvas.draw(canvas1, ctx1, new Vector2(mousePosition.x, mousePosition.y), drawOptions);
  proj_canvas.draw(canvas2, ctx2, new Vector2(mousePosition.x, mousePosition.y), drawOptions);
}

// Initial draw
draw();
