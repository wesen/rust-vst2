use std::f32::consts::PI;
use std::f32;

use constants::SAMPLERATE;

struct BiQuad {
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

impl BiQuad {
    fn new() -> BiQuad {
        BiQuad {
            a0: 0., a1: 0., a2: 0.,
            b1: 0., b2: 0.,
            z_1: 0., z_2: 0.,
            y_1: 0., y_2: 0.,
        }
    }

    fn process(&mut self, v: f32) -> f32 {
        let ret = self.a0 * v + self.a1 * self.z_1 + self.a2 * self.z_2
            - self.b1 * self.y_1 + self.b2 * self.y_2;
        self.z_2 = self.z_1;
        self.z_1 = v;
        self.y_2 = self.y_1;
        self.y_1 = ret;
        ret
    }

    fn set_lpf_params(&mut self, f: f32) {
        // LPF biquad equations

        let theta_c = 2. * PI * f / SAMPLERATE;
        let gamma = theta_c.cos() / (1. + theta_c.sin());

        self.a0 = (1. - gamma) / 2.;
        self.a1 = (1. - gamma) / 2.;
        self.a2 = 0.;
        self.b1 = - gamma;
        self.b2 = 0.;
    }
}

