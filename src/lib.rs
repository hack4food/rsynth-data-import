extern crate plotlib;

pub mod data_types;
mod parser;

pub use data_types::*;
use std::cmp::{max, min};
use std::path::Path;

use plotlib::style::Line;

pub fn load_csv_data(file_path: &Path) -> WaveDataResult {
    open_and_parse_csv(file_path)
        .map(normalise_values)
        .map(gather_zero_crossings)
        .map(plot_values)
}

fn open_and_parse_csv(csv_path: &Path) -> RawWaveDataResult {
    std::fs::read_to_string(csv_path)?
        .lines()
        .fold(Ok(vec![]), parser::process_line)
}

fn normalise_values(raw_data: RawWaveData) -> WaveData {
    let time_start = raw_data.get(0).unwrap().0;
    let time_end = raw_data.get(raw_data.len() - 1).unwrap().0;
    let time_duration = time_end - time_start;

    let (amp_min, amp_max) = compute_amp_range(&raw_data);
    let amp_range = (amp_max - amp_min) as f64;

    raw_data
        .iter()
        .map(|f| {
            let t = (f.0 - time_start) as f64 / time_duration as f64;
            let a = ((f.1 as f64 - amp_min as f64) * 2. / amp_range) - 1.;

            (t, a)
        })
        .collect()
}

fn compute_amp_range(raw_data: &RawWaveData) -> (u64, u64) {
    let limit = raw_data.get(0).unwrap().1;
    raw_data
        .iter()
        .fold((limit, limit), |(a_min, a_max), (_time, amp)| {
            (min(a_min, *amp), max(a_max, *amp))
        })
}

#[derive(Debug)]
struct DerivedCrossing<T> {
    before: T,
    crossing: T,
    after: T,
}

fn gather_zero_crossings(wave_data: WaveData) -> WaveData {
    let _ = wave_data
        .iter()
        .zip(wave_data.iter().skip(1))
        .filter(|((_t1, a1), (_t2, a2))| {
            *a2 as f64 == 0.
                || ((*a1 as f64) <= 0. && 0. < *a2 as f64)
                || ((*a1 as f64 >= 0.) && 0. > *a2 as f64)
        })
        .map(|((t1, a1), (t2, _a2))| {
            if *a1 == 0. {
                DerivedCrossing {
                    before: *t1,
                    after: *t1,
                    crossing: *t1,
                }
            } else {
                DerivedCrossing {
                    before: *t1,
                    after: *t2,
                    // TODO: find a better zero-point! (think: kx+m ;)
                    crossing: (*t1 + *t2) / 2.,
                }
            }
        })
        .collect::<Vec<DerivedCrossing<f64>>>();

    wave_data
}

fn plot_values(wave_data: WaveData) -> WaveData {
    let mut hand_line_style = plotlib::line::Style::new();
    hand_line_style.colour("#ff0000");
    hand_line_style.width(1.);

    let plot_points: Vec<(f64, f64)> = wave_data.iter().map(|(t, a)| (*t as f64, *a)).collect();
    let points_line = plotlib::line::Line::new(plot_points.as_slice()).style(&hand_line_style);

    let x_min = wave_data.get(0).unwrap().0 as f64;
    let x_max = wave_data.get(wave_data.len() - 1).unwrap().0 as f64;

    let v = plotlib::view::ContinuousView::new()
        .add(&points_line)
        .x_range(x_min, x_max)
        // NOTE: should this be dynamic? probably not ... but maybe?
        .y_range(-0.5, 0.5)
        .x_label("Time")
        .y_label("Amplitude");

    let _plot_save = plotlib::page::Page::single(&v).save("wave.svg");

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
