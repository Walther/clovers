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

fn trilinear_interp(c: [[[Float; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
    let mut accum: Float = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f = i as Float;
                let j_f = j as Float;
                let k_f = k as Float;
                accum += (i_f * u + (1.0 - i_f) * (1.0 - u))
                    * (j_f * v + (1.0 - j_f) * (1.0 - v))
                    * (k_f * w + (1.0 - k_f) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    return accum;
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
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i: usize = point.x.floor() as usize;
        let j: usize = point.y.floor() as usize;
        let k: usize = point.z.floor() as usize;

        let mut c: [[[Float; 2]; 2]; 2] = [[[0.0, 0.0], [0.0, 0.0]], [[0.0, 0.0], [0.0, 0.0]]];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_float[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }
        return trilinear_interp(c, u, v, w);
    }
}
