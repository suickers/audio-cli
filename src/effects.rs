use std::collections::VecDeque;

pub fn round(n: f32, mut x: f32) -> f32 {
	x /= n;
	x = x.round();
	x * n
} 

pub fn bitcrush(sample: i16, depth: f32) -> i16 {
	let normalised = sample as f32 / 32768.0;
	let interval = 1.0 / depth;
	let rounded = round(interval, normalised);
	(rounded * 32767.0) as i16
}

pub fn downsample(sample: i16, depth: usize, i: usize, last: i16) -> (i16, i16) {
	if i % depth == 0 {
		(sample, sample)
	} else {
		(last, last)
	}
}

pub fn gain(sample: i16, value: f32) -> i16 {
	sample * value as i16
}

pub fn soft_clip(sample: i16, value: f32) -> i16 {
	let normalised = sample as f32 / 32768.0;
	let driven = normalised * value;
	let clipped = driven.tanh();
	(clipped * 32767.0) as i16
}

pub fn delay(
	sample: i16, 
	buffer: &mut VecDeque<i16>, 
	delay_samples: usize, 
	mix: f32
) -> i16 {
	let delayed = buffer.front().copied().unwrap_or(0);
	buffer.push_back(sample);
	if buffer.len() > delay_samples {
		buffer.pop_front();
	}

	let wet = (delayed as f32 * mix) as i16;
	sample.saturating_add(wet)
}
