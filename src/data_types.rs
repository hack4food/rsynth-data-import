use std::io::{
    Error as IOError,
};

pub type RawWaveFrame = (u64, u64);
pub type RawWaveData = Vec<RawWaveFrame>;
pub type RawWaveDataResult = Result<RawWaveData, IOError>;

pub type WaveFrame = (u64, f64);
pub type WaveData = Vec<WaveFrame>;
pub type WaveDataResult = Result<WaveData, IOError>;