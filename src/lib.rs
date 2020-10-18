use dasp::signal::Signal;
#[macro_use]
extern crate vst;
use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{CanDo, Category, Info, Plugin};

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
    type Frame = f32;

    fn next(&mut self) -> f32 {
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
        value
    }
}

// TODO: use library
fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    ((f64::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

#[derive(Default)]
struct DumbsynthPlugin {
    note: Option<Saw>,
}

impl DumbsynthPlugin {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8) {
        let freq = midi_pitch_to_freq(note);
        self.note = Some(Saw::new(freq as f32));
    }

    fn note_off(&mut self, _note: u8) {
        self.note = None; // TODO: don't turn off if another key depressed
    }
}

impl Plugin for DumbsynthPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Dumbsynth".to_string(),
            unique_id: 1357,
            outputs: 2,
            inputs: 0,
            category: Category::Synth,
            parameters: 0,
            initial_delay: 0,

            ..Default::default()
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let n_samples = buffer.samples();
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();
        for sample_i in 0..n_samples {
            let output = self.note.as_mut().map_or(0.0, |n| n.next());
            for chan_i in 0..output_count {
                outputs.get_mut(chan_i)[sample_i] = output;
            }
        }
    }
}

plugin_main!(DumbsynthPlugin);
