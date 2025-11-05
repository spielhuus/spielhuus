const lcs_input_a = document.querySelector<HTMLInputElement>('#lcs_input_a');
const lcs_input_b = document.querySelector<HTMLInputElement>('#lcs_input_b');
const lcs_result = document.querySelector<HTMLInputElement>('#lcs_result');
const lcs_button = document.getElementById('lcs_button') as HTMLButtonElement;
const lcs_canvas_1 = document.getElementById('lcs_canvas_1') as HTMLCanvasElement;
const lcs_ctx_1 = lcs_canvas_1?.getContext('2d');


let rootStyles = getComputedStyle(document.documentElement);
let black = rootStyles.getPropertyValue('--black').trim();
let blue = rootStyles.getPropertyValue('--blue').trim();
let green = rootStyles.getPropertyValue('--green').trim();
let orange = rootStyles.getPropertyValue('--orange').trim();
let grey = rootStyles.getPropertyValue('--grey').trim();

function drawArrow(ctx: CanvasRenderingContext2D, x: number, y: number, length: number, angle: number, headLength = 15, color: string) { 
  const halfLen = length / 2; 
  ctx.save(); 
  ctx.translate(x, y); 
  ctx.rotate(angle); 
  ctx.lineWidth = 1;
  ctx.strokeStyle = color; 
  ctx.beginPath(); 
  ctx.moveTo(-halfLen, 0); 
  ctx.lineTo(halfLen, 0); 
  ctx.lineTo(halfLen - headLength, -headLength / 2); 
  ctx.moveTo(halfLen, 0); 
  ctx.lineTo(halfLen - headLength, headLength / 2); 
  ctx.stroke(); 
  ctx.restore(); 
} 

function drawLcsGrid(ctx: CanvasRenderingContext2D, a: string, b: string, dp?: number[][], path?: [number, number][]) {
  const length_a = a.length + 2;
  const length_b = b.length + 2;
  const cell_size = Math.floor(Math.min(
    lcs_canvas_1.width / length_a,
    lcs_canvas_1.height / length_b,

  ));
  const start_x: number = (lcs_canvas_1.width - cell_size * length_a) / 2;
  const start_y: number = (lcs_canvas_1.height - cell_size * length_b) / 2;
  const text_size = cell_size * 0.4;

  lcs_canvas_1.width = lcs_canvas_1.width; 
  ctx.strokeStyle = black;
  ctx.lineWidth = 1;
  ctx.beginPath();

  for (let i:number = 0; i<length_a; i++) {
    if (i > 1) {
      ctx.fillStyle = blue;
      ctx.font = text_size + 'px Arial';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(a.charAt(i-2), start_x + cell_size * i + cell_size/2, start_y + cell_size/2);
    }
    ctx.moveTo(start_x + i * cell_size + cell_size, start_y + cell_size);
    ctx.lineTo(start_x + i * cell_size + cell_size, start_y + cell_size*length_b);
  }
  for (let i:number = 0; i<length_b; i++) {
    if (i > 1) {
      ctx.fillStyle = green;
      ctx.font = text_size + 'px Arial';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(b.charAt(i-2), start_x + cell_size/2, start_y + cell_size * i + cell_size/2, );
    }
    ctx.moveTo(start_x + cell_size, start_y + i * cell_size + cell_size);
    ctx.lineTo(start_x + cell_size*length_a, start_y + i * cell_size + cell_size);
  }
  ctx.stroke();
  ctx.beginPath();

  if (dp) {
    const arrow_size = 0.4 * cell_size;
    const head_size = 0.1 * cell_size;
    for (let i: number = 0; i<dp.length; i++) {
      for (let j: number = 0; j<dp[i].length; j++) {
        ctx.fillStyle = orange;
        ctx.font = text_size + 'px Arial';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText(dp[i][j].toString(), start_x + ((i + 1) * cell_size) + cell_size/2, start_y + ((j + 1) * cell_size) + cell_size/2);
        if (i > 0 && j > 0) {
          const act_val = dp[i][j];
          let color = grey;
          if (path && path.some(point => point[0] === i && point[1] === j)) {
            color = black;
          }
          if (dp[i - 1][j] < act_val && dp[i][j - 1] < act_val && dp[i - 1][j - 1] < act_val) {
            drawArrow(ctx, start_x + ((i + 1) * cell_size), start_y + ((j + 1) * cell_size), arrow_size, 1.25 * 3.14, head_size, color);
          } else if (dp[i][j - 1] === act_val && dp[i - 1][j - 1] <= act_val && dp[i - 1][j] <= act_val) {
            //up
            drawArrow(ctx, start_x + ((i + 1) * cell_size) + cell_size/2, start_y + ((j + 1) * cell_size), arrow_size, 1.5 * 3.14, head_size, color);
          } else {
            // left
            drawArrow(ctx, start_x + ((i + 1) * cell_size), start_y + ((j + 1) * cell_size) + cell_size/2, arrow_size, 3.14, head_size, color);
          }

        }
      }
    }
  }
  ctx.stroke();
}

function lcs(left: string, right: string): number[][] {
  const len1 = left.length;
  const len2 = right.length;
  const dp: number[][] = Array(len1 + 1)
  .fill(null)
  .map(() => Array(len2 + 1).fill(0));

  for (let i = 1; i <= len1; i++) {
    for (let j = 1; j <= len2; j++) {
      if (i == 0 || j == 0) {
        dp[i][j] = 0;
      } else if (left[i - 1] === right[j - 1]) {
        dp[i][j] = dp[i - 1][j - 1] + 1;
      } else {
        dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
      }
    }
  }
  return dp
}

function lcs_backtrack(a: string, b: string, dp: number[][]): [number, number][] {
  let path: [number, number][] = [];
  let i = dp.length - 1, j = dp[dp.length-1].length - 1;
  path.push([i, j])
  while (i > 0 && j > 0) {

    path.push([i, j]);
    // If characters match, the path is diagonal
    if (a[i - 1] === b[j - 1]) {
      i--;
      j--;
    }
    // If characters don't match, move to the cell with the larger value
    else if (dp[i - 1][j] > dp[i][j - 1]) {
      i--;
    } else {
      j--;
    }
  }
  path.push([i, j]);
  return path;
}

function lcs_get_result(a: string, b: string, dp: number[][]): string {
  let path = [];
  let i = dp.length - 1, j = dp[dp.length-1].length - 1;
  while (i > 0 && j > 0) {
    if (a.charAt(i - 1) == b.charAt(j - 1)) {
      path.push(a.charAt(i - 1));
      i--;
      j--;
    } else if (dp[i - 1][j] > dp[i][j - 1]) {
      i--;
    } else {
      j--;
    }
  }
  return path.join('');
}

function draw() {
  if (lcs_button && lcs_input_a && lcs_input_b && lcs_ctx_1) {
    let lcs_path = lcs(lcs_input_a.value, lcs_input_b.value);
    let result = lcs_get_result(lcs_input_a.value, lcs_input_b.value, lcs_path);
    if (lcs_result) {
      lcs_result.value = result;
    }
    drawLcsGrid(lcs_ctx_1, lcs_input_a.value, lcs_input_b.value, lcs_path, lcs_backtrack(lcs_input_a.value, lcs_input_b.value, lcs_path))
  }
}

async function main() {
  if (lcs_button && lcs_input_a && lcs_input_b && lcs_ctx_1) {
    const handleClick = (_: MouseEvent) => {
      draw();
    };
    lcs_button.addEventListener('click', handleClick);

    // @ts-ignore
    window.themeController.subscribe(() => {
      black = rootStyles.getPropertyValue('--black').trim();
      blue = rootStyles.getPropertyValue('--blue').trim();
      green = rootStyles.getPropertyValue('--green').trim();
      orange = rootStyles.getPropertyValue('--orange').trim();
      grey = rootStyles.getPropertyValue('--grey').trim();
      draw();
    });
    drawLcsGrid(lcs_ctx_1, lcs_input_a.value, lcs_input_b.value)

  } else {
    console.error('Button with ID "elements" not found.');
  }
}

main();

