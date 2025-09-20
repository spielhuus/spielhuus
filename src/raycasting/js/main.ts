import { drawGrid } from './grid';
import { createLevel } from './maps';
import { drawRaycast } from './canvas1';
import { drawViewRaycast } from './canvas2';
import { Vector2 } from './utils';
import type { DrawOptions } from './utils';

const canvas1 = document.querySelector<HTMLCanvasElement>('#grid-canvas');
const ctx1 = canvas1?.getContext('2d');

const canvas2 = document.querySelector<HTMLCanvasElement>('#view-canvas');
const ctx2 = canvas2?.getContext('2d');

const MAP = createLevel();

let drawOptions: DrawOptions = {
        gridSize: new Vector2(0,0),
        lineColor: 'rgba(50, 50, 200, 0.7)',
        lineWidth: 0.5,
        line_radius: 1,
        circle_radius: 4,
        circle_color: 'orange'
};

if (!canvas1 || !ctx1 || !canvas2 || !ctx2) {
        throw new Error('Failed to find one or more canvas elements or contexts');
}

// Set dimensions for both canvases
// TODO: respect window size
canvas1.width = 800;
canvas1.height = 600;
canvas2.width = 800;
canvas2.height = 600; 

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


// Unified draw function that updates both canvases
function draw() {
    if (!ctx1 || !canvas1 || !ctx2 || !canvas2) {
        return;
    }

    ctx1.clearRect(0, 0, canvas1.width, canvas1.height);
    drawGrid(canvas1, ctx1, MAP, drawOptions);
    MAP.draw(canvas1, ctx1);
    if (isMouseInside) {
        drawOptions.gridSize = MAP.cellSize(canvas1.width, canvas1.height);
        drawRaycast(canvas1, ctx1, mousePosition.x, mousePosition.y, MAP, drawOptions);
    }

    ctx2.clearRect(0, 0, canvas2.width, canvas2.height);
    drawGrid(canvas2, ctx2, MAP, drawOptions);
    MAP.draw(canvas2, ctx2);
    if (isMouseInside) {
        drawOptions.gridSize = MAP.cellSize(canvas1.width, canvas1.height);
        drawViewRaycast(canvas2, ctx2, mousePosition.x, mousePosition.y, drawOptions);
    }
}

// Initial draw
draw();
