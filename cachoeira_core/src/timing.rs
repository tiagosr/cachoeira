use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
	/// timespan since last frame, in seconds
	delta_seconds: f32,
	/// timespan since last frame
	delta_time: Duration,
	/// timespan 
	delta_real_seconds: f32,
	delta_real_time: Duration,
	fixed_seconds: f32,
	fixed_time: Duration,
	pub last_fixed_update: Instant,
	frame_number: u64,
	absolute_real_time: Duration,
	absolute_time: Duration,
	time_scale: f32,
}

impl Time {
	pub fn delta_seconds(&self) -> f32 { self.delta_seconds }
	pub fn delta_time(&self) -> Duration { self.delta_time }
	pub fn delta_real_seconds(&self) -> f32 { self.delta_real_seconds }
	pub fn delta_real_time(&self) -> Duration { self.delta_real_time }
	pub fn fixed_seconds(&self) -> f32 { self.fixed_seconds }
	pub fn fixed_time(&self) -> Duration { self.fixed_time }
	pub fn frame_number(&self) -> u64 { self.frame_number }
	pub fn last_fixed_update(&self) -> Instant { self.last_fixed_update }
	pub fn absolute_time(&self) -> Duration { self.absolute_time }
	pub fn absolute_time_seconds(&self) -> f64 {
		duration_to_secs_f64(self.absolute_time)
	}
	pub fn absolute_real_time(&self) -> Duration { self.absolute_real_time }
	pub fn absolute_real_time_seconds(&self) -> f64 {
		duration_to_secs_f64(self.absolute_real_time)
	}
	pub fn time_scale(&self) -> f32 { self.time_scale }
	
	pub fn set_delta_seconds(&mut self, secs: f32) {
		self.delta_seconds = secs * self.time_scale;
		self.delta_time = secs_to_duration(secs * self.time_scale);
		self.delta_real_seconds = secs;
		self.delta_real_time = secs_to_duration(secs);
		
		self.absolute_time += self.delta_time;
		self.absolute_real_time += self.delta_real_time;
	}

	pub fn set_delta_time(&mut self, duration: Duration) {
		self.delta_seconds = duration_to_secs(duration) * self.time_scale;
		self.delta_time = secs_to_duration(duration_to_secs(duration) * self.time_scale);
		self.delta_real_seconds = duration_to_secs(duration);
		self.delta_real_time = duration;

		self.absolute_time += self.delta_time;
		self.absolute_real_time += self.delta_real_time;
	}

	pub fn set_fixed_seconds(&mut self, secs: f32) {
		self.fixed_seconds = secs;
		self.fixed_time = secs_to_duration(secs);
	}

	pub fn set_fixed_time(&mut self, duration: Duration) {
		self.fixed_seconds = duration_to_secs(duration);
		self.fixed_time = duration;
	}

	pub fn increment_frame_number(&mut self) {
		self.frame_number += 1;
	}

	pub fn set_time_scale(&mut self, multiplier: f32) {
		use std::f32::INFINITY;
		assert!(multiplier >= 0.0);
		assert!(multiplier != INFINITY);
		self.time_scale = multiplier;
	}

	pub fn finish_fixed_update(&mut self) {
		self.last_fixed_update += self.fixed_time;
	}
}

impl Default for Time {
	fn default() -> Time {
		let fixed_timestep_duration = Duration::new(0, 16_666_666);
		Time {
			delta_seconds: 0.0,
			delta_time: Duration::from_secs(0),
			delta_real_seconds: 0.0,
			delta_real_time: Duration::from_secs(0),
			fixed_seconds: duration_to_secs(fixed_timestep_duration), // 1 fixed update at 60 hz
			fixed_time: fixed_timestep_duration,
			last_fixed_update: Instant::now(),
			frame_number: 0,
			absolute_real_time: Duration::default(),
			absolute_time: Duration::default(),
			time_scale: 1.0,
		}
	}
}


#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Stopwatch {
	Waiting,
	Started(Duration, Instant),
	Ended(Duration)
}

impl Default for Stopwatch {
	fn default() -> Stopwatch {
		Stopwatch::Waiting
	}
}

impl Stopwatch {
	pub fn new() -> Stopwatch {
		Default::default()
	}

	pub fn elapsed(&self) -> Duration {
		match *self {
			Stopwatch::Waiting => Duration::new(0, 0),
			Stopwatch::Started(dur, start) => dur + start.elapsed(),
			Stopwatch::Ended(dur) => dur,
		}
	}

	pub fn restart(&mut self) {
		*self = Stopwatch::Started(Duration::new(0, 0), Instant::now());
	}

	pub fn start(&mut self) {
		match *self {
			Stopwatch::Waiting => self.restart(),
			Stopwatch::Ended(dur) => {
				*self = Stopwatch::Started(dur, Instant::now());
			}
			_ => {}
		}
	}

	pub fn stop(&mut self) {
		if let Stopwatch::Started(dur, start) = *self {
			*self = Stopwatch::Ended(dur + start.elapsed());
		}
	}

	pub fn reset(&mut self) {
		*self = Stopwatch::Waiting;
	}
}

#[cfg(test)]
mod tests {
	use super::Stopwatch;
	use std::thread;
	use std::time::Duration;

	#[test]
	fn elapsed() {
	    const DURATION: u64 = 1;
	    const UNCERTAINTY: u32 = 10;
	    let mut watch = Stopwatch::new();

	    watch.start();
	    thread::sleep(Duration::from_secs(DURATION));
	    watch.stop();

	    let elapsed = watch.elapsed();
	    let duration = Duration::new(DURATION, 0);
	    let lower = duration / 100 * (100 - UNCERTAINTY);
	    let upper = duration / 100 * (100 + UNCERTAINTY);
	    assert!(
	    	elapsed < upper && elapsed > lower,
	    	"expected {} +- {}% seconds, got {:?}",
	    	DURATION,
	    	UNCERTAINTY,
	    	elapsed
	    );
	}

	#[test]
	fn reset() {
	    const DURATION: u64 = 2;
	    let mut watch = Stopwatch::new();

	    watch.start();
	    thread::sleep(Duration::from_secs(DURATION));
	    watch.stop();
	    watch.reset();

	    assert_eq!(0, watch.elapsed().subsec_nanos());
	}

	#[test]
	fn restart() {
	    const DURATION0: u64 = 2;
	    const DURATION: u64 = 1;
	    const UNCERTAINTY: u32 = 10;
	    let mut watch = Stopwatch::new();

	    watch.start();
	    thread::sleep(Duration::from_secs(DURATION0));
	    watch.stop();

	    watch.restart();
	    thread::sleep(Duration::from_secs(DURATION));
	    watch.stop();

	    let elapsed = watch.elapsed();
	    let duration = Duration::new(DURATION, 0);
	    let lower = duration / 100 * (100 - UNCERTAINTY);
	    let upper = duration / 100 * (100 + UNCERTAINTY);
	    assert!(
	    	elapsed < upper && elapsed > lower,
	    	"expected {} +- {}% seconds, got {:?}",
	    	DURATION,
	    	UNCERTAINTY,
	    	elapsed
	    );
	}
}

/// Utility functions for dealing with time units and precision

// Duration -> f32 (in seconds)
pub fn duration_to_secs(duration: Duration) -> f32 {
	duration.as_secs() as f32 + (duration.subsec_nanos() as f32 / 1.0e9)
}

// Duration -> f64 (in seconds)
pub fn duration_to_secs_f64(duration: Duration) -> f64 {
	duration.as_secs() as f64 + (duration.subsec_nanos() as f64 / 1.0e9)
}

// f32 (in seconds) -> Duration
pub fn secs_to_duration(secs: f32) -> Duration {
	Duration::new(secs as u64, ((secs % 1.0) * 1.0e9) as u32)
}

// Duration -> u64 (in nanoseconds)
pub fn duration_to_nanos(duration: Duration) -> u64 {
	(duration.as_secs() * 1_000_000_000) + duration.subsec_nanos() as u64
}

// u64 (in nanoseconds) -> Duration
pub fn nanos_to_duration(nanos: u64) -> Duration {
	Duration::new(nanos / 1_000_000_000, (nanos % 1_000_000_000) as u32)
}

