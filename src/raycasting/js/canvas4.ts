import type { DrawOptions } from './utils';
import { Map } from './maps';
import { Vector2 } from '../../js/vector';
import { Player } from './player';

const SCREEN_WIDTH = 640;
const SCREEN_HEIGHT = 480;

type Texture = {
  data: Uint8ClampedArray;
  width: number;
  height: number;
};

export class Canvas4 {
  map = new Map(
    [
      [1, 1, 2, 1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 1, 1, 1, 7, 7, 7, 7, 7, 7, 7, 7],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7],
      [1, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7],
      [1, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7],
      [1, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7],
      [2, 0, 5, 0, 0, 0, 0, 5, 5, 6, 5, 5, 5, 6, 5, 5, 7, 7, 0, 7, 7, 7, 7, 7],
      [1, 0, 5, 0, 0, 0, 0, 5, 0, 8, 0, 8, 0, 8, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1],
      [1, 0, 5, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8],
      [1, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 7, 1],
      [1, 0, 5, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8],
      [1, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1],
      [1, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 0, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 1],
      [1, 3, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4],
      [1, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6],
      [2, 4, 4, 4, 4, 4, 0, 4, 4, 4, 6, 0, 6, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2],
      [1, 0, 6, 0, 6, 0, 0, 0, 0, 4, 6, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 2],
      [1, 0, 0, 5, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2],
      [1, 0, 7, 0, 7, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2],
      [1, 5, 5, 6, 7, 5, 6, 5, 5, 5, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3]
    ]);
  player = new Player(1.5, 1.5, new Vector2(1.0, 0));;
  imageData: ImageData;
  keyPressed: string | undefined;
  lastTime = 0;
  private isReady = false; // The "ready" flag
  private frameCount = 0;
  private lastFpsUpdate = 0;
  private displayedFps = 0;
  maxVisibleDistance = 10.0;
  minBrightness = 0.1;
  isActive = false;
  textures: Texture[] = [];
  private renderCanvas: HTMLCanvasElement;
  private renderCtx: CanvasRenderingContext2D;
  constructor(private canvas: HTMLCanvasElement, private ctx: CanvasRenderingContext2D, private options: DrawOptions) {
    this.renderCanvas = document.createElement('canvas');
    this.renderCanvas.width = SCREEN_WIDTH;
    this.renderCanvas.height = SCREEN_HEIGHT;
    this.renderCtx = this.renderCanvas.getContext('2d')!;
    canvas.addEventListener('keydown', (event: KeyboardEvent) => {
      this.keyPressed = event.code;
    });

    canvas.addEventListener('keyup', () => {
      this.keyPressed = undefined;
    });

    this.imageData = this.ctx.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT);
  }

  public draw(currentTime: number) {
    console.log("draw canvas3");

    if (!this.isReady) {
      this.ctx.fillStyle = 'black';
      this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
      this.ctx.font = "20px Arial";
      this.ctx.fillStyle = 'white';
      this.ctx.textBaseline = 'middle';
      this.ctx.textAlign = 'center';
      this.ctx.fillText("Loading Textures...", this.canvas.width / 2, this.canvas.height / 2);

      if (this.isActive) {
        requestAnimationFrame((dt) => { this.draw(dt) });
      }
      return;
    }

    const currentTimeInSeconds = currentTime / 1000;

    if (this.lastTime === 0) {
      this.lastTime = currentTimeInSeconds;
    }
    const deltaTime = currentTimeInSeconds - this.lastTime;


    const dir = this.player.dir.clone().norm();
    const plane = new Vector2(-dir.y, dir.x).mulScalar(0.66);

    let rayDir = new Vector2();
    const playerMapPos = new Vector2(Math.floor(this.player.pos.x), Math.floor(this.player.pos.y));
    let mapPos = new Vector2();
    const deltaDist = new Vector2();
    let sideDist = new Vector2();
    let step = new Vector2();
    let target = new Vector2();

    for (let x = 0; x < SCREEN_WIDTH; x++) {
      let cameraX = 2 * x / SCREEN_WIDTH - 1; //x-coordinate in camera space
      rayDir.copyFrom(dir.clone().add(plane.clone().mulScalar(cameraX)));
      mapPos.copyFrom(playerMapPos);
      deltaDist.set(
        (rayDir.x == 0) ? 1e30 : Math.abs(1 / rayDir.x),
        (rayDir.y == 0) ? 1e30 : Math.abs(1 / rayDir.y),
      );

      if (rayDir.x < 0) {
        step.x = -1;
        sideDist.x = (this.player.pos.x - mapPos.x) * deltaDist.x;
      }
      else {
        step.x = 1;
        sideDist.x = (mapPos.x + 1.0 - this.player.pos.x) * deltaDist.x;
      }
      if (rayDir.y < 0) {
        step.y = -1;
        sideDist.y = (this.player.pos.y - mapPos.y) * deltaDist.y;
      }
      else {
        step.y = 1;
        sideDist.y = (mapPos.y + 1.0 - this.player.pos.y) * deltaDist.y;
      }

      let hit = false;
      let side = 0;

      //perform DDA
      while (!hit) {

        if (sideDist.x <= sideDist.y) {
          target.copyFrom(rayDir.clone().mulScalar(sideDist.x));
          sideDist.x += deltaDist.x;
          mapPos.x += step.x;
          side = 0;
        }
        else {
          target.copyFrom(rayDir.clone().mulScalar(sideDist.y));
          sideDist.y += deltaDist.y;
          mapPos.y += step.y;
          side = 1;
        }

        if (mapPos.x < 0 || mapPos.y < 0 || mapPos.x >= this.map.height || mapPos.y >= this.map.height) {
          break;
        }

        //Check if ray has hit a wall
        if (this.map.data[mapPos.y][mapPos.x] > 0) {
          hit = true;
        }
      }

      let perpWallDist;
      let wallHeight;
      let wallStart;
      let wallEnd;

      if (hit) {
        // A wall was hit, calculate its properties as before.
        if (side == 0) {
          perpWallDist = (sideDist.x - deltaDist.x);
        } else {
          perpWallDist = (sideDist.y - deltaDist.y);
        }

        wallHeight = SCREEN_HEIGHT / perpWallDist;
        const drawStart = -wallHeight / 2 + SCREEN_HEIGHT / 2;
        const drawEnd = wallHeight / 2 + SCREEN_HEIGHT / 2;
        wallStart = Math.max(0, Math.floor(drawStart));
        wallEnd = Math.min(SCREEN_HEIGHT, Math.floor(drawEnd));

      } else {
        // NO WALL WAS HIT. Treat as infinitely far away.
        perpWallDist = Infinity;
        wallHeight = 0;
        wallStart = SCREEN_HEIGHT / 2;
        wallEnd = SCREEN_HEIGHT / 2;
      }

      const data = this.imageData.data;

      // draw the ceiling
      for (let y = 0; y < wallStart; y++) {
        const index = (y * SCREEN_WIDTH + x) * 4;
        data[index] = this.options.ceilingColor[0];
        data[index + 1] = this.options.ceilingColor[1];
        data[index + 2] = this.options.ceilingColor[2];
        data[index + 3] = this.options.ceilingColor[3];
      }

      // draw line column
      if (hit) {

        //draw the walls
        let texNum = this.map.data[mapPos.y][mapPos.x] - 1;

        const texture = this.textures[texNum];
        const texWidth = texture.width;
        const texHeight = texture.height;

        let wallX;

        if (side == 0) wallX = this.player.pos.y + perpWallDist * rayDir.y;
        else wallX = this.player.pos.x + perpWallDist * rayDir.x;
        wallX -= Math.floor((wallX));

        let texX = Math.floor(wallX * texWidth);
        if (side == 0 && rayDir.x > 0) texX = texWidth - texX - 1;
        if (side == 1 && rayDir.y < 0) texX = texWidth - texX - 1;

        const step = 1.0 * texHeight / wallHeight;

        const drawStart = -wallHeight / 2 + SCREEN_HEIGHT / 2;
        let texPos = (drawStart - SCREEN_HEIGHT / 2 + wallHeight / 2) * step;

        const wallStart = Math.max(0, Math.floor(drawStart));
        const wallEnd = Math.min(SCREEN_HEIGHT, Math.floor(wallHeight / 2 + SCREEN_HEIGHT / 2));

        if (drawStart < 0) {
          texPos += -drawStart * step;
        }

        for (let y = wallStart; y < wallEnd; y++) {
          let texY = Math.floor(texPos) % texHeight;
          texPos += step;

          let brightness = 1.0 - (perpWallDist / this.maxVisibleDistance);
          brightness = Math.max(brightness, this.minBrightness);
          brightness = Math.min(brightness, 1.0);
          if (side === 1) {
            brightness *= 0.7;
          }

          if (side === 1) {
            brightness *= 0.7;
          }

          const texIndex = (texY * texWidth + texX) * 4;
          const screenIndex = (y * SCREEN_WIDTH + x) * 4;
          data[screenIndex] = texture.data[texIndex] * brightness;     // R
          data[screenIndex + 1] = texture.data[texIndex + 1] * brightness; // G
          data[screenIndex + 2] = texture.data[texIndex + 2] * brightness; // B
          data[screenIndex + 3] = texture.data[texIndex + 3] * brightness; // A
        }
      }

      //draw the floor
      for (let y = wallEnd; y < SCREEN_HEIGHT; y++) {
        const index = (y * SCREEN_WIDTH + x) * 4;
        data[index] = this.options.floorColor[0];
        data[index + 1] = this.options.floorColor[1];
        data[index + 2] = this.options.floorColor[2];
        data[index + 3] = this.options.floorColor[3];
      }
    }

    this.renderCtx.putImageData(this.imageData, 0, 0);
    this.ctx.drawImage(
      this.renderCanvas,
      0, 0, SCREEN_WIDTH, SCREEN_HEIGHT,
      0, 0, this.canvas.width, this.canvas.height
    );

    if (this.keyPressed) {
      let moveSpeed = deltaTime * 5.0;
      let rotSpeed = deltaTime * 100.0;
      switch (this.keyPressed) {
        case 'KeyW':
        case 'KeyS': { // Combine W and S logic
          const direction = (this.keyPressed === 'KeyW') ? 1 : -1;
          const moveVector = this.player.dir.clone().mulScalar(moveSpeed * direction);
          const newPos = this.player.pos.clone().add(moveVector);
          const mapX = Math.floor(newPos.x);
          const mapY = Math.floor(newPos.y);

          if (mapX >= 0 && mapX < this.map.width && mapY >= 0 && mapY < this.map.height) {
            if (this.map.data[mapY][mapX] === 0) {
              this.player.pos = newPos;
            }
          }
        }
          break;
        case 'KeyA':
          this.player.dir = this.player.dir.clone().rotate(-rotSpeed);
          break;
        case 'KeyD':
          this.player.dir = this.player.dir.clone().rotate(rotSpeed);
          break;
        default:
          break;
      }
    }

    if (this.lastFpsUpdate === 0) {
      this.lastFpsUpdate = currentTimeInSeconds;
    }
    this.frameCount++;
    if (currentTimeInSeconds - this.lastFpsUpdate >= 1.0) {
      this.displayedFps = this.frameCount;
      this.frameCount = 0;
      this.lastFpsUpdate = currentTimeInSeconds;
    }

    this.ctx.font = this.options.title_text_font;
    this.ctx.fillStyle = this.options.title_text_style;
    this.ctx.textBaseline = 'top';
    this.ctx.textAlign = 'left';
    this.ctx.fillText(`${this.displayedFps} FPS`, 10, 10);

    this.lastTime = currentTimeInSeconds;

    if (!this.isActive) {
      this.ctx.fillStyle = this.options.pause_color;
      this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
      this.ctx.fillText("click to start", this.canvas.width / 2 - 20, this.canvas.height / 2);
    } else {
      requestAnimationFrame((dt) => { this.draw(dt) });
    }
  }

  public async initialize() {
    const textureUrls = [
      '../../textures/walls-brick-stone-tiles-2/wall_corrugated_no-drain.png',
      '../../textures/walls-brick-stone-tiles-2/wall_corrugated_drain.png',
      '../../textures/walls-brick-stone-tiles-2/wall_stonechip_panel.png',
      '../../textures/walls-brick-stone-tiles-2/wall_concrete_dirty2.png',
      '../../textures/walls-brick-stone-tiles-2/wall_steel_plate2.png',
      '../../textures/walls-brick-stone-tiles-2/wall_steel_plate1.png',
      '../../textures/walls-brick-stone-tiles-2/wall_steel_rustic.png',
      '../../textures/walls-brick-stone-tiles-2/wall_conc_slabs_yellow.png',

      // '../../textures/wolftex/eagle.png',
      // '../../textures/wolftex/redbrick.png', 
      // '../../textures/wolftex/purplestone.png',
      // '../../textures/wolftex/greystone.png',
      // '../../textures/wolftex/bluestone.png',
      // '../../textures/wolftex/mossy.png',
      // '../../textures/wolftex/wood.png',  
      // '../../textures/wolftex/colorstone.png'
    ];

    try {
      console.log("Loading textures...");
      // Use Promise.all to load all textures concurrently.
      const loadedTextures = await Promise.all(
        textureUrls.map(url => this.loadAndProcessTexture(url))
      );
      this.textures = loadedTextures;
      console.log("All textures loaded successfully!");
      this.isReady = true;
    } catch (error) {
      console.error("Could not initialize Canvas4:", error);
    }
  }

  private async loadAndProcessTexture(url: string): Promise<Texture> {
    return new Promise((resolve, reject) => {
      const image = new Image();
      image.onload = () => {
        const tempCanvas = document.createElement('canvas');
        tempCanvas.width = image.width;
        tempCanvas.height = image.height;

        const ctx = tempCanvas.getContext('2d');
        if (!ctx) { throw "not loaded" }

        ctx.drawImage(image, 0, 0);
        const imageData = ctx.getImageData(0, 0, image.width, image.height);

        resolve({
          data: imageData.data,
          width: image.width,
          height: image.height,
        });
      };
      image.onerror = () => reject(new Error(`Failed to load texture: ${url}`));
      image.src = url;
    });
  }
}
