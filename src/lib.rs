use dasp::signal::Signal;

const SAMP_RATE: i32 = 44_100;

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
        let mut value = 0f32;

        let mut harm_f = self.frequency;
        let mut harm_i = 1;

        while harm_f < ((SAMP_RATE as f32) / 2f32) {
            let polarity = if harm_i % 2 == 0 { 1f32 } else { -1f32 };
            value += polarity
                * (((std::f32::consts::TAU * (self.phase as f32) * harm_f) / (SAMP_RATE as f32))
                    .sin())
                / harm_i as f32;

            harm_i += 1;
            harm_f = self.frequency * (harm_i as f32);
        }

        value = -(std::f32::consts::FRAC_1_PI * value);

        self.phase += 1;
        (value * 32768f32) as i16
    }
}
