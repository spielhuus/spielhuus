import { Color } from "../../js/color";
import { Vector3 } from "../../js/vector";
import { Ray } from "./canvas1";
import { HitRecord, type IHitable } from "./hitable"
import { Interval } from "./interval";

export class Camera {
  /* Public Camera Parameters Here */
  center: Vector3;
  pixel00_loc: Vector3;
  pixel_delta_u: Vector3;
  pixel_delta_v: Vector3;
  samples_per_pixel = 100;   // Count of random samples for each pixel
  private imageData: ImageData | null = null;
  private world: IHitable | null = null;
  private renderY = 0; // Current row being rendered
  private width = 0;
  private height = 0;
  max_depth = 50;
  private pixel_samples_scale = 0;

  private random_on_hemisphere(normal: Vector3): Vector3 {
    let on_unit_sphere = Vector3.random_unit_vector();
    if (on_unit_sphere.dot(normal) > 0.0) // In the same hemisphere as the normal
      return on_unit_sphere;
    else
      return on_unit_sphere.mulScalar(-1);
  }

  render(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, world: IHitable) {
    let width = canvas.width;
    let height = canvas.height;
    let imageData: ImageData = ctx.createImageData(width, height);
    let renderCanvas = document.createElement('canvas');
    renderCanvas.width = width;
    renderCanvas.height = height;
    let renderCtx = renderCanvas.getContext('2d')!;
    this.initialize(width, height);

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        let pixel_color = new Color(0, 0, 0, 1);
        for (let sample = 0; sample < this.samples_per_pixel; sample++) {
          let r = this.get_ray(x, y);
          pixel_color.add(this.ray_color(r, this.max_depth, world));
        }
        // let pixel_center = this.pixel00_loc.clone().add(this.pixel_delta_u.clone().mulScalar(x).add(this.pixel_delta_v.clone().mulScalar(y)));
        // let ray_direction = pixel_center.clone().sub(this.center);
        // let r = new Ray(this.center.clone(), ray_direction);
        // let color = this.ray_color(r, world);
        const index = (y * width + x) * 4;
        // // let color = new Color((x / SCREEN_WIDTH) * 255, (y / SCREEN_HEIGHT) * 255.0, 0, 255);
        pixel_color.mulScalar(this.pixel_samples_scale);
        // write color
        let intensity = new Interval(0.000, 0.999);
        imageData.data[index] = intensity.clamp(pixel_color.r) * 255.0;
        imageData.data[index + 1] = intensity.clamp(pixel_color.g) * 255.0;
        imageData.data[index + 2] = intensity.clamp(pixel_color.b) * 255.0;
        imageData.data[index + 3] = 255; //ixel_color.a) * 255.0 * this.pixel_samples_scale;
      }
    }
    console.log("done redering");

    renderCtx.putImageData(imageData, 0, 0);
    ctx.drawImage(
      renderCanvas,
      0, 0, width, height,
      0, 0, canvas.width, canvas.height
    );

  }

  get_ray(i: number, j: number): Ray {
    // Construct a camera ray originating from the origin and directed at randomly sampled
    // point around the pixel location i, j.

    let offset = this.sample_square();
    let pixel_sample = this.pixel00_loc.clone().add(
      this.pixel_delta_u.clone().mulScalar((i + offset.x))
        .add(this.pixel_delta_v.clone().mulScalar(j + offset.y)));

    let ray_origin = this.center.clone();
    let ray_direction = pixel_sample.sub(ray_origin);

    return new Ray(ray_origin, ray_direction);
  }

  sample_square(): Vector3 {
    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    return new Vector3(Math.random() - 0.5, Math.random() - 0.5, 0);
  }


  /* Private Camera Variables Here */
  initialize(width: number, height: number) {
    this.width = width;
    this.height = height;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * width / height;

    this.center = new Vector3(0, 0, 0);
    // the start x and y vector
    let viewport_u = new Vector3(viewport_width, 0, 0);
    let viewport_v = new Vector3(0, -viewport_height, 0);
    // the step sizes

    this.pixel_delta_u = viewport_u.clone().divScalar(width);
    this.pixel_delta_v = viewport_v.clone().divScalar(height);

    let viewport_upper_left = this.center.clone().sub(
      new Vector3(0, 0, focal_length)).sub(viewport_u.clone().divScalar(2)).sub(viewport_v.clone().divScalar(2));
    this.pixel00_loc = viewport_upper_left.clone().add((this.pixel_delta_u.clone().add(this.pixel_delta_v).mulScalar(0.5)));
    this.pixel_samples_scale = 1.0 / this.samples_per_pixel;
  }

  ray_color(ray: Ray, depth: number, world: IHitable): Color {
    if (depth <= 0)
      return new Color(0, 0, 0);
    let record = new HitRecord(new Vector3(), new Vector3(), 0);
    if (world.hit(ray, new Interval(0.0001, Infinity), record)) {
      // let direction = this.random_on_hemisphere(record.normal);
      let direction = Vector3.random_unit_vector().add(record.normal);
      return this.ray_color(new Ray(record.p, direction), depth - 1, world).mulScalar(0.5);
    }

    let unit_direction = ray.direction.clone().norm();
    let a = 0.5 * (unit_direction.y + 1.0);
    const start_color = new Color(1.0, 1.0, 1.0, 1.0); // white
    const end_color = new Color(0.5, 0.7, 1.0, 1.0);   // blue
    return start_color.mulScalar(1.0 - a).add(end_color.mulScalar(a));
  }

  // --- NEW METHOD: startRender ---
  public startRender(ctx: CanvasRenderingContext2D, world: IHitable) {
    this.imageData = ctx.createImageData(this.width, this.height);
    this.world = world;
    this.renderY = 0;
  }

  // --- NEW METHOD: renderChunk ---
  // Renders one row of pixels and returns the progress.
  public renderChunk(): { progress: number, imageData: ImageData } {
    if (!this.imageData || !this.world || this.renderY >= this.height) {
      // Should not happen if logic is correct, but a good safeguard.
      return { progress: 1, imageData: this.imageData! };
    }

    const y = this.renderY;
    for (let x = 0; x < this.width; x++) {
      let pixel_color = new Color(0, 0, 0, 1);
      for (let sample = 0; sample < this.samples_per_pixel; sample++) {
        let r = this.get_ray(x, y);
        pixel_color.add(this.ray_color(r, this.max_depth, this.world));
      }

      pixel_color.mulScalar(this.pixel_samples_scale);
      // Gamma correction
      pixel_color.r = Math.sqrt(pixel_color.r);
      pixel_color.g = Math.sqrt(pixel_color.g);
      pixel_color.b = Math.sqrt(pixel_color.b);

      const index = (y * this.width + x) * 4;
      const intensity = new Interval(0.000, 0.999);
      this.imageData.data[index] = intensity.clamp(pixel_color.r) * 255.0;
      this.imageData.data[index + 1] = intensity.clamp(pixel_color.g) * 255.0;
      this.imageData.data[index + 2] = intensity.clamp(pixel_color.b) * 255.0;
      this.imageData.data[index + 3] = 255;
    }

    this.renderY++; // Move to the next row for the next chunk

    return {
      progress: this.renderY / this.height,
      imageData: this.imageData
    };
  }
};
