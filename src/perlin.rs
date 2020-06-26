use crate::{Float, Vec3};
use rand::prelude::*;

pub struct Perlin {
    random_float: Vec<Float>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

fn perlin_generate_perm(point_count: usize, rng: ThreadRng) -> Vec<usize> {
    let mut perm: Vec<usize> = Vec::with_capacity(point_count);

    for i in 0..point_count {
        perm.push(i);
    }
    permute(point_count, &mut perm, rng);

    return perm;
}

fn permute(point_count: usize, p: &mut Vec<usize>, mut rng: ThreadRng) {
    // For some reason the tutorial wants the reverse loop
    for i in (1..point_count).rev() {
        let target: usize = rng.gen_range(0, i);
        let tmp: usize = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

impl Perlin {
    pub fn new(point_count: usize, mut rng: ThreadRng) -> Self {
        let mut random_float: Vec<Float> = Vec::with_capacity(point_count);
        for i in 0..point_count {
            random_float.push(rng.gen::<Float>());
        }

        let perm_x = perlin_generate_perm(point_count, rng);
        let perm_y = perlin_generate_perm(point_count, rng);
        let perm_z = perlin_generate_perm(point_count, rng);

        Perlin {
            random_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: Vec3) -> Float {
        // Why are these unused in the tutorial?
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = (4.0 * point.x) as usize & 255;
        let j = (4.0 * point.y) as usize & 255;
        let k = (4.0 * point.z) as usize & 255;

        return self.random_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]];
    }
}
