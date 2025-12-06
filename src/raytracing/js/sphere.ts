import type { Vector3 } from "../../js/vector.ts";
import type { Ray } from "./canvas1.ts";
import type { HitRecord, IHitable } from "./hitable.ts"
import type { Interval } from "./interval.ts";

export class Sphere implements IHitable {
  constructor(private center: Vector3, private radius: number) { };
  hit(ray: Ray, ray_t: Interval, rec: HitRecord): boolean {
    let oc = this.center.clone().sub(ray.origin);
    let a = ray.direction.lengthSquared();
    let h = ray.direction.dot(oc);
    let c = oc.lengthSquared() - this.radius * this.radius;
    let d = h * h - a * c;
    if (d < 0) {
      return false;
    }
    let sqrtd = Math.sqrt(d);

    // Find the nearest root that lies in the acceptable range.
    let root = (h - sqrtd) / a;
    if (!ray_t.surrounds(root)) {
      root = (h + sqrtd) / a;
      if (!ray_t.surrounds(root))
        return false;
    }

    rec.t = root;
    rec.p = ray.at(rec.t);
    let outward_normal = rec.p.clone().sub(this.center).divScalar(this.radius);
    rec.setFaceNormal(ray, outward_normal);

    return true;
  }
}
