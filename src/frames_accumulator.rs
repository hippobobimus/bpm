use std::time::Duration;

#[derive(Debug)]
pub struct FramesAccumulator {
    frames: Duration,
    fps: u32,
}

impl FramesAccumulator {
    pub fn new(fps: u32) -> Self {
        Self { 
            frames: Duration::new(0, 0),
            fps,
        }
    }

    /// Moves the frame accumulator forward by the given Duration.
    pub fn increment(&mut self, inc_duration: Duration) {
        let frames_elapsed = inc_duration * self.fps;
        self.frames += frames_elapsed;
    }

    pub fn get_whole_frames(&self) -> u64 {
        self.frames.as_secs()
    }

    pub fn reset_whole_frames(&mut self) {
        self.frames = Duration::new(0, self.frames.subsec_nanos());
    }

    pub fn get_sub_frames(&self) -> u32 {
        self.frames.subsec_nanos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fa = FramesAccumulator::new(5);

        assert_eq!(fa.get_whole_frames(), 0);
        assert_eq!(fa.get_sub_frames(), 0);
    }

    #[test]
    fn test_increment() {
        let fps = 5;  // frames per second.
        let mut fa = FramesAccumulator::new(fps);

        // move forward by 1s, equivalent to 5 frames exactly.
        fa.increment(Duration::new(1, 0));

        assert_eq!(fa.get_whole_frames(), 5);
        assert_eq!(fa.get_sub_frames(), 0);
    }

    #[test]
    fn test_reset_whole_frames() {
        let fps = 5;  // frames per second.
        let mut fa = FramesAccumulator::new(fps);

        fa.increment(Duration::new(1, 123));

        assert_eq!(fa.get_whole_frames(), 5);

        let sub_frames = fa.get_sub_frames();

        fa.reset_whole_frames();

        assert_eq!(fa.get_whole_frames(), 0);
        assert_eq!(fa.get_sub_frames(), sub_frames);
    }
}
