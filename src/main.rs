use hound::{WavReader, WavWriter};
use anyhow::Error; 
use clap::{Parser, ValueEnum};
// use std::f32::consts::PI;

#[derive(Debug, Clone, ValueEnum)]
enum Effect {
	Gain,
	SoftClip,
	Delay,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	input: String,
	
	#[arg(short, long)]
	output: String,
	
	#[arg(value_enum)]
	effect: Effect,
	
	#[arg(short, long)]
	value: f32,
}

fn main() -> Result<(), Error> {

	let args = Args::parse();

	let mut reader = WavReader::open(args.input)?;
	let spec = reader.spec();
	let duration = reader.duration() as usize;
	let samples = reader.samples::<i16>();
	
	let mut writer = WavWriter::create(args.output, spec)?;
	
	match args.effect {
	
		Effect::Gain => {
			for sample in samples {
				let driven = (sample? as f32 * args.value) as i16;
				writer.write_sample(driven)?;
			}
		},

		Effect::SoftClip => {
			for sample in samples {
				let normalised = sample? as f32 / 32768.0;
				let driven = normalised * args.value;
				let clipped = driven.tanh();
				let output = (clipped * 32767.0) as i16;
				writer.write_sample(output)?;
			}
		},

		Effect::Delay => {
			let sample_rate = spec.sample_rate as usize;
			let delay_time = sample_rate / args.value as usize;
			let feedback_gain = 0.5;
			
			let input: Vec<i16> = reader.samples()
				.collect::<Result<Vec<_>, _>>()?;
			let mut output = Vec::with_capacity(duration * 2);
			
			for n in 0.. {
				
				let current = if n < duration {
					input[n] as f32 / 32768.0
				} else {
					0.0
				};
				
				let delayed = if n >= delay_time {
					output[n - delay_time] as f32 / 32768.0
				} else {
					0.0
				};

				let mixed = current + (delayed * feedback_gain);

				if n >= duration + (delay_time * 20) {
					println!("Stopped at {} samples, {}s", n, n / sample_rate);
					break;
				}
				
				output.push((mixed * 32767.0) as i16);
			}

			for sample in output {
				writer.write_sample(sample)?;
			}
		}
	}
	
	Ok(())
}
