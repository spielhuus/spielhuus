
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

class Diff {
  trace: number[][];
  cell_d_k: Array<Array<[number, number]>>;

  constructor(public left: string[], public right: string[]) {
    const n = this.left.length; 
    const m = this.right.length; 
    this.cell_d_k = Array.from( 
                               { length: n + 1 },  
                               () => new Array(m + 1) 
                              ); 
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
        if (x >= 0 && y >= 0 && x <= n && y <= m) { 
          this.cell_d_k[x][y] = [d, k]; 
        } 

        // Follow the "snake" of matches
        while (x < n && y < m && this.left[x] === this.right[y]) {
          x++;
          y++;

          this.cell_d_k[x][y] = [d, k]; 
        }

        v[k + offset] = x;

        if (x >= n && y >= m) {
          // Add the final state to the trace before returning
          trace.push(v.slice());
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


class DiffGraph {
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

  constructor(private diff: Diff) {
    const canvas = document.getElementById('diff_canvas_1') as HTMLCanvasElement;
    const ctx = canvas?.getContext('2d');
    if (!ctx) {
      throw new Error("Context not found"); 
    }

    let rootStyles = getComputedStyle(document.documentElement);
    this.black = rootStyles.getPropertyValue('--black').trim();
    this.blue = rootStyles.getPropertyValue('--blue').trim();
    this.green = rootStyles.getPropertyValue('--green').trim();
    this.orange = rootStyles.getPropertyValue('--orange').trim();
    this.grey = rootStyles.getPropertyValue('--grey').trim();

    this.canvas = canvas;
    this.ctx = ctx;

    const length_a = diff.left.length + 1;
    const length_b = diff.right.length + 1;
    this.cell_size = Math.floor(Math.min(
      (this.canvas.width -20) / length_a,
      (this.canvas.height - 20) / length_b,

    ));
    this.start_x = (this.canvas.width - 20 - this.cell_size * length_a) / 2;
    this.start_y = (this.canvas.height - 20 - this.cell_size * length_b) / 2;
  }

  private drawWeights() {
    for (let x = 0; x<this.diff.cell_d_k.length; x++) {
      for (let y = 0; y<this.diff.cell_d_k[x].length; y++) {
        if (this.diff.cell_d_k[x][y]) {
          this.ctx.beginPath();
          this.ctx.fillStyle = this.grey;
          this.ctx.strokeStyle = this.grey;
          this.ctx.font = (this.cell_size * 0.2) + 'px Arial';
          this.ctx.textAlign = 'left';
          this.ctx.textBaseline = 'top';
          this.ctx.fillText(
            this.diff.cell_d_k[x][y][0].toString() + "/" + this.diff.cell_d_k[x][y][1].toString(), 
            this.start_x + this.cell_size * (x + 1) + this.cell_size / 8,
            this.start_y + this.cell_size * (y + 1) + this.cell_size / 8);
          this.ctx.stroke();
          this.ctx.beginPath();
          this.ctx.arc(this.start_x + this.cell_size * (x + 1), this.start_y + this.cell_size * (y + 1), 2, 0, 2*3.14);
          this.ctx.fill();
        }
      }
    }
  }

  private drawPath() {
    const n = this.diff.left.length;
    const m = this.diff.right.length;
    const max = n + m;
    const offset = max;
    let result = [];
    let x = this.diff.left.length;
    let y = this.diff.right.length;
    let prev_k, prev_y;
    //iterate the trace from the end
    for (let d = this.diff.trace.length - 2; d > 0; d--) {
      const v = this.diff.trace[d];
      const k = x - y;
      if (k == -d || (k != d && v[k + offset - 1] < v[k + offset + 1])) {
        prev_k = k + 1
      } else {
        prev_k = k - 1
      }

      let prev_x = v[prev_k + offset];
      prev_y = prev_x - prev_k;

      while (x > prev_x && y > prev_y) {
        result.push([x - 1, y - 1, x, y]);
        x = x - 1;
        y = y - 1;
      }

      if (d > 0) {
        result.push([prev_x, prev_y, x, y]);
      }
      x = prev_x;
      y = prev_y;
    }

    this.ctx.fillStyle = this.orange;
    this.ctx.strokeStyle = this.orange;
    this.ctx.beginPath();
    this.ctx.moveTo(Math.floor(this.start_x + (this.diff.left.length + 1) * this.cell_size), Math.floor(this.start_y + (this.diff.right.length + 1) * this.cell_size));
    for (let i = 0; i < result.length; i++) {
      const x1 = result[i][0];
      const y1 = result[i][1];
      this.ctx.lineTo(Math.floor(this.start_x + (x1 + 1) * this.cell_size), Math.floor(this.start_y + (y1 + 1) * this.cell_size));
      this.ctx.stroke();
    }
    this.ctx.stroke();

    this.ctx.beginPath();
    this.ctx.arc(Math.floor(this.start_x + (this.diff.left.length + 1) * this.cell_size), Math.floor(this.start_y + (this.diff.right.length + 1) * this.cell_size), 4, 0, 2*3.14);
    this.ctx.fill();
    for (let i = 0; i < result.length; i++) {
      const x1 = result[i][0];
      const y1 = result[i][1];
      this.ctx.beginPath();
      this.ctx.arc(Math.floor(this.start_x + this.cell_size * (x1 + 1)), Math.floor(this.start_y + this.cell_size * (y1 + 1)), 4, 0, 2*3.14);
      this.ctx.fill();
    }
  }

  private drawGrid() {
    const length_a = this.diff.left.length + 1;
    const length_b = this.diff.right.length + 1;
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
        this.ctx.fillText(this.diff.left[i - 1], this.start_x + this.cell_size * i + this.cell_size/2, this.start_y + this.cell_size/2);
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
        this.ctx.fillText(this.diff.right[i - 1], this.start_x + this.cell_size/2, this.start_y + this.cell_size * i + this.cell_size/2, );
      }
      this.ctx.moveTo(this.start_x + this.cell_size, this.start_y + i * this.cell_size + this.cell_size);
      this.ctx.lineTo(this.start_x + this.cell_size*length_a, this.start_y + i * this.cell_size + this.cell_size);
    }
    this.ctx.stroke();

    this.ctx.beginPath();
    for (let i = 0; i < this.diff.left.length; i++) {
      for (let j = 0; j < this.diff.right.length; j++) {
        if (this.diff.left[i] === this.diff.right[j]) {
          this.ctx.moveTo(this.start_x + (i + 1) * this.cell_size, this.start_y + (j + 1) * this.cell_size);
          this.ctx.lineTo(this.start_x + (i + 1) * this.cell_size + this.cell_size, this.start_y + (j + 1) * this.cell_size + this.cell_size);
        }
      }
    }
    this.ctx.stroke();
  }

  public draw() {
    let rootStyles = getComputedStyle(document.documentElement);
    this.black = rootStyles.getPropertyValue('--black').trim();
    this.blue = rootStyles.getPropertyValue('--blue').trim();
    this.green = rootStyles.getPropertyValue('--green').trim();
    this.orange = rootStyles.getPropertyValue('--orange').trim();
    this.grey = rootStyles.getPropertyValue('--grey').trim();

    this.drawGrid();
    let diff_data:DiffItem[]  = this.diff.diff();
    if (diff_data) {
      const table = document.getElementById('diffData') as HTMLTableElement;
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
    this.drawWeights();
    this.drawPath();
  }
}

const lcs_button = document.getElementById('diff_button') as HTMLButtonElement;
const lcs_input_a = document.querySelector<HTMLInputElement>('#diff_input_a');
const lcs_input_b = document.querySelector<HTMLInputElement>('#diff_input_b');
const source_code_button2 = document.getElementById('source_code_button2') as HTMLButtonElement;
const source_code_input_a2 = document.querySelector<HTMLInputElement>('#source_code_input_a2');
const source_code_input_b2 = document.querySelector<HTMLInputElement>('#source_code_input_b2');

let diff: Diff, drawer: DiffGraph;
let source_diff2: Diff;
if (lcs_input_a && lcs_input_b && lcs_button) {
  async function main() {
    const handleClick = (_: MouseEvent) => {
      if (lcs_input_a && lcs_input_b && lcs_button) {
        diff = new Diff([...lcs_input_a.value], [...lcs_input_b.value]);
        drawer = new DiffGraph(diff);
        drawer.draw();
      }
    };
    lcs_button.addEventListener('click', handleClick);
    // @ts-ignore
    window.themeController.subscribe(() => {
      if (drawer) { drawer.draw(); }
    });

    //handle source code
    const handleSourceClick2 = (_: MouseEvent) => {
      if (source_code_input_a2 && source_code_input_b2 && source_code_button2) {
        source_diff2 = new Diff(source_code_input_a2.value.split(/\r?\n/), source_code_input_b2.value.split(/\r?\n/));
        let diff_data:DiffItem[]  = source_diff2.diff();
        if (diff_data) {
          const table = document.getElementById('diffSourceData2') as HTMLTableElement;
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
      }
    };
    source_code_button2.addEventListener('click', handleSourceClick2);
    // @ts-ignore
    window.themeController.subscribe(() => {
      if (drawer) { drawer.draw(); }
    });
  }
  main();
} else {
  console.error('Button with ID "elements" not found.');
}
