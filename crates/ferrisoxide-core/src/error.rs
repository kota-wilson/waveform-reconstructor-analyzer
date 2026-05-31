use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, WaveformError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaveformError {
    EmptyInput,
    MissingColumn { column: String },
    InvalidNumber { column: String, value: String },
    Csv { message: String },
    MismatchedSampleCount { expected: usize, actual: usize },
    InvalidWaveform { reason: String },
    InvalidParameter { name: String, reason: String },
    ReportSerialization { message: String },
    NotImplemented { feature: String },
}

impl Display for WaveformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "input contains no waveform samples"),
            Self::MissingColumn { column } => write!(f, "missing required column `{column}`"),
            Self::InvalidNumber { column, value } => {
                write!(f, "invalid numeric value `{value}` in column `{column}`")
            }
            Self::Csv { message } => write!(f, "csv parse error: {message}"),
            Self::MismatchedSampleCount { expected, actual } => {
                write!(
                    f,
                    "mismatched sample count: expected {expected}, got {actual}"
                )
            }
            Self::InvalidWaveform { reason } => write!(f, "invalid waveform: {reason}"),
            Self::InvalidParameter { name, reason } => {
                write!(f, "invalid parameter `{name}`: {reason}")
            }
            Self::ReportSerialization { message } => {
                write!(f, "report serialization error: {message}")
            }
            Self::NotImplemented { feature } => write!(f, "{feature} is not implemented yet"),
        }
    }
}

impl std::error::Error for WaveformError {}
