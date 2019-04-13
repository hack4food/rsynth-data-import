use std::io::Error as IOError;

type WaveFrame = (u64, f32);
type WaveData = Vec<WaveFrame>;
type WaveDataResult = Result<WaveData, IOError>;

pub fn open_and_parse_csv(csv_path: &str) -> WaveDataResult {
    let contents = std::fs::read_to_string(csv_path)?;
    parse_csv(contents)
}

fn parse_csv(contents: String) -> Result<WaveData, IOError> {
    let acc = Ok(vec![]);
    contents
        .lines()
        .filter(is_non_empty)
        .fold(acc, parse_csv_line)
}

fn parse_csv_line(res: WaveDataResult, line: &str) -> WaveDataResult {
    match res {
        Ok(mut acc) => match tokenise_line(line).map(parse_wave_data) {
            Ok(line_data) => {
                acc.push(line_data);
                Ok(acc)
            }
            Err(e) => Err(e),
        },
        Err(_) => res,
    }
}

fn is_non_empty<'r>(line: &'r &str) -> bool {
    !line.trim().is_empty()
}

fn tokenise_line<'a>(line: &'a str) -> Result<(String, String), IOError> {
    let mut line_parts = line.split(",");
    let line_ts_raw = line_parts
        .next()
        .ok_or_else(|| IOError::from(std::io::ErrorKind::InvalidInput))?;
    let line_amp_raw = line_parts
        .next()
        .ok_or_else(|| IOError::from(std::io::ErrorKind::InvalidInput))?;
    Ok((line_ts_raw.to_string(), line_amp_raw.to_string()))
}

fn parse_wave_data((a, b): (String, String)) -> WaveFrame {
    (a.parse().unwrap(), b.parse().unwrap())
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
    fn test_api_errors() {
        assert!(open_and_parse_csv("asset/test_data_malformed.csv").is_err());
    }
}
