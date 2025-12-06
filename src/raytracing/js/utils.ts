import { Vector2 } from '../../js/vector';

export class DrawOptions {
  gridSize: Vector2 = new Vector2(0, 0);
  lineColor: string = 'rgba(50, 50, 200, 0.7)';
  lineWidth: number = 0.5;
  line_radius: number = 1;
  circle_radius: number = 4;
  circle_color: string = 'orange';
  wallColor: string = 'rgba(50, 50, 200, 0.7)';
  pause_color: string = "#222222aa";
  title_text_font: string = '32px sans-serif';
  title_text_style: string = "white";
  ceilingColor: number[] = [52, 152, 219, 255];
  floorColor: number[] = [46, 64, 83, 255];
  wallColors = [
    [255, 0, 0, 255],
    [255, 100, 0, 255],
    [0, 255, 0, 255],
    [0, 0, 255, 255],
    [255, 0, 255, 255],
    [255, 200, 255, 255],
    [255, 100, 255, 255],
    [255, 40, 255, 255],
  ];
  constructor(options?: Partial<DrawOptions>) {
    Object.assign(this, options);


  }
}
