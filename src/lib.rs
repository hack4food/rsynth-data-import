use std::io::{
    Error as IOError,
    ErrorKind::{InvalidData, InvalidInput},
};

type WaveFrame = (u64, f32);
type WaveData = Vec<WaveFrame>;
type WaveDataResult = Result<WaveData, IOError>;

pub fn open_and_parse_csv(csv_path: &str) -> WaveDataResult {
    std::fs::read_to_string(csv_path)?
        .lines()
        .filter(is_non_empty)
        .fold(Ok(vec![]), process_line)
}

fn is_non_empty<'r>(line: &'r &str) -> bool {
    !line.trim().is_empty()
}

fn process_line(res: WaveDataResult, line: &str) -> WaveDataResult {
    res.and_then(|mut acc| {
        parse_csv_line(line).map(|line_data| {
            acc.push(line_data);
            acc
        })
    })
}

fn parse_csv_line(line: &str) -> Result<WaveFrame, IOError> {
    tokenise_line(line).and_then(parse_wave_data)
}

fn tokenise_line<'a>(line: &'a str) -> Result<(String, String), IOError> {
    let mut line_parts = line.split(",");
    let line_ts_raw = next_line_part(&mut line_parts)?;
    let line_amp_raw = next_line_part(&mut line_parts)?;
    Ok((line_ts_raw.to_string(), line_amp_raw.to_string()))
}

fn next_line_part<'a>(line_parts: &mut std::str::Split<'a, &str>) -> Result<&'a str, IOError> {
    line_parts.next().ok_or_else(|| IOError::from(InvalidInput))
}

fn parse_wave_data((a, b): (String, String)) -> Result<WaveFrame, IOError> {
    let a_val = a.parse().or_else(|_| Err(IOError::from(InvalidData)))?;
    let b_val = b.parse().or_else(|_| Err(IOError::from(InvalidData)))?;
    Ok((a_val, b_val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api() {
        assert_eq!(
            open_and_parse_csv("asset/test_data.csv").unwrap(),
            vec!(
                (1554451200000, 10.0),
                (1554454800000, 25.0),
                (1554458400000, 25.0),
                (1554462000000, 22.0)
            )
        );
    }

    #[test]
    fn verify_bad_line_errors() {
        assert!(open_and_parse_csv("asset/test_data_malformed_line.csv").is_err());
    }

    #[test]
    fn verify_bad_data_errors() {
        assert!(open_and_parse_csv("asset/test_data_malformed_data.csv").is_err());
    }
}
