use dasp::signal::Signal;

pub struct Saw {
    frequency: f32,
    phase: i32,
}

impl Saw {
    pub fn new(frequency: f32) -> Self {
        Saw {
            frequency,
            phase: 0,
        }
    }
}

impl Signal for Saw {
    type Frame = i16;

    fn next(&mut self) -> i16 {
        let value = ((std::f32::consts::TAU * (self.phase as f32) * self.frequency)
                     / 44_100f32)
            .sin();

        self.phase += 1;
        (value * 32768f32) as i16
    }
}
