import type { DrawOptions } from './utils';
import { Map } from './maps';
import { Vector2 } from '../../js/vector';
import { Player } from './player';

const SCREEN_WIDTH = 640;
const SCREEN_HEIGHT = 480;

export class Canvas3 {
  map = new Map([
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 2, 0, 0, 3, 1, 1, 2, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 1, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  ]);
  player = new Player(1.5, 1.5, new Vector2(1.0, 0));;
  imageData: ImageData;
  keyPressed: string | undefined;
  lastTime = 0;
  maxVisibleDistance = 10.0;
  minBrightness = 0.1;
  constructor(private canvas: HTMLCanvasElement, private ctx: CanvasRenderingContext2D, private options: DrawOptions) {
    canvas.addEventListener('keydown', (event: KeyboardEvent) => {
      this.keyPressed = event.code;
    });

    canvas.addEventListener('keyup', () => {
      this.keyPressed = undefined;
    });

    this.imageData = this.ctx.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT);
  }

  public async draw(currentTime: number) {
    if (this.lastTime === 0) {
      this.lastTime = currentTime / 1000;
    }

    const deltaTime = currentTime / 1000 - this.lastTime;

    const ceilingColor = [52, 152, 219, 255]; // A nice sky blue
    const floorColor = [46, 64, 83, 255];     // A dark gray/brown
    // this.ctx.clearRect(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    // draw the direction arrow
    const dir = this.player.dir.clone().norm();
    const plane = new Vector2(-dir.y, dir.x).mulScalar(0.66);

    for (let x = 0; x < SCREEN_WIDTH; x++) {
      //calculate ray position and direction
      let cameraX = 2 * x / SCREEN_WIDTH - 1; //x-coordinate in camera space
      const rayDir = dir.clone().add(plane.clone().mulScalar(cameraX));
      const mapPos = new Vector2(Math.floor(this.player.pos.x), Math.floor(this.player.pos.y));
      const deltaDist = new Vector2(
        (rayDir.x == 0) ? 1e30 : Math.abs(1 / rayDir.x),
        (rayDir.y == 0) ? 1e30 : Math.abs(1 / rayDir.y),
      );

      let sideDist = new Vector2();
      let step = new Vector2();

      //calculate step and initial sideDist
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

        let target;
        if (sideDist.x <= sideDist.y) {
          target = rayDir.clone().mulScalar(sideDist.x);
          sideDist.x += deltaDist.x;
          mapPos.x += step.x;
          side = 0;
        }
        else {
          target = rayDir.clone().mulScalar(sideDist.y);
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

      //Calculate distance projected on camera direction (Euclidean distance would give fisheye effect!)
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

        wallHeight = Math.floor(SCREEN_HEIGHT / perpWallDist);
        wallStart = Math.floor(-wallHeight / 2 + SCREEN_HEIGHT / 2);
        if (wallStart < 0) wallStart = 0;
        wallEnd = Math.floor(wallHeight / 2 + SCREEN_HEIGHT / 2);
        if (wallEnd >= SCREEN_HEIGHT) wallEnd = SCREEN_HEIGHT;

      } else {
        // NO WALL WAS HIT. Treat as infinitely far away.
        perpWallDist = Infinity;
        wallHeight = 0;
        wallStart = SCREEN_HEIGHT / 2; // Horizon line
        wallEnd = SCREEN_HEIGHT / 2;   // Horizon line
      }

      const data = this.imageData.data; // Type: Uint8ClampedArray
      // draw the ceiling
      for (let y = 0; y < wallStart; y++) {
        const index = (y * SCREEN_WIDTH + x) * 4;
        data[index] = ceilingColor[0];
        data[index + 1] = ceilingColor[1];
        data[index + 2] = ceilingColor[2];
        data[index + 3] = ceilingColor[3];
      }

      // draw line column
      //
      if (hit) {
        let color = undefined;
        switch (this.map.data[mapPos.y][mapPos.x]) {
          case 1:
            color = [255, 0, 0, 255];
            break;
          case 2:
            color = [0, 255, 0, 255];
            break;
          case 3:
            color = [0, 0, 255, 255];
            break;
          case 4:
            color = [255, 0, 255, 255];
            break;
          default:
            throw `undefined color: ${this.map.data[mapPos.y][mapPos.x]}`;
        }
        if (color !== undefined) {

          let brightness = 1.0 - (perpWallDist / this.maxVisibleDistance);
          brightness = Math.max(brightness, this.minBrightness);
          brightness = Math.min(brightness, 1.0); // Safety clamp
          if (side === 1) {
            brightness *= 0.7; // Make N/S walls 30% darker
          }

           if (side === 1) {
            brightness *= 0.7; // Make N/S walls 30% darker
            }

          const shadedColor = [
            Math.floor(color[0] * brightness),
            Math.floor(color[1] * brightness),
            Math.floor(color[2] * brightness),
            color[3]
          ];

          for (let y = wallStart; y < wallEnd; y++) {
            const index = (y * SCREEN_WIDTH + x) * 4;
            data[index] = shadedColor[0];   // Red
            data[index + 1] = shadedColor[1];   // Green
            data[index + 2] = shadedColor[2];   // Blue
            data[index + 3] = shadedColor[3];   // Alpha (transparency)
          }
        }
      }
      //draw the floor
      for (let y = wallEnd; y < SCREEN_HEIGHT; y++) {
        const index = (y * SCREEN_WIDTH + x) * 4;
        data[index] = floorColor[0];
        data[index + 1] = floorColor[1];
        data[index + 2] = floorColor[2];
        data[index + 3] = floorColor[3];
      }
    }

    const imageBitmap = await createImageBitmap(this.imageData);
    // this.ctx.imageSmoothingEnabled = false; //TODO: really?
    this.ctx.drawImage(imageBitmap, 0, 0, this.canvas.width, this.canvas.height);
    imageBitmap.close();

    if (this.keyPressed) {
      let moveSpeed = deltaTime * 10.0; //the constant value is in squares/second
      let rotSpeed = deltaTime * 70.0; //the constant value is in radians/second
      switch (this.keyPressed) {
        case 'KeyW':
        case 'KeyS': { // Combine W and S logic
          const direction = (this.keyPressed === 'KeyW') ? 1 : -1;
          const moveVector = this.player.dir.clone().mulScalar(moveSpeed * direction);
          const newPos = this.player.pos.clone().add(moveVector);
          const mapX = Math.floor(newPos.x);
          const mapY = Math.floor(newPos.y);

          // Make sure we don't check outside the map bounds during the collision check itself
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
    this.lastTime = currentTime / 1000;
    requestAnimationFrame((dt) => { this.draw(dt) });
  }
}
