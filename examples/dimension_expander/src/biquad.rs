use std::f32::consts::PI;
use std::f32;

use constants::SAMPLERATE;

pub struct BiQuad {
    a0: f32,
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
    z_1: f32,
    z_2: f32,
    y_1: f32,
    y_2: f32,
}

pub struct LPF(BiQuad);

impl BiQuad {
    pub fn new() -> BiQuad {
        BiQuad {
            a0: 0., a1: 0., a2: 0.,
            b1: 0., b2: 0.,
            z_1: 0., z_2: 0.,
            y_1: 0., y_2: 0.,
        }
    }

    #[inline(always)]
    pub fn process(&mut self, v: f32) -> f32 {
        let mut y_n = self.a0 * v + self.a1 * self.z_1 + self.a2 * self.z_2
            - self.b1 * self.y_1 - self.b2 * self.y_2;

        if y_n > 0. && y_n < 1.175494351e-38 {
            y_n = 0.;
        }
        if y_n < 0. && y_n > -1.175494351e-38 {
            y_n = 0.;
        }

        self.z_2 = self.z_1;
        self.z_1 = v;
        self.y_2 = self.y_1;
        self.y_1 = y_n;
        y_n
    }
    pub fn set_lpf_params(&mut self, f: f32) {
        // LPF biquad equations
        println!("cutoff: {}", f);
        let theta_c = 2. * PI * f / SAMPLERATE;
        let gamma = theta_c.cos() / (1. + theta_c.sin());

        self.a0 = (1. - gamma) / 2.;
        self.a1 = (1. - gamma) / 2.;
        self.a2 = 0.;
        self.b1 = -gamma;
        self.b2 = 0.;
    }

    pub fn set_lpf2_params(&mut self, f: f32, q: f32) {
        println!("cutoff: {}, q: {}", f, q);
        let theta_c = 2. * PI * f / SAMPLERATE;
        let d = 1. / (q + 0.001);
        println!("theta_c: {}, d: {}, q: {}", theta_c, d, q);
        let beta_numerator = 1. - ((d / 2.) * theta_c.sin());
        let beta_denominat = 1. + ((d / 2.) * theta_c.sin());
        let beta = 0.5 * beta_numerator / beta_denominat;
        let gamma = (0.5 + beta) * theta_c.cos();

        self.a0 = (0.5 + beta - gamma) / 2.;
        println!("set a0: {} to {}", self.a0, (0.5 + beta - gamma) / 2.);
        self.a1 = 0.5 + beta - gamma;
        self.a2 = (0.5 + beta - gamma) / 2.0;
        self.b1 = -2. * gamma;
        self.b2 = 2. * beta;
        println!("beta: {}, gamma: {}, d: {}", beta, gamma, d);

        println!("a0: {}, a1: {}, a2: {}, b1: {}, b2: {}\n",
                 self.a0, self.a1, self.a2,
                 self.b1, self.b2);
    }
}

