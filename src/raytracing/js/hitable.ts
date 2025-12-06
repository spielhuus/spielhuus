import { Vector3 } from '../../js/vector.ts';
import { Ray } from "./canvas1.ts";
import type { Interval } from './interval.ts';

export class HitRecord {
  p: Vector3;
  normal: Vector3;
  t: number;
  front_face: boolean;
  constructor(p: Vector3, normal: Vector3, t: number) {
    this.p = p;
    this.normal = normal;
    this.t = t;
    this.front_face = false;
  }

  setFaceNormal(ray: Ray, outward_normal: Vector3) {
    // Sets the hit record normal vector.
    // NOTE: the parameter `outward_normal` is assumed to have unit length.

    this.front_face = ray.direction.clone().dot(outward_normal) < 0;
    this.normal = this.front_face ? outward_normal.clone() : outward_normal.clone().mulScalar(-1);
  }
};

export interface IHitable {
  hit(r: Ray, ray_t: Interval, rec: HitRecord): boolean;
}
