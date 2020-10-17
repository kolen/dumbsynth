use dasp::Signal;
use dumbsynth::Saw;
use std::fs::File;
use std::path::Path;

fn main() {
    let header = wav::Header::new(1, 1, 44_100, 16);
    let mut out_file = File::create(Path::new("output.wav")).unwrap();

    let saw = Saw::new(1_000f32);
    let data: Vec<i16> = saw.take(44_100).collect();

    wav::write(header, wav::BitDepth::Sixteen(data), &mut out_file).unwrap();
}
