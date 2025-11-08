
enum DiffType {
  Add,
  Delete,
  NoChange,
}

type DiffItem = {
  change: DiffType;
  position_a: number;
  position_b: number;
  content: string,
};

class DiffStep {
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D;

  black: string;
  blue: string;
  green: string;
  orange: string;
  grey: string;

  cell_size: number;
  start_x: number;
  start_y: number;
  state = 0;

  constructor(public left: string[], public right: string[], id: string) {

    const canvas = document.getElementById(id) as HTMLCanvasElement;
    const ctx = canvas?.getContext('2d');
    if (!ctx) {
      throw new Error("Context not found with"); 
    }

    let rootStyles = getComputedStyle(document.documentElement);
    this.black = rootStyles.getPropertyValue('--black').trim();
    this.blue = rootStyles.getPropertyValue('--blue').trim();
    this.green = rootStyles.getPropertyValue('--green').trim();
    this.orange = rootStyles.getPropertyValue('--orange').trim();
    this.grey = rootStyles.getPropertyValue('--grey').trim();

    this.canvas = canvas;
    this.ctx = ctx;

    const length_a = left.length + 1;
    const length_b = right.length + 1;
    this.cell_size = Math.floor(Math.min(
      (this.canvas.width -20) / length_a,
      (this.canvas.height - 20) / length_b,

    ));
    this.start_x = (this.canvas.width - 20 - this.cell_size * length_a) / 2;
    this.start_y = (this.canvas.height - 20 - this.cell_size * length_b) / 2;
  }

  public *path_step() {
    this.state = 1;
    const n = this.left.length;
    const m = this.right.length;
    const max = n + m;
    const offset = max;

    const v = new Array(2 * max + 1).fill(-1);
    v[1 + offset] = 0;

    const trace = [];

    for (let d = 0; d <= max; d++) {
      trace.push(v.slice());

      for (let k = -d; k <= d; k += 2) {
        let x: number;
        let prev_x: number;
        let prev_y: number;
        if (k === -d || (k !== d && v[k + offset - 1] < v[k + offset + 1])) {
          // Move down (insertion), from k+1 diagonal
          prev_x = v[k + offset + 1];
          prev_y = prev_x - (k + 1);
          x = v[k + offset + 1];
        } else {
          // Move right (deletion), from k-1 diagonal
          prev_x = v[k + offset - 1];
          prev_y = prev_x - (k - 1);
          x = v[k + offset - 1] + 1;
        }

        let y = x - k;
        this.drawLine(prev_x, prev_y, x, y, this.black);

        // Follow the "snake" of matches
        while (x < n && y < m && this.left[x] === this.right[y]) {
          this.drawLine(x, y, x+1, y+1, this.black);
          x++;
          y++;
        }

        v[k + offset] = x;

        if (x >= n && y >= m) {
          // Add the final state to the trace before returning
          trace.push(v.slice());
          this.state = 2;
          return trace;
        }
        if (d >= 1) { 
          yield;
        }
      }
    }
    throw Error("shortest edit did not properly terminate");
  }

  public *backtrack_step(trace: number[][], step: boolean) {
    const n = this.left.length;
    const m = this.right.length;
    const max = n + m;
    const offset = max;
    let result: DiffItem[] = [];
    let x = this.left.length;
    let y = this.right.length;
    let prev_k, prev_y, edit_type;
    //iterate the trace from the end
    for (let d = trace.length - 2; d >= 0; d--) {
      const v = trace[d];
      const k = x - y;
      if (k == -d || (k != d && v[k + offset - 1] < v[k + offset + 1])) {
        prev_k = k + 1
        edit_type = DiffType.Add;
      } else {
        prev_k = k - 1
        edit_type = DiffType.Delete;
      }

      let prev_x = v[prev_k + offset];
      prev_y = prev_x - prev_k;

      while (x > prev_x && y > prev_y) {
        result.push({
          change: DiffType.NoChange,
          position_a: x,
          position_b: y,
          content: this.left[x - 1],
        });
        this.drawLine(x, y, x-1, y-1, this.orange);
        x = x - 1;
        y = y - 1;
      }

      if (d > 0) {
        if (edit_type === DiffType.Add) {
          result.push({
            change: DiffType.Add,
            position_a: 0,
            position_b: y,
            content: this.right[y - 1],
          });
        } else { // DiffType.Delete
          result.push({
            change: DiffType.Delete,
            position_a: x,
            position_b: 0,
            content: this.left[x - 1],
          });
        }
      }

      this.drawLine(x, y, prev_x, prev_y, this.orange);
      x = prev_x;
      y = prev_y;

      if (x === 0 && y === 0) break; // Terminate when we reach the origin 
      if (step) yield;
    }

    return result;
  }

  private drawLine(x1: number, y1: number, x2: number, y2: number, color: string) {
    if (x1 >= 0 && y1 >= 0 && x1 <= this.left.length /* && y1  <= this.right.length */ &&
        x2 >= 0 && y2 >= 0 && x2 <= this.left.length /* && y2  <= this.right.length */ ) {
      this.ctx.strokeStyle = color;
      this.ctx.lineWidth = 2;
      this.ctx.beginPath();
      this.ctx.moveTo(this.start_x + (x1 + 1) * this.cell_size, this.start_y + (y1 + 1) * this.cell_size);
      this.ctx.lineTo(this.start_x + (x2 + 1) * this.cell_size, this.start_y + (y2 + 1) * this.cell_size);
      this.ctx.stroke();
    }
  }

}

class DiffDraw {
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D;

  black: string;
  blue: string;
  green: string;
  orange: string;
  grey: string;

  cell_size: number;
  start_x: number;
  start_y: number;

  constructor(private left: string, private right: string, id: string) {
    const canvas = document.getElementById(id) as HTMLCanvasElement;
    const ctx = canvas?.getContext('2d');
    if (!ctx) {
      throw new Error("Context not found with"); 
    }

    let rootStyles = getComputedStyle(document.documentElement);
    this.black = rootStyles.getPropertyValue('--black').trim();
    this.blue = rootStyles.getPropertyValue('--blue').trim();
    this.green = rootStyles.getPropertyValue('--green').trim();
    this.orange = rootStyles.getPropertyValue('--orange').trim();
    this.grey = rootStyles.getPropertyValue('--grey').trim();

    this.canvas = canvas;
    this.ctx = ctx;

    const length_a = left.length + 1;
    const length_b = right.length + 1;
    this.cell_size = Math.floor(Math.min(
      (this.canvas.width -20) / length_a,
      (this.canvas.height - 20) / length_b,

    ));
    this.start_x = (this.canvas.width - 20 - this.cell_size * length_a) / 2;
    this.start_y = (this.canvas.height - 20 - this.cell_size * length_b) / 2;
  }


  public grid() {
    const length_a = this.left.length + 1;
    const length_b = this.right.length + 1;
    const text_size = this.cell_size * 0.4;

    this.canvas.width = this.canvas.width; 
    this.ctx.strokeStyle = this.black;
    this.ctx.lineWidth = 1;
    this.ctx.beginPath();

    for (let i:number = 0; i<length_a; i++) {
      if (i > 0) {
        this.ctx.fillStyle = this.blue;
        this.ctx.font = text_size + 'px Arial';
        this.ctx.textAlign = 'center';
        this.ctx.textBaseline = 'middle';
        this.ctx.fillText(this.left[i - 1], this.start_x + this.cell_size * i + this.cell_size/2, this.start_y + this.cell_size/2);
      }
      this.ctx.moveTo(this.start_x + i * this.cell_size + this.cell_size, this.start_y + this.cell_size);
      this.ctx.lineTo(this.start_x + i * this.cell_size + this.cell_size, this.start_y + this.cell_size*length_b);
    }
    for (let i:number = 0; i<length_b; i++) {
      if (i > 0) {
        this.ctx.fillStyle = this.green;
        this.ctx.font = text_size + 'px Arial';
        this.ctx.textAlign = 'center';
        this.ctx.textBaseline = 'middle';
        this.ctx.fillText(this.right[i - 1], this.start_x + this.cell_size/2, this.start_y + this.cell_size * i + this.cell_size/2, );
      }
      this.ctx.moveTo(this.start_x + this.cell_size, this.start_y + i * this.cell_size + this.cell_size);
      this.ctx.lineTo(this.start_x + this.cell_size*length_a, this.start_y + i * this.cell_size + this.cell_size);
    }
    this.ctx.stroke();

    this.ctx.beginPath();
    for (let i = 0; i < this.left.length; i++) {
      for (let j = 0; j < this.right.length; j++) {
        if (this.left[i] === this.right[j]) {
          this.ctx.moveTo(this.start_x + (i + 1) * this.cell_size, this.start_y + (j + 1) * this.cell_size);
          this.ctx.lineTo(this.start_x + (i + 1) * this.cell_size + this.cell_size, this.start_y + (j + 1) * this.cell_size + this.cell_size);
        }
      }
    }
    this.ctx.stroke();
  }
}

class Diff {
  trace: number[][];

  constructor(public left: string[], public right: string[]) {
    this.trace = this.shortest_edit();
  };

  private shortest_edit(): number[][] {
    const n = this.left.length;
    const m = this.right.length;
    const max = n + m;
    const offset = max;

    const v = new Array(2 * max + 1).fill(-1);
    v[1 + offset] = 0;

    const trace = [];

    for (let d = 0; d <= max; d++) {
      trace.push(v.slice());

      for (let k = -d; k <= d; k += 2) {
        let x: number;
        if (k === -d || (k !== d && v[k + offset - 1] < v[k + offset + 1])) {
          // Move down (insertion), from k+1 diagonal
          x = v[k + offset + 1];
        } else {
          // Move right (deletion), from k-1 diagonal
          x = v[k + offset - 1] + 1;
        }

        let y = x - k;

        // Follow the "snake" of matches
        while (x < n && y < m && this.left[x] === this.right[y]) {
          x++;
          y++;
        }

        v[k + offset] = x;

        if (x >= n && y >= m) {
          // Add the final state to the trace before returning
          trace.push(v.slice());
          console.log("result: ", trace)
          return trace;
        }
      }
    }
    throw Error("shortest edit did not properly terminate");
  }


  public diff(): DiffItem[] {
    const n = this.left.length;
    const m = this.right.length;
    const max = n + m;
    const offset = max;
    let result: DiffItem[] = [];
    let x = this.left.length;
    let y = this.right.length;
    let prev_k, prev_y, edit_type;
    //iterate the trace from the end
    for (let d = this.trace.length - 2; d >= 0; d--) {
      const v = this.trace[d];
      const k = x - y;
      if (k == -d || (k != d && v[k + offset - 1] < v[k + offset + 1])) {
        prev_k = k + 1
        edit_type = DiffType.Add;
      } else {
        prev_k = k - 1
        edit_type = DiffType.Delete;
      }

      let prev_x = v[prev_k + offset];
      prev_y = prev_x - prev_k;

      while (x > prev_x && y > prev_y) {
        result.push({
          change: DiffType.NoChange,
          position_a: x,
          position_b: y,
          content: this.left[x - 1],
        });
        x = x - 1;
        y = y - 1;
      }

      if (d > 0) {
        if (edit_type === DiffType.Add) {
          result.push({
            change: DiffType.Add,
            position_a: 0,
            position_b: y,
            content: this.right[y - 1],
          });
        } else { // DiffType.Delete
          result.push({
            change: DiffType.Delete,
            position_a: x,
            position_b: 0,
            content: this.left[x - 1],
          });
        }
      }

      x = prev_x;
      y = prev_y;

      if (x === 0 && y === 0) break; // Terminate when we reach the origin 
    }

    return result;
  }
}

const diff_input_a = document.querySelector<HTMLInputElement>('#diff_input_a');
const diff_input_b = document.querySelector<HTMLInputElement>('#diff_input_b');
const source_code_input_a2 = document.querySelector<HTMLInputElement>('#source_code_input_a2');
const source_code_input_b2 = document.querySelector<HTMLInputElement>('#source_code_input_b2');


let diff: Diff

  async function main() {
  if (diff_input_a && diff_input_b) {

    // draw the empty grid
    let grid = new DiffDraw(diff_input_a.value, diff_input_b.value, "diff_grid")
    const step_button = document.getElementById('step_button') as HTMLButtonElement;
    let diff_step = new DiffStep([...diff_input_a.value], [...diff_input_b.value], "diff_grid");
    let diff_iterator: Generator<void, number[][], void>|undefined;
    let backtrack_iterator: Generator<void, DiffItem[], void>|undefined;
    // grid.grid();

    function drawTable(diff_data: DiffItem[], id: string) {
          const table = document.getElementById(id) as HTMLTableElement;
          const tableBody = table.getElementsByTagName('tbody')[0];
          tableBody.innerHTML = '';
          [...diff_data].reverse().forEach(item => {
            const row = tableBody.insertRow(); 
            const changeCell = row.insertCell(0);
            const pos1Cell = row.insertCell(1);
            const pos2Cell = row.insertCell(2);
            const contentCell = row.insertCell(3);

            let changeText;
            if (item.change == DiffType.Add) {
              changeText = "+";
              row.classList.add("diff-add");
            } else if (item.change == DiffType.Delete) {
              changeText = "-";
              row.classList.add("diff-delete");
            } else { 
              changeText = " ";
            }
            changeCell.textContent = changeText;
            pos1Cell.textContent = item.position_a?.toString();
            pos2Cell.textContent = item.position_b?.toString();
            contentCell.textContent = item.content;
          });
    }

    function reset() {
      if (diff_input_a && diff_input_b) {
        // draw the diff
        diff = new Diff([...diff_input_a.value], [...diff_input_b.value]);
        drawTable(diff.diff(), 'diff-table');
        grid = new DiffDraw(diff_input_a.value, diff_input_b.value, "diff_grid")
        diff_iterator = undefined;
        diff_step = new DiffStep([...diff_input_a.value], [...diff_input_b.value], "diff_grid");
        backtrack_iterator = undefined;
        grid.grid();
      } else {
        console.error("input fields are not defined.");
      }
    }

    reset();

    //subscribe for events when the content changes.
    function onContentChange(_: Event) { 
      reset();
    } 
     
    ([diff_input_a, diff_input_b] as const).forEach((el) => { 
      if (!el) return; 
      el.addEventListener('input', onContentChange, { passive: true });
      el.addEventListener('change', onContentChange);
    }); 

    //subscribe to the step button
    let trace: number[][];
    const handleStep = (_: MouseEvent) => {
      if (step_button) {
        if (diff_iterator === undefined) {
          diff_iterator = diff_step.path_step();
          diff_step.state = 1;
        } else if (diff_step.state == 1) {
          let result = diff_iterator.next();
          if (result.done) {
            trace = result.value;
          }
        } else if (diff_step.state == 2) {
          backtrack_iterator = diff_step.backtrack_step(trace, true);
          diff_step.state = 3;
        } else if (diff_step.state == 3) {
          if (backtrack_iterator) {
          let result = backtrack_iterator.next();
          if (result.done) {
            diff_step.state = 4;
          }
          } else {
            console.error("backtrack_iterator is not defined!");
          }
        } else if (diff_step.state == 4) {
          grid.grid();
          backtrack_iterator = diff_step.backtrack_step(trace, false);
          backtrack_iterator.next();
          diff_step.state = 5;
        }
      }
    };
    step_button.addEventListener('click', handleStep);


    //handle source code
    function updateSourceDiff() {
      if (source_code_input_a2 && source_code_input_b2) {
        let diff = new Diff(source_code_input_a2.value.split(/\r?\n/), source_code_input_b2.value.split(/\r?\n/));
        drawTable(diff.diff(), 'diffSourceData2');
      } else {
        console.error("source code input not found!");
      }
    }

    function onSourceChange(_: Event) { 
      updateSourceDiff();
    } 
     
    ([source_code_input_a2, source_code_input_b2] as const).forEach((el) => { 
      if (!el) return; 
      el.addEventListener('input', onSourceChange, { passive: true });
      el.addEventListener('change', onSourceChange);
    }); 

    updateSourceDiff();

    // @ts-ignore
    window.themeController.subscribe(() => {
      reset();
      updateSourceDiff();
    });
} else {
  console.error('Button with ID "elements" not found.');
}
  }
  main();
