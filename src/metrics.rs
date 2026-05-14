use std::time::Duration;

use byte_unit::Byte;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Verdict {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    CompilationError,
    MemoryLimitExceeded,
    RuntimeError,
    IdleTimeLimitExceeded,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Metrics {
    pub verdict: Verdict,
    pub run_time: Duration,
    pub memory_usage: Byte,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
