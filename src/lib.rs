mod parser;
mod data_types;

use std::path::Path;
use data_types::*;

pub fn load_csv_data(file_path: &Path) -> WaveDataResult {
    open_and_parse_csv(file_path).map(normalise_values)
}

fn open_and_parse_csv(csv_path: &Path) -> RawWaveDataResult {
    std::fs::read_to_string(csv_path)?
        .lines()
        .fold(Ok(vec![]), parser::process_line)
}

// TODO: introduce fn normalise_timepoints (or something)

fn normalise_values(raw_data: RawWaveData) -> WaveData {
    let (min, max) = raw_data.iter()
        .fold((&0u64, &0u64), |(min, max), (_time, amplitude)| {
            match amplitude {
                amp if amp < min => (amp, max),
                amp if amp > max => (min, amp),
                _ => (min, max)
            }
        });
    let amp_range = (max - min) as f64;
    raw_data.iter().map(|(t, a)| (*t, ((*a as f64 - min)/amp_range) - 0.5f64)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Write test for normalize cases

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
