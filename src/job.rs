use std::time::Duration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    // Workload simulation: "run" a certain number of requests for a total specific duration
    pub duration: Duration,
    pub requests_count: usize,
    pub name: String,

    pub seq: usize, // the job index in the jobs list
    pub last_seq: usize, // the last index in the jobs list
}

#[derive(Debug)]
pub struct JobResult;

impl Job {
    pub fn new(name: &str, duration: Duration, requests_count: usize, seq: usize, last_seq: usize) -> Self {
        Job {
            name: name.to_string(),
            duration,
            requests_count,
            seq,
            last_seq,
        }
    }
}

impl JobResult {
    pub fn new() -> Self {
        JobResult {}
    }
}
