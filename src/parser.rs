use super::data_types::*;
use std::io::{
    Error as IOError,
    ErrorKind::{InvalidData, InvalidInput},
};
use std::str::Split;

pub fn process_line(res: RawWaveDataResult, line: &str) -> RawWaveDataResult {
    if is_empty(&line) {
        return res;
    }

    res.and_then(|mut acc| {
        parse_csv_line(line).map(|line_data| {
            acc.push(line_data);
            acc
        })
    })
}

fn is_empty<'r>(line: &'r &str) -> bool {
    line.trim().is_empty()
}

fn parse_csv_line(line: &str) -> Result<RawWaveFrame, IOError> {
    split_line(line).and_then(parse_wave_data)
}

fn split_line<'a>(line: &'a str) -> Result<(String, String), IOError> {
    let mut line_parts = line.split(",");
    Ok((
        next_line_part(&mut line_parts)?.to_string(),
        next_line_part(&mut line_parts)?.to_string(),
    ))
}

fn next_line_part<'a>(line_parts: &mut Split<'a, &str>) -> Result<&'a str, IOError> {
    line_parts.next().ok_or_else(|| IOError::from(InvalidInput))
}

fn parse_wave_data((a, b): (String, String)) -> Result<RawWaveFrame, IOError> {
    Ok((
        a.parse().or_else(|_| Err(IOError::from(InvalidData)))?,
        b.parse().or_else(|_| Err(IOError::from(InvalidData)))?,
    ))
}