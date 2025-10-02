import { Canvas1 } from './canvas1';
import { Canvas2 } from './canvas2';
import { Canvas3 } from './canvas3';
import { Vector2 } from '../../js/vector';
import type { DrawOptions } from './utils';

const canvas1 = document.querySelector<HTMLCanvasElement>('#grid-canvas');
const ctx1 = canvas1?.getContext('2d');

const canvas2 = document.querySelector<HTMLCanvasElement>('#view-canvas');
const ctx2 = canvas2?.getContext('2d');

const canvas3 = document.querySelector<HTMLCanvasElement>('#wall-canvas');
const ctx3 = canvas3?.getContext('2d');

if (!canvas1 || !ctx1 || !canvas2 || !ctx2 || !canvas3 || !ctx3) {
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

// const MAP = createLevel();
// let player = new Player(canvas1.width/2, canvas2.height/2, 0)

let drawOptions: DrawOptions = {
        gridSize: new Vector2(0,0),
        lineColor: 'rgba(50, 50, 200, 0.7)',
        lineWidth: 0.5,
        line_radius: 1,
        circle_radius: 4,
        circle_color: 'orange',
        wallColor: 'rgba(50, 50, 200, 0.7)'
};


const ray_canvas = new Canvas1();
const proj_canvas = new Canvas2();
const walls_canvas = new Canvas3(canvas3, ctx3, drawOptions);

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
    if (!ctx1 || !canvas1 || !ctx2 || !canvas2 || !ctx3 || !canvas3) {
        return;
    }

    ray_canvas.draw(canvas1, ctx1, new Vector2(mousePosition.x, mousePosition.y), drawOptions);
    proj_canvas.draw(canvas2, ctx2, new Vector2(mousePosition.x, mousePosition.y), drawOptions);
    requestAnimationFrame((time) => walls_canvas.draw(time));
}

// Initial draw
draw();
