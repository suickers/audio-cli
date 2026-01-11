use hound::{WavReader, WavWriter};
use anyhow::Error; 
use clap::Parser;
use std::collections::VecDeque;
use std::f32::consts::PI; 

mod effects;
mod cli;

fn main() -> Result<(), Error> {
	use cli::{Commands, Cli};
	
	let args = Cli::parse();
	
	
	match args.command {

		Commands::Fx(args) => {
			use cli::Effect;
			
			let mut reader = WavReader::open(args.input)?;
			let spec = reader.spec();
			let samples = reader.samples::<i16>();
			let mut writer = WavWriter::create(args.output, spec)?;

			
			match args.effect {
				Effect::Gain => {
					for sample in samples {
						let driven = effects::gain(sample?, args.value);
						writer.write_sample(driven)?;
					}
				},
		
				Effect::SoftClip => {
					for sample in samples {
						let clipped = effects::soft_clip(sample?, args.value);
						writer.write_sample(clipped)?;
					}
				},
		
				Effect::Delay => {
					let mut buffer = VecDeque::new();
					let delay_samples = (0.5 * spec.sample_rate as f32) as usize;
					for sample in samples {
						let output = effects::delay(sample?, &mut buffer, delay_samples, args.value);
						writer.write_sample(output)?;
					}
				},
		
				Effect::Downsampler => {
					let mut last_sample = 0;
					for (i, sample) in samples.enumerate() {
						let (out, last) = effects::
							downsample(sample?, args.value as usize, i, last_sample);
						last_sample = last;
						writer.write_sample(out)?;
					}
				},
		
				Effect::Bitcrusher => {
					for sample in samples {
						let processed = effects::bitcrush(sample?, args.value);
						writer.write_sample(processed)?;
					}	
				},
			}
		},


		Commands::Osc(args) => {
			use cli::Wave; 
			
			let spec = hound::WavSpec {
				channels: 1, 
				sample_rate: 44100, 
				bits_per_sample: 16, 
				sample_format: hound::SampleFormat::Int,
			};
			
			let mut writer = WavWriter::create(args.output, spec)?;

			let tau = 2.0 * PI;
			let amp = i16::MAX;
			let sample_rate = spec.sample_rate as f32;
			let total_samples = (sample_rate * args.duration) as usize;
			
			match args.wave {
				Wave::Sine => {
					for t in (0..total_samples).map(|t| t as f32 / sample_rate) {
						let sample = (tau * args.freq * t).sin() ;
						let output = (sample * amp as f32) as i16;
						writer.write_sample(output)?;
					}
					writer.finalize()?;
				},
				_ => unimplemented!("Add other waves.")
			}
		}
	}
	
	Ok(())
}


