use crate::core::vec3::{Point3, Vec3};
use crate::sampling::random::random_int_range;
use rand::Rng;

#[derive(Debug)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let ranvec_count = 256;
        let mut ranvec = Vec::with_capacity(ranvec_count);
        let mut rng = rand::rng();

        for _ in 0..ranvec_count {
            let v = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            );
            ranvec.push(v.normalize());
        }

        let perm_x = Self::perlin_generate_perm(ranvec_count);
        let perm_y = Self::perlin_generate_perm(ranvec_count);
        let perm_z = Self::perlin_generate_perm(ranvec_count);

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        // Hermite smoothing
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::zeros(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn perlin_generate_perm(point_count: usize) -> Vec<i32> {
        let mut p: Vec<i32> = (0..point_count as i32).collect();
        Self::permute(&mut p, point_count);
        p
    }

    fn permute(p: &mut [i32], n: usize) {
        for i in (1..n).rev() {
            let target = random_int_range(0, i as i32) as usize;
            p.swap(i, target);
        }
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let ii = i as f64;
                    let jj = j as f64;
                    let kk = k as f64;

                    let weight_v = Vec3::new(u - ii, v - jj, w - kk);

                    accum += (ii * u + (1.0 - ii) * (1.0 - u))
                        * (jj * v + (1.0 - jj) * (1.0 - v))
                        * (kk * w + (1.0 - kk) * (1.0 - w))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }
}
