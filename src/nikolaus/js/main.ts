const canvas1 = document.querySelector<HTMLCanvasElement>('#static-canvas');
const ctx1 = canvas1?.getContext('2d');

const canvas2 = document.querySelector<HTMLCanvasElement>('#draw-canvas');
const ctx2 = canvas2?.getContext('2d');

const MOVES = [
  [0, 1, 0, 1, 1],
  [1, 0, 1, 1, 1],
  [0, 1, 0, 1, 0],
  [1, 1, 1, 0, 1],
  [1, 1, 0, 1, 0],
];

type Point = [number, number];
const POSITIONS: Point[] = [
  [50, 180],
  [50, 80],
  [100, 20],
  [150, 80],
  [150, 180],
];

class SolutionAnimator {

  private selectedSolutionIndex = 0;
  private currentStep = 1;
  private lineProgress = 0.0;

  constructor(private ctx: CanvasRenderingContext2D, private speed: number, private solutions: Result) {

    const container = document.getElementById("solutions");
    if (!container) return;

    container.innerHTML = '';

    solutions.forEach((result, i) => {
      const pathString = result[0].join('');

      const p = document.createElement('p');

      const radio = document.createElement('input');
      radio.type = 'radio';
      radio.id = `solution${i}`;
      radio.name = 'selected';
      radio.value = i.toString();
      if (i === 0) {
        radio.checked = true;
      }

      const label = document.createElement('label');
      label.htmlFor = `solution${i}`;
      label.textContent = pathString;

      p.appendChild(radio);
      p.appendChild(label);
      container.appendChild(p);
    });
  }

  private reset() {
    this.selectedSolutionIndex = (this.selectedSolutionIndex + 1) % this.solutions.length;
    this.currentStep = 1;
    this.lineProgress = 0;
    this.ctx.clearRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);

    const selected = document.querySelector<HTMLInputElement>('#solution' + this.selectedSolutionIndex);
    if (!selected) {
      throw new Error("selected solution element is null: #solution" + this.selectedSolutionIndex);
    }
    selected.checked = true;
  }

  updateAndDraw() {
    if (this.solutions.length === 0) return;

    this.lineProgress += this.speed;

    if (this.lineProgress >= 1.0) {
      this.lineProgress = 0.0;
      this.currentStep++;
      if (this.currentStep > 9) {
        this.reset();
      }
    }
    this.ctx.clearRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);
    this.drawCurrentPath();

    requestAnimationFrame(() => this.updateAndDraw());
  }

  private drawCurrentPath() {
    const currentSolution = this.solutions[this.selectedSolutionIndex][0];
    if (!currentSolution) return;

    this.ctx.strokeStyle = "orange";
    this.ctx.lineCap = 'round';
    this.ctx.lineJoin = 'round';
    this.ctx.lineWidth = 4;
    this.ctx.beginPath();
    this.ctx.moveTo(...POSITIONS[currentSolution[0]]);

    // Draw completed segments
    for (let i = 1; i < this.currentStep; i++) {
      if (currentSolution[i] === undefined) break;
      this.ctx.lineTo(...POSITIONS[currentSolution[i]]);
    }

    // Draw the currently animating segment
    const startNodeIndex = currentSolution[this.currentStep - 1];
    const endNodeIndex = currentSolution[this.currentStep];

    if (startNodeIndex !== undefined && endNodeIndex !== undefined) {
      const [x1, y1] = POSITIONS[startNodeIndex];
      const [x2, y2] = POSITIONS[endNodeIndex];
      const newX = x1 + this.lineProgress * (x2 - x1);
      const newY = y1 + this.lineProgress * (y2 - y1);
      this.ctx.lineTo(newX, newY);
    }

    this.ctx.stroke();
  }
}

export type ResultItem = [number[], [number, number][]];
export type Result = ResultItem[];

function extend_paths(current_paths: Result): Result {
  const new_paths: Result = [];

  for (const current_path_item of current_paths) {
    const path_nodes = current_path_item[0];
    const path_edges = current_path_item[1];
    const last_node = path_nodes[path_nodes.length - 1];

    for (let next_node = 0; next_node < MOVES[last_node].length; next_node++) {
      const can_move = MOVES[last_node][next_node] === 1;

      const edge_exists = path_edges.some(
        edge => (edge[0] === last_node && edge[1] === next_node) ||
          (edge[0] === next_node && edge[1] === last_node)
      );

      if (can_move && !edge_exists) {
        const new_path_nodes = [...path_nodes];
        const new_path_edges = [...path_edges];

        new_path_nodes.push(next_node);
        new_path_edges.push([last_node, next_node]);

        new_paths.push([new_path_nodes, new_path_edges]);
      }
    }
  }

  return new_paths;
}

function nikolaus(start: number): Result {
  let result: Result = [];
  for (let i: number = 0; i < MOVES[start].length; i++) {
    if (MOVES[start][i] == 1) {
      let item: ResultItem = [
        [start, i],
        [[start, i], [i, start]]
      ];
      result.push(item);
    }
  }
  for (let i = 0; i < 7; i++) {
    result = extend_paths(result);
  }
  return result;
}

function drawStaticHouse(ctx: CanvasRenderingContext2D) {
  ctx.strokeStyle = "orange";
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
  ctx.lineWidth = 4;
  ctx.beginPath();
  ctx.moveTo(50, 180);
  ctx.lineTo(50, 80);
  ctx.lineTo(150, 180);
  ctx.lineTo(50, 180);
  ctx.lineTo(150, 80);
  ctx.lineTo(50, 80);
  ctx.lineTo(100, 20);
  ctx.lineTo(150, 80);
  ctx.lineTo(150, 180);
  ctx.stroke();

  ctx.strokeStyle = "blue";
  ctx.font = "12px sens";
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.strokeText("0", 30, 185);
  ctx.strokeText("1", 30, 85);
  ctx.strokeText("2", 97, 10);
  ctx.strokeText("3", 162, 85);
  ctx.strokeText("4", 162, 185);
}

async function draw() {
  if (!ctx1 || !canvas1 || !ctx2 || !canvas2) {
    throw new Error("canvas or context is null");
  }

  ctx1.clearRect(0, 0, canvas1.width, canvas1.height);
  drawStaticHouse(ctx1);

  const animator = new SolutionAnimator(ctx2, 0.02, nikolaus(0));
  animator.updateAndDraw();
}

draw();

