use hound::{WavReader, WavWriter};
use anyhow::Error; 
use clap::Parser;
use std::collections::VecDeque;

mod effects;
mod cli;

fn main() -> Result<(), Error> {
	use cli::Effect;
	
	let args = cli::Args::parse();
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
	
	Ok(())
}


