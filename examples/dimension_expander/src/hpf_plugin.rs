use vst2::plugin::{Category, Info, Plugin};
use vst2::buffer::AudioBuffer;

use hpf::HighPassFilter;
use constants::SAMPLERATE;

/// The Compressor
pub struct HPFPlugin {
    highpass_right: HighPassFilter,
    highpass_left: HighPassFilter,
    cutoff: f32,
}

impl Default for HPFPlugin {
    fn default() -> HPFPlugin {
        HPFPlugin::new(6000.)
    }
}

impl HPFPlugin {
    fn new(cutoff: f32) -> HPFPlugin {
        HPFPlugin {
            highpass_left: HighPassFilter::new(),
            highpass_right: HighPassFilter::new(),
            cutoff: cutoff,
        }
    }
}

impl Plugin for HPFPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "HPFPlugin".to_string(),
            vendor: "slono".to_string(),
            unique_id: 243723072,
            version: 0001,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            category: Category::Effect,

            ..Default::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.cutoff,
            _ => 0.0,
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.1}", self.cutoff * SAMPLERATE),
            _ => "".to_string()
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Cutoff",
            _ => "",
        }.to_string()
    }

    fn set_parameter(&mut self, index: i32, val: f32) {
        match index {
            0 => {
                self.cutoff = val;
                let v = (self.cutoff / 2.) * (SAMPLERATE / 2.);
                self.highpass_left.set_frequency(v);
                self.highpass_right.set_frequency(v);
            },
            _ => (),
        }
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (inputs, mut outputs) = buffer.split();

        // Assume 2 channels
        if inputs.len() < 2 || outputs.len() < 2 {
            return;
        }

        // Iterate over inputs as (&f32, &f32)
        let stereo_in = match inputs.split_at(1) {
            (l, r) => l[0].iter().zip(r[0].iter())
        };

        // Iterate over outputs as (&mut f32, &mut f32)
        let stereo_out = match outputs.split_at_mut(1) {
            (l, r) => l[0].iter_mut().zip(r[0].iter_mut())
        };

        // Zip and process
        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            *left_out = self.highpass_left.process(*left_in);
            *right_out = self.highpass_right.process(*right_in);
        }
    }
}

