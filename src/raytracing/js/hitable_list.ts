import { Vector3 } from '../../js/vector.ts';
import type { IHitable } from "./hitable.ts";
import { HitRecord } from "./hitable.ts";
import { Ray } from "./canvas1.ts";
import { Interval } from './interval.ts';

export class HitableList implements IHitable {
  objects: IHitable[];
  constructor() {
    this.objects = [];
  }

  clear() { this.objects = []; }

  add(object: IHitable) {
    this.objects.push(object);
  }
  hit(r: Ray, ray_t: Interval, rec: HitRecord): boolean {
    let temp_rec = new HitRecord(new Vector3(), new Vector3(), 0);
    let hit_anything = false;
    let closest_so_far = ray_t.max;

    for (let object of this.objects) {
      if (object.hit(r, new Interval(ray_t.min, closest_so_far), temp_rec)) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        rec.p = temp_rec.p.clone();
        rec.normal = temp_rec.normal.clone();
        rec.t = temp_rec.t;
        rec.front_face = temp_rec.front_face;
      }
    }

    return hit_anything;
  }
}
