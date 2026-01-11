use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum Effect {
	Gain,
	SoftClip,
	Delay,
	Downsampler,
	Bitcrusher
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	#[arg(short, long)]
	pub input: String,
	
	#[arg(short, long)]
	pub output: String,
	
	#[arg(value_enum, short, long)]
	pub effect: Effect,
	
	#[arg(short, long)]
	pub value: f32,
}
