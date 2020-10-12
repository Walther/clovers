use crate::{
    materials::Material,
    objects::{
        Boxy, ConstantMedium, FlipFace, MovingSphere, RotateY, Sphere, Translate, XYRect, XZRect,
        YZRect,
    },
    Float, Ray, Vec3,
};
use rand::prelude::*;

use std::{cmp::Ordering, sync::Arc};

pub struct HitRecord<'a> {
    /// Distance from the ray origin to the hitpoint
    pub distance: Float,
    /// 3D coordinate of the hitpoint
    pub position: Vec3,
    /// Surface normal from the hitpoint
    pub normal: Vec3,
    /// U surface coordinate of the hitpoint
    pub u: Float,
    /// V surface coordinate of the hitpoint
    pub v: Float,
    /// Reference to the material at the hitpoint
    pub material: &'a Material,
    /// Is the hitpoint at the front of the surface
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    // Helper function for getting normals pointing at the correct direction
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub enum Hitable {
    Boxy(Boxy),
    ConstantMedium(ConstantMedium),
    MovingSphere(MovingSphere),
    XZRect(XZRect),
    XYRect(XYRect),
    YZRect(YZRect),
    RotateY(RotateY),
    Sphere(Sphere),
    Translate(Translate),
    BVHNode(BVHNode),
    HitableList(HitableList),
    FlipFace(FlipFace),
}

impl Hitable {
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        match self {
            Hitable::Boxy(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::ConstantMedium(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::MovingSphere(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::XZRect(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::XYRect(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::YZRect(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::RotateY(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Sphere(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Translate(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::BVHNode(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::HitableList(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::FlipFace(h) => h.hit(ray, distance_min, distance_max, rng),
        }
    }

    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        match self {
            Hitable::Boxy(h) => h.bounding_box(t0, t1),
            Hitable::ConstantMedium(h) => h.bounding_box(t0, t1),
            Hitable::MovingSphere(h) => h.bounding_box(t0, t1),
            Hitable::XZRect(h) => h.bounding_box(t0, t1),
            Hitable::XYRect(h) => h.bounding_box(t0, t1),
            Hitable::YZRect(h) => h.bounding_box(t0, t1),
            Hitable::RotateY(h) => h.bounding_box(t0, t1),
            Hitable::Sphere(h) => h.bounding_box(t0, t1),
            Hitable::Translate(h) => h.bounding_box(t0, t1),
            Hitable::BVHNode(h) => h.bounding_box(t0, t1),
            Hitable::HitableList(h) => h.bounding_box(t0, t1),
            Hitable::FlipFace(h) => h.bounding_box(t0, t1),
        }
    }

    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: ThreadRng) -> Float {
        match self {
            Hitable::XZRect(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::XYRect(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::YZRect(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::HitableList(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Sphere(h) => h.pdf_value(origin, vector, time, rng),
            _ => 0.0,
        }
    }

    pub fn random(&self, origin: Vec3, rng: ThreadRng) -> Vec3 {
        match self {
            Hitable::XZRect(h) => h.random(origin, rng),
            Hitable::XYRect(h) => h.random(origin, rng),
            Hitable::YZRect(h) => h.random(origin, rng),
            Hitable::HitableList(h) => h.random(origin, rng),
            Hitable::Sphere(h) => h.random(origin, rng),
            _ => Vec3::new(1.0, 0.0, 0.0),
        }
    }

    pub fn add(&mut self, object: Hitable) {
        match self {
            Hitable::HitableList(h) => h.add(object),
            _ => panic!("Cannot add to other types of Hitable"),
        }
    }
}

/// Helper struct for storing multiple `Hitable` objects. This list has a `Hitable` implementation too, returning the closest possible hit
pub struct HitableList(pub Vec<Arc<Hitable>>);

impl From<Vec<Arc<Hitable>>> for HitableList {
    fn from(v: Vec<Arc<Hitable>>) -> Self {
        HitableList(v)
    }
}

impl HitableList {
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = distance_max;
        for hitable in self.0.iter() {
            if let Some(record) = hitable.hit(ray, distance_min, closest, rng) {
                closest = record.distance;
                hit_record = Some(record);
            }
        }
        hit_record
    }

    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        if self.0.is_empty() {
            return None;
        }

        let mut output_box: Option<AABB> = None;

        for object in self.0.iter() {
            // Check if the object has a box
            match object.bounding_box(t0, t1) {
                // No box found, early return.
                // Having even one unbounded object in a list makes the entire list unbounded
                None => return None,
                // Box found
                Some(bounding) =>
                // Do we have an output_box already saved?
                {
                    match output_box {
                        // If we do, expand it & recurse
                        Some(old_box) => {
                            output_box = Some(AABB::surrounding_box(old_box, bounding));
                        }
                        // Otherwise, set output box to be the newly-found box
                        None => {
                            output_box = Some(bounding);
                        }
                    }
                }
            }
        }

        output_box
    }
    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: ThreadRng) -> Float {
        let weight = 1.0 / self.0.len() as Float;
        let mut sum = 0.0;

        self.0.iter().for_each(|object| {
            sum += weight * object.pdf_value(origin, vector, time, rng);
        });

        sum
    }

    pub fn random(&self, origin: Vec3, mut rng: ThreadRng) -> Vec3 {
        let int_size = self.0.len();
        self.0[rng.gen_range(0, int_size)].random(origin, rng)
    }

    pub fn new() -> HitableList {
        HitableList(Vec::new())
    }

    pub fn add(&mut self, object: Hitable) {
        self.0.push(Arc::new(object));
    }

    pub fn into_bvh(self, time_0: Float, time_1: Float, rng: ThreadRng) -> Hitable {
        let bvh_node = BVHNode::from_list(self.0, time_0, time_1, rng);
        Hitable::BVHNode(bvh_node)
    }

    // TODO: fixme, silly
    pub fn into_hitable(self) -> Hitable {
        Hitable::HitableList(self)
    }
}

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    /// "An Optimized AABB Hit Method" https://raytracing.github.io/books/RayTracingTheNextWeek.html
    pub fn hit(&self, ray: &Ray, mut tmin: Float, mut tmax: Float) -> bool {
        for a in 0..3 {
            let invd = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * invd;
            let mut t1 = (self.max[a] - ray.origin[a]) * invd;
            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small: Vec3 = Vec3::new(
            (box0.min.x).min(box1.min.x),
            (box0.min.y).min(box1.min.y),
            (box0.min.z).min(box1.min.z),
        );

        let big: Vec3 = Vec3::new(
            (box0.max.x).max(box1.max.x),
            (box0.max.y).max(box1.max.y),
            (box0.max.z).max(box1.max.z),
        );

        AABB::new(small, big)
    }
}

pub struct BVHNode {
    left: Arc<Hitable>,
    right: Arc<Hitable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn from_list(
        mut objects: Vec<Arc<Hitable>>,
        time_0: Float,
        time_1: Float,
        mut rng: ThreadRng,
    ) -> BVHNode {
        {
            let axis: usize = rng.gen_range(0, 2);
            let comparators = [box_x_compare, box_y_compare, box_z_compare];
            let comparator = comparators[axis];

            let object_span = objects.len();

            let left: Arc<Hitable>;
            let right: Arc<Hitable>;

            if object_span == 1 {
                // If we only have one object, return itself. Note: no explicit leaf type in our tree
                left = objects[0].clone();
                right = objects[0].clone();
            } else if object_span == 2 {
                // If we are comparing two objects, perform the comparison
                match comparator(&objects[0], &objects[1]) {
                    Ordering::Less => {
                        left = objects[0].clone();
                        right = objects[1].clone();
                    }
                    Ordering::Greater => {
                        left = objects[1].clone();
                        right = objects[0].clone();
                    }
                    Ordering::Equal => {
                        // TODO: what should happen here?
                        panic!("Equal objects in BVHNode from_list");
                    }
                }
            } else {
                // Otherwise, recurse
                objects.sort_by(|a, b| comparator(&*a, &*b));

                // Split the vector; divide and conquer
                let mid = object_span / 2;
                let objects_right = objects.split_off(mid);
                left = Arc::new(Hitable::BVHNode(BVHNode::from_list(
                    objects, time_0, time_1, rng,
                )));
                right = Arc::new(Hitable::BVHNode(BVHNode::from_list(
                    objects_right,
                    time_0,
                    time_1,
                    rng,
                )));
            }

            let box_left = left.bounding_box(time_0, time_1);
            let box_right = right.bounding_box(time_0, time_1);

            if box_left.is_none() || box_right.is_none() {
                panic!("No bounding box in bvh_node constructor");
            } else {
                let bounding_box = AABB::surrounding_box(box_left.unwrap(), box_right.unwrap());

                BVHNode {
                    left,
                    right,
                    bounding_box,
                }
            }
        }
    }

    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        match self.bounding_box.hit(&ray, distance_min, distance_max) {
            false => None,
            true => {
                let hit_left = self.left.hit(&ray, distance_min, distance_max, rng);
                let hit_right = self.right.hit(&ray, distance_min, distance_max, rng);

                match &hit_left {
                    Some(left) => {
                        match &hit_right {
                            // Both hit
                            Some(right) => {
                                if left.distance < right.distance {
                                    hit_left
                                } else {
                                    hit_right
                                }
                            }
                            // Left hit
                            None => hit_left,
                        }
                    }
                    None => match &hit_right {
                        // Right hit
                        Some(_right) => hit_right,
                        // Neither hit
                        None => None,
                    },
                }
            }
        }
    }
    pub fn bounding_box(&self, _t0: Float, _t11: Float) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

fn box_compare(a: &Hitable, b: &Hitable, axis: usize) -> Ordering {
    let box_a: Option<AABB> = a.bounding_box(0.0, 0.0);
    let box_b: Option<AABB> = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        panic!("No bounding box in BVHNode constructor.")
    } else if box_a.unwrap().min[axis] < box_b.unwrap().min[axis] {
        Ordering::Less
    } else {
        // Default to greater, even if equal
        Ordering::Greater
    }
}

fn box_x_compare(a: &Hitable, b: &Hitable) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Hitable, b: &Hitable) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Hitable, b: &Hitable) -> Ordering {
    box_compare(a, b, 2)
}
