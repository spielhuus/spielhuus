import { Vector3 } from '../../js/vector.ts';
import { Color } from '../../js/color.ts';
import { HitableList } from './hitable_list.ts';
import { Sphere } from './sphere.ts';
import { Camera } from './camera.ts';
import { Interval } from './interval.ts';

// Re-create the world inside the worker. In a real app, you would deserialize
// this from a message sent by the main thread.
function createWorld(): HitableList {
  const world = new HitableList();
  world.add(new Sphere(new Vector3(0, 0.1, -1), 0.5));
  world.add(new Sphere(new Vector3(0, -100.5, -1), 100));
  return world;
}

// The main rendering function, adapted from Camera.render
function render(width: number, height: number, world: HitableList, cameraSettings: any) {
  const cam = new Camera();
  // Apply settings from main thread
  cam.samples_per_pixel = cameraSettings.samples_per_pixel || 10;
  cam.max_depth = cameraSettings.max_depth || 10;

  cam.initialize(width, height);
  const imageData = new ImageData(width, height);

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      let pixel_color = new Color(0, 0, 0, 1);
      for (let sample = 0; sample < cam.samples_per_pixel; sample++) {
        let r = cam.get_ray(x, y);
        pixel_color.add(cam.ray_color(r, cam.max_depth, world));
      }

      // This scale was defined in camera.initialize()
      const scale = 1.0 / cam.samples_per_pixel;
      pixel_color.mulScalar(scale);

      // Apply gamma correction (gamma=2.0)
      pixel_color.r = Math.sqrt(pixel_color.r);
      pixel_color.g = Math.sqrt(pixel_color.g);
      pixel_color.b = Math.sqrt(pixel_color.b);

      const index = (y * width + x) * 4;
      const intensity = new Interval(0.000, 0.999);
      imageData.data[index] = intensity.clamp(pixel_color.r) * 255.0;
      imageData.data[index + 1] = intensity.clamp(pixel_color.g) * 255.0;
      imageData.data[index + 2] = intensity.clamp(pixel_color.b) * 255.0;
      imageData.data[index + 3] = 255;
    }

    // After each row, post a progress update.
    // To avoid message spam, you could do this less often (e.g., y % 5 === 0).
    self.postMessage({ type: 'progress', progress: (y + 1) / height });
  }

  // When done, post the final result.
  self.postMessage({ type: 'done', imageData: imageData });
}

self.onmessage = (event: MessageEvent) => {
  const { type, width, height, cameraSettings } = event.data;
  if (type === 'start') {
    const world = createWorld();
    render(width, height, world, cameraSettings);
  }
};

// This export statement is needed to satisfy TypeScript's module system for workers.
export default {};
