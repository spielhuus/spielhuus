import { Vector2 } from '../../js/vector';

const WALLS_COUNT = 5;
const RAY_ANGLE = 0.02;
const TRANSPARENT = "#00000000";
const WIDTH = 1280;
const HEIGHT = 860;

const canvas1 = document.querySelector<HTMLCanvasElement>('#wall-canvas');
const ctx1 = canvas1?.getContext('2d');

const canvas2 = document.querySelector<HTMLCanvasElement>('#mover-canvas');
const ctx2 = canvas2?.getContext('2d');

const canvas3 = document.querySelector<HTMLCanvasElement>('#ray-canvas');
const ctx3 = canvas3?.getContext('2d');


if (!canvas1 || !ctx1 || !canvas2 || !ctx2 || !canvas3 || !ctx3) {
  throw new Error('Failed to find one or more canvas elements or contexts');
}

class Line {
  constructor(public start: Vector2, public end: Vector2, public width: number, public color: string) { }
}

class SolutionAnimator {
  walls: Line[];
  position: Vector2;
  acceleration = new Vector2(0, 0);
  velocity: Vector2;
  theta = 0;
  maxSpeed = 2;
  circleCenter = new Vector2(0,0);
  target = new Vector2(0,0);
  circleRadius = 15;
  black: string;
  darkorange: string;

  constructor(private ctx1: CanvasRenderingContext2D, 
              private ctx2: CanvasRenderingContext2D, 
              private ctx3: CanvasRenderingContext2D) {

    let rootStyles = getComputedStyle(document.documentElement);
    this.black = rootStyles.getPropertyValue('--black');
    this.darkorange = rootStyles.getPropertyValue('--darkorange');
    this.position = new Vector2(WIDTH / 2, HEIGHT / 2);
    this.velocity = new Vector2(Math.random() * 2 - 1, Math.random() * 2 - 1);
    
    this.walls = [
      // the border around the canvas
      new Line(new Vector2(0, 0), new Vector2(WIDTH, 0), 2, TRANSPARENT),
      new Line(new Vector2(WIDTH, 0), new Vector2(WIDTH, HEIGHT), 1, TRANSPARENT),
      new Line(new Vector2(WIDTH, HEIGHT), new Vector2(0, HEIGHT), 1, TRANSPARENT),
      new Line(new Vector2(0, HEIGHT), new Vector2(0, 0), 2, TRANSPARENT),
    ];

    // some lines
    for (let _i = 0; _i < WALLS_COUNT; _i++) {
      let x1 = Math.floor(Math.random() * WIDTH);
      let y1 = Math.floor(Math.random() * HEIGHT);
      let x2 = Math.floor(Math.random() * WIDTH);
      let y2 = Math.floor(Math.random() * HEIGHT);
      this.walls.push(new Line(new Vector2(x1, y1), new Vector2(x2, y2), 4, this.black));
    }
  }

  move() {
    const circleDistance = 50;
    const circleRadius = 15;

    // calculate the force
    this.circleCenter = this.velocity.length() > 0
      ? this.velocity.norm().mul_scalar(circleDistance)
      : new Vector2(circleDistance, 0);
    this.theta += (Math.random() * 2 - 1) * 0.3;
    this.target = new Vector2(
      Math.cos(this.theta) * circleRadius, Math.sin(this.theta) * circleRadius
    );

    // calculate next position
    const wanderForce = this.circleCenter.add(this.target);
    this.acceleration = this.acceleration.add(wanderForce);
    this.velocity = this.velocity.add(this.acceleration);
    this.velocity = this.velocity.limit(this.maxSpeed);
    this.position = this.position.add(this.velocity);
    this.acceleration = this.acceleration.mul_scalar(0);

    //wrap edges 
    if (this.position.x > WIDTH) this.position.x = 0;
    if (this.position.x < 0) this.position.x = WIDTH;
    if (this.position.y > HEIGHT) this.position.y = 0;
    if (this.position.y < 0) this.position.y = HEIGHT;
  }

  intersection(dir: Vector2, wall: Line): Vector2 | void {
    const x1 = wall.start.x;
    const y1 = wall.start.y;
    const x2 = wall.end.x;
    const y2 = wall.end.y;
    const x3 = this.position.x;
    const y3 = this.position.y;
    const x4 = this.position.x + dir.x;
    const y4 = this.position.y + dir.y;

    const den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if (den == 0) {
      return;
    }

    const t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
    const u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;
    if (t > 0 && t < 1 && u > 0) {
      const pt = new Vector2(
        x1 + t * (x2 - x1),
        y1 + t * (y2 - y1)
      );
      return pt;
    } else {
      return;
    }
  }

  rays() {
    for (let r = 0; r < 2 * Math.PI - RAY_ANGLE; r += RAY_ANGLE) {
      let shortest_interseption = this.position.add(Vector2.max());
      let dir = new Vector2(0, 0).direction(r);
      for (let wall of this.walls) {
        let pt = this.intersection(dir, wall);
        if (pt) {
          if (this.position.distance(shortest_interseption) >
            this.position.distance(pt)) {
            shortest_interseption = pt;
          }
        }
      }

      const blackRgba = this.black.replace('rgb', 'rgba').replace(')', ', 0.2)');
      this.ctx3.fillStyle = blackRgba;
      this.ctx3.strokeStyle = blackRgba;
      this.ctx3.lineWidth = 1;
      this.ctx3.beginPath();
      this.ctx3.moveTo(this.position.x, this.position.y);
      this.ctx3.lineTo(shortest_interseption.x, shortest_interseption.y);
      this.ctx3.stroke();
    }
  }

  draw_walls(ctx: CanvasRenderingContext2D) {
    for (let line of this.walls) {
      ctx.strokeStyle = line.color;
      ctx.lineCap = 'round';
      ctx.lineJoin = 'round';
      ctx.lineWidth = line.width;
      ctx.beginPath();
      ctx.moveTo(line.start.x, line.start.y);
      ctx.lineTo(line.end.x, line.end.y);
      ctx.stroke();
    }
  }

  draw_mover() {
      this.ctx2.save();
      this.ctx2.fillStyle = this.darkorange;
      this.ctx2.strokeStyle = this.darkorange;
      this.ctx2.lineWidth = 1;
      this.ctx2.beginPath();
      this.ctx2.arc(this.position.x, this.position.y, 8, 0, Math.PI * 2);
      this.ctx2.fill();
      this.ctx2.stroke();

      this.ctx2.beginPath();
      this.ctx2.moveTo(...this.position.array());
      this.ctx2.lineTo(...this.position.add(this.acceleration).array());
      this.ctx2.stroke();

      this.ctx2.beginPath();
      this.ctx2.moveTo(...this.position.array());
      this.ctx2.lineTo(...this.position.add(this.circleCenter).array());
      this.ctx2.stroke();

      this.ctx2.strokeStyle = this.darkorange;
      this.ctx2.lineWidth = 1;
      this.ctx2.beginPath();
      this.ctx2.arc(...this.position.add(this.circleCenter).array(), this.circleRadius, 0, Math.PI * 2);
      this.ctx2.stroke();

      this.ctx2.fillStyle = this.darkorange;
      this.ctx2.strokeStyle = this.darkorange;
      this.ctx2.lineWidth = 1;
      this.ctx2.beginPath();
      this.ctx2.arc(...this.position.add(this.circleCenter).add(this.target).array(), 4, 0, Math.PI * 2);
      this.ctx2.fill();
      this.ctx2.stroke();
      this.ctx2.restore();
  }

  updateAndDraw() {
    if (this.walls.length === 0) return;

    let rootStyles = getComputedStyle(document.documentElement);
    this.black = rootStyles.getPropertyValue('--black');
    this.darkorange = rootStyles.getPropertyValue('--darkorange');
    for (let wall of this.walls) {
      if (wall.color !== TRANSPARENT) {
        wall.color = this.black;
      }
    }

    this.ctx1.clearRect(0, 0, this.ctx3.canvas.width, this.ctx3.canvas.height);
    this.ctx2.clearRect(0, 0, this.ctx3.canvas.width, this.ctx3.canvas.height);
    this.ctx3.clearRect(0, 0, this.ctx3.canvas.width, this.ctx3.canvas.height);

    this.move();
    this.draw_walls(this.ctx1);
    this.draw_mover();
    this.draw_walls(this.ctx3);

    this.rays();

    requestAnimationFrame(() => this.updateAndDraw());
  }
}

// Set dimensions for the canvases
canvas1.width = WIDTH;
canvas1.height = HEIGHT;
canvas2.width = WIDTH;
canvas2.height = HEIGHT;
canvas3.width = WIDTH;
canvas3.height = HEIGHT;

const drawer = new SolutionAnimator(ctx1, ctx2, ctx3);

function draw() {
  drawer.updateAndDraw();
}

draw();
