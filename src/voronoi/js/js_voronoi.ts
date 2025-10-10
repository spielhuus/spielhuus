import { Vector2 } from '../../js/vector';

const SCREEN_WIDTH = 800;
const SCREEN_HEIGHT = 600;

export const PASTEL_PALETTE: number[][] = [
  // Reds & Terracottas 
  [0xD0, 0xA0, 0xA5], // 1. Dusty Rose 
  [0xC8, 0x7E, 0x6F], // 2. Terracotta 
  [0xB0, 0x6B, 0x5C], // 3. Cinnamon Stick 
  [0x9B, 0x5B, 0x5F], // 4. Faded Garnet 
  [0xC5, 0x8C, 0x79], // 5. Muted Clay 
  // Oranges & Yellows 
  [0xDD, 0xAA, 0x55], // 6. Golden Ochre 
  [0xC7, 0xA2, 0x50], // 7. Spiced Mustard 
  [0xD1, 0x84, 0x60], // 8. Burnt Sienna 
  [0xEA, 0xA2, 0x43], // 9. Marigold 
  [0xC8, 0x8A, 0x3B], // 10. Amber Glow 
  // Greens 
  [0x7A, 0x8D, 0x7B], // 11. Deep Sage 
  [0x6A, 0x70, 0x49], // 12. Olive Drab 
  [0x83, 0x8A, 0x73], // 13. Mossy Stone 
  [0x5A, 0x63, 0x49], // 14. Forest Floor 
  [0x4A, 0x7C, 0x82], // 15. Muted Teal 
  // Blues 
  [0x6A, 0x82, 0x9A], // 16. Slate Blue 
  [0x6C, 0x85, 0xA1], // 17. Faded Denim 
  [0x5E, 0x74, 0x85], // 18. Stormy Sky 
  [0x7E, 0x88, 0xB0], // 19. Deep Periwinkle 
  [0x8D, 0x9B, 0xA6], // 20. Coastal Fog 
  // Purples & Mauves 
  [0x9A, 0x8B, 0x9E], // 21. Dusty Heather 
  [0x7D, 0x5A, 0x7D], // 22. Faded Plum 
  [0xA3, 0x86, 0x9C], // 23. Smoky Mauve 
  [0x78, 0x66, 0x84], // 24. Withered Iris 
  [0x82, 0x6D, 0x8C], // 25. Vintage Violet 
  // Neutrals & Browns 
  [0xA8, 0x98, 0x8A], // 26. Warm Taupe 
  [0x8C, 0x8C, 0x8C], // 27. Stonewash 
  [0x6E, 0x5A, 0x4F], // 28. Rich Umber 
  [0x5A, 0x5A, 0x5A], // 29. Charcoal Dust 
  [0xB4, 0xAF, 0xAF], // 30. Pebble Path 
];

export class JsVoronoi {
  seed_count = PASTEL_PALETTE.length - 1;
  seeds: Array<Vector2> = [];
  distance = 1;
  buffer: number[][] = [];
  constructor(private canvas: HTMLCanvasElement, private ctx: CanvasRenderingContext2D) {
    for (let y = 0; y < SCREEN_HEIGHT; y++) {
      // Create a new row (an array)
      const row = new Array(SCREEN_WIDTH).fill(-1);
      // Push it into our main array
      this.buffer.push(row);
    }
    for (let i = 0; i < this.seed_count; i++) {
      const x = Math.floor(Math.random() * canvas.width);
      const y = Math.floor(Math.random() * canvas.height);
      this.seeds.push(new Vector2(x, y));
    }
  }

  private neighbours(pos: Vector2, distance: number): Array<Vector2> {
    let n = []
    for (let y = -distance; y <= distance; y++) {
      for (let x = -distance; x <= distance; x++) {
        if (!(y === 0 && x === 0)) {
          if (pos.x + x >= 0 && pos.x + x < SCREEN_WIDTH && pos.y + y >= 0 && pos.y + y < SCREEN_HEIGHT) {
          n.push(new Vector2(pos.x + x, pos.y + y));
        }
        }
      }
    }
    return n;
  }
  public draw() {
    let found = 0;
    for (let i = 0; i < this.seed_count; i++) {
      // this.ctx.fillStyle = "#FF0000";
      // this.ctx.beginPath();
      // this.ctx.arc(...this.seeds[i].array(), 4, 0, 2 * Math.PI);
      // this.ctx.fill();

      for (let n of this.neighbours(this.seeds[i], this.distance)) {
        if (this.buffer[n.y][n.x] === -1) {
          this.buffer[n.y][n.x] = i;
          found += 1;
          
          const color = PASTEL_PALETTE[i % PASTEL_PALETTE.length];
          this.ctx.fillStyle = `rgb(${color[0]}, ${color[1]}, ${color[2]})`;
          this.ctx.fillRect(n.x, n.y, 1, 1);
        }
      }
    }
    // draw the result
    // let imageData = this.ctx.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT); //TODO
    // const data = imageData.data; // Type: Uint8ClampedArray
    // for (let x = 0; x < SCREEN_WIDTH; x++) {
    //   for (let y = 0; y < SCREEN_HEIGHT; y++) {
    //     if (this.buffer[y][x] >= 0) {
    //       const index = (y * SCREEN_WIDTH + x) * 4;
    //       data[index] = PASTEL_PALETTE[this.buffer[y][x]][0];
    //       data[index + 1] = PASTEL_PALETTE[this.buffer[y][x]][1];
    //       data[index + 2] = PASTEL_PALETTE[this.buffer[y][x]][2];
    //       data[index + 3] = 255;
    //     }
    //   }
    // }

    // this.ctx.putImageData(imageData, 0, 0);
    for (let i = 0; i < this.seed_count; i++) {
      this.ctx.fillStyle = "#FF0000";
      this.ctx.beginPath();
      this.ctx.arc(...this.seeds[i].array(), 4, 0, 2 * Math.PI);
      this.ctx.fill();
    }
    // and redraw
    this.distance += 1;
    if (found > 0 ) {
      requestAnimationFrame(() => { this.draw() });
    } else {
      console.log("DONE");
    }
  }
}
