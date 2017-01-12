#[macro_use] extern crate vst2;
extern crate time;

mod constants;
mod compressor;
mod biquad;
mod hpf_plugin;
mod hpf;
mod lpf_plugin;

use hpf_plugin::HPFPlugin;

plugin_main!(HPFPlugin);
