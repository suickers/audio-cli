use clap::{Args, Parser, ValueEnum, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	Fx(FxArgs),
	Osc(OscArgs),
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Effect {
	Gain,
	SoftClip,
	Delay,
	Downsampler,
	Bitcrusher
}

#[derive(Args)]
pub struct FxArgs {
	#[arg(short, long)]
	pub input: String,
	
	#[arg(short, long)]
	pub output: String,
	
	#[arg(value_enum, short, long)]
	pub effect: Effect,
	
	#[arg(short, long)]
	pub value: f32,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Wave {
	Sine,
	Square,
	Triangle,
}

#[derive(Args)]
pub struct OscArgs {
	#[arg(short, long)]
	pub output: String, 
	
	#[arg(value_enum, short, long)]
	pub wave: Wave,
	
	#[arg(short, long)]
	pub freq: f32,

	#[arg(short, long)]
	pub duration: f32,
}
