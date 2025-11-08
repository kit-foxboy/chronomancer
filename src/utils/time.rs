use std::fmt;
use crate::fl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeUnit::Seconds => write!(f, "{}", fl!("seconds")),
            TimeUnit::Minutes => write!(f, "{}", fl!("minutes")),
            TimeUnit::Hours => write!(f, "{}", fl!("hours")),
            TimeUnit::Days => write!(f, "{}", fl!("days")),
        }
    }
}