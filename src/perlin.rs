use crate::{Float, Vec3};
use rand::prelude::*;

// TODO: This might be currently oddly broken and resulting in overflowy surfaces

pub struct Perlin {
    random_vectors: Vec<Vec3>,
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

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
    let uu: Float = u * u * (3.0 - 2.0 * u);
    let vv: Float = v * v * (3.0 - 2.0 * v);
    let ww: Float = w * w * (3.0 - 2.0 * w);
    let mut accum: Float = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f = i as Float;
                let j_f = j as Float;
                let k_f = k as Float;
                let weight_v: Vec3 = Vec3::new(u - i_f, v - j_f, w - k_f);
                accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                    * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                    * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                    * c[i][j][k].dot(&weight_v);
            }
        }
    }
    return accum;
}

impl Perlin {
    pub fn new(point_count: usize, mut rng: ThreadRng) -> Self {
        let mut random_vectors: Vec<Vec3> = Vec::with_capacity(point_count);
        for _i in 0..point_count {
            random_vectors.push(rng.gen::<Vec3>());
        }

        let perm_x = perlin_generate_perm(point_count, rng);
        let perm_y = perlin_generate_perm(point_count, rng);
        let perm_z = perlin_generate_perm(point_count, rng);

        Perlin {
            random_vectors,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: Vec3) -> Float {
        let u: Float = point.x - point.x.floor();
        let v: Float = point.y - point.y.floor();
        let w: Float = point.z - point.z.floor();
        // Hermitian cubic smoothing
        let u: Float = u * u * (3.0 - 2.0 * u);
        let v: Float = v * v * (3.0 - 2.0 * v);
        let w: Float = w * w * (3.0 - 2.0 * w);

        let i: usize = point.x.floor() as usize;
        let j: usize = point.y.floor() as usize;
        let k: usize = point.z.floor() as usize;

        let mut c: [[[Vec3; 2]; 2]; 2] = [
            [
                [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
                [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
            ],
            [
                [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
                [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
            ],
        ];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vectors[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }
        return perlin_interp(c, u, v, w);
    }

    pub fn turbulence(&self, position: Vec3, depth: usize) -> Float {
        let mut accum: Float = 0.0;
        let mut temp_p: Vec3 = position;
        let mut weight: Float = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        return accum.abs();
    }
}