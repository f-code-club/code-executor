use std::time::Duration;

use byte_unit::Byte;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Verdict {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    CompilationError,
    MemoryLimitExceeded,
    RuntimeError,
    IdleTimeLimitExceeded,
}

#[derive(Debug, Clone)]
pub struct Metrics {
    pub verdict: Verdict,
    pub run_time: Duration,
    pub memory_usage: Byte,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AggregatedMetrics {
    pub verdict: Verdict,
    pub average_run_time: Duration,
    pub average_memory_usage: Byte,
}
