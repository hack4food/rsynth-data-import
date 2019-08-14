extern crate plotlib;

mod data_types;
mod parser;

use data_types::*;
use std::cmp::{max, min};
use std::path::Path;

use plotlib::style::Line;

pub fn load_csv_data(file_path: &Path) -> WaveDataResult {
    open_and_parse_csv(file_path)
        .map(normalise_values)
        .map(plot_values)
}

fn open_and_parse_csv(csv_path: &Path) -> RawWaveDataResult {
    std::fs::read_to_string(csv_path)?
        .lines()
        .fold(Ok(vec![]), parser::process_line)
}

// TODO: introduce fn normalise_timepoints (or something)

fn normalise_values(raw_data: RawWaveData) -> WaveData {
    let (amp_min, amp_max) = compute_amp_range(&raw_data);
    let time_start = raw_data.get(0).unwrap().0;
    let time_end = raw_data.get(raw_data.len() - 1).unwrap().0;
    let amp_range = (amp_max - amp_min) as f64;

    raw_data
        .iter()
        .map(|frame| normalise_frame(frame, time_start, *amp_min as f64, amp_range))
        .collect()
}

fn compute_amp_range(raw_data: &RawWaveData) -> (&u64, &u64) {
    raw_data
        .iter()
        .fold((&0u64, &0u64), |(a_min, a_max), (_time, amp)| {
            (min(a_min, amp), max(a_max, amp))
        })
}

fn normalise_frame(
    raw_frame: &RawWaveFrame,
    time_start: u64,
    amp_min: f64,
    amp_range: f64,
) -> WaveFrame {
    (
        raw_frame.0 - time_start,
        ((raw_frame.1 as f64 - amp_min) / amp_range) - 0.5f64,
    )
}

fn plot_values(wave_data: WaveData) -> WaveData {
    let mut hand_line_style = plotlib::line::Style::new();
    hand_line_style.colour("#ff0000");

    let plot_points: Vec<(f64, f64)> = wave_data.iter().map(|(t, a)| (*t as f64, *a)).collect();
    let points_line = plotlib::line::Line::new(plot_points.as_slice()).style(&hand_line_style);

    let x_min = wave_data.get(0).unwrap().0 as f64;
    let x_max = wave_data.get(wave_data.len() - 1).unwrap().0 as f64;

    let v = plotlib::view::ContinuousView::new()
        .add(&points_line)
        .x_range(x_min, x_max)
        .y_range(-0.5, 0.5)
        .x_label("Time")
        .y_label("Amplitude");

    let plot_save = plotlib::page::Page::single(&v).save("wave.svg");

    wave_data
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Write test for normalize cases

    #[test]
    fn plot_data_csv() {
        load_csv_data("asset/data.csv".as_ref());
    }

    #[test]
    fn verify_successful_csv_reading() {
        assert_eq!(
            open_and_parse_csv("asset/test_data_valid.csv".as_ref()).unwrap(),
            vec!(
                (1554451200000, 10),
                (1554454800000, 25),
                (1554458400000, 25),
                (1554462000000, 22)
            )
        );
    }

    #[test]
    fn verify_bad_line_errors() {
        assert!(open_and_parse_csv("asset/test_data_malformed_line.csv".as_ref()).is_err());
    }

    #[test]
    fn verify_bad_data_errors() {
        assert!(open_and_parse_csv("asset/test_data_malformed_data.csv".as_ref()).is_err());
    }
}
