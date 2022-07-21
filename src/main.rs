use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use rodio::{Decoder, OutputStream, Sink, Source};

fn main() {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let file = File::open("assets/discover_universe.mp3").unwrap();
    let buffer = BufReader::new(file);
    let mut source = Decoder::new(buffer).unwrap().buffered();

    let beats_per_minute = 130.0;
    let time_signature = 4.0 / 4.0;
    let beat_delta = 60.0 / beats_per_minute;
    let measure_delta = beat_delta * 4.0 * time_signature;

    let mut measure = 0;
    let reference = Instant::now();
    loop {
        if measure == 4 || measure == 12 {
            let beat = source
                .clone()
                .take_duration(Duration::from_secs_f64(beat_delta / 2.0))
                .repeat_infinite();

            let slice = beat.take_duration(Duration::from_secs_f64(measure_delta / 2.0));

            source = source
                .skip_duration(Duration::from_secs_f64(measure_delta / 2.0))
                .into_inner();

            let rest = source
                .clone()
                .take_duration(Duration::from_secs_f64(measure_delta / 2.0));

            source = source
                .skip_duration(Duration::from_secs_f64(measure_delta / 2.0))
                .into_inner();

            sink.append(slice);
            sink.append(rest);
        } else {
            let slice = source
                .clone()
                .take_duration(Duration::from_secs_f64(measure_delta));

            source = source
                .skip_duration(Duration::from_secs_f64(measure_delta))
                .into_inner();

            sink.append(slice);
        };

        if sink.empty() {
            break;
        }

        let offset = reference
            .elapsed()
            .saturating_sub(Duration::from_secs_f64(measure_delta) * measure);

        measure += 1;

        std::thread::sleep(Duration::from_secs_f64(measure_delta) - offset);
    }
}
