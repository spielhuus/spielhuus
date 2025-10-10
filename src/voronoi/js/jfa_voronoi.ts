import { Vector2 } from '../../js/vector';

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

export class JfaVoronoi {
  seed_count = PASTEL_PALETTE.length - 1;
  seeds: Array<Vector2> = [];
  private readGrid: (Vector2 | null)[][] = [];
  private writeGrid: (Vector2 | null)[][] = [];
  private seedToColorIndex: Map<string, number> = new Map();
  private step: number;

  constructor(private canvas: HTMLCanvasElement, private ctx: CanvasRenderingContext2D) {
    this.step = Math.floor(Math.max(canvas.width, canvas.height) / 2);
    for (let y = 0; y < canvas.height; y++) {
      this.readGrid.push(new Array(canvas.width).fill(null));
      this.writeGrid.push(new Array(canvas.width).fill(null));
    }

    for (let i = 0; i < this.seed_count; i++) {
      const x = Math.floor(Math.random() * canvas.width);
      const y = Math.floor(Math.random() * canvas.height);
      let seed = new Vector2(x, y);
      this.seeds.push(seed);
      if (this.readGrid[y][x] === null) {
        this.readGrid[y][x] = seed;
        this.seedToColorIndex.set(`${x},${y}`, i);
      }
    }
  }

  private floodStep(x: number, y: number, step: number) {
    const currentPixelPos = new Vector2(x, y);
    let bestSeed = this.readGrid[y][x];
    // Use squared distance to avoid expensive sqrt() calls
    let minDisSq = bestSeed ? currentPixelPos.distanceToSquared(bestSeed) : Infinity;

    for (let j = -1; j <= 1; j++) {
      for (let i = -1; i <= 1; i++) {
        const nx = x + i * step;
        const ny = y + j * step;

        // Ensure the neighbor is within bounds
        if (nx >= 0 && nx < this.canvas.width && ny >= 0 && ny < this.canvas.height) {
          const candidateSeed = this.readGrid[ny][nx];

          if (candidateSeed) {
            const disSq = currentPixelPos.distanceToSquared(candidateSeed);
            if (disSq < minDisSq) {
              minDisSq = disSq;
              bestSeed = candidateSeed;
            }
          }
        }
      }
    }
    this.writeGrid[y][x] = bestSeed;
  }

  public draw() {

    // flood next step
    if (this.step >= 1) {
      // For every pixel...
      for (let y = 0; y < this.canvas.height; y++) {
        for (let x = 0; x < this.canvas.width; x++) {
          this.floodStep(x, y, this.step);
        }
      }

      [this.readGrid, this.writeGrid] = [this.writeGrid, this.readGrid];
      this.step = Math.floor(this.step / 2);
    }
    
    const imageData = this.ctx.createImageData(this.canvas.width, this.canvas.height);
    const data = imageData.data;

    for (let y = 0; y < this.canvas.height; y++) {
      for (let x = 0; x < this.canvas.width; x++) {
        const closestSeed = this.readGrid[y][x];

        if (closestSeed) {
          // Look up the color index using our map
          const colorIndex = this.seedToColorIndex.get(`${closestSeed.x},${closestSeed.y}`);
          if (colorIndex !== undefined) {
            const color = PASTEL_PALETTE[colorIndex % PASTEL_PALETTE.length];
            const pixelIndex = (y * this.canvas.width + x) * 4;

            data[pixelIndex] = color[0]; // R
            data[pixelIndex + 1] = color[1]; // G
            data[pixelIndex + 2] = color[2]; // B
            data[pixelIndex + 3] = 255;      // A
          }
        }
      }
    }

    this.ctx.putImageData(imageData, 0, 0);

    for (const seed of this.seeds) {
      this.ctx.fillStyle = "#FF0000";
      this.ctx.beginPath();
      this.ctx.arc(seed.x, seed.y, 4, 0, 2 * Math.PI);
      this.ctx.fill();
    }
    if (this.step >= 1) {
      requestAnimationFrame(() => { this.draw() });
    } else {
      console.log("Done.");
    }
  }

}
