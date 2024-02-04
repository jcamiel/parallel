mod job;
mod runner;
mod term;
mod worker;

use crate::job::Job;
use crate::runner::ParallelRunner;
use std::time::Duration;

fn main() {
    let jobs = [
        // name, duration in s, entry count
        ("/tmp/foo/bar/baz/job-1.hurl", 10, 10),
        ("/tmp/foo/bar/job-2.hurl", 3, 2),
        ("/tmp/foo/bar/zzzzzz/job-3.hurl", 7, 3),
        ("/tmp/foo/bar/job-4.hurl", 4, 7),
        ("/tmp/foo/bar/ddd/job-5.hurl", 8, 12),
        ("/tmp/foo/bar/job-6.hurl", 1, 4),
        ("/tmp/foo/bar/ee/job-7.hurl", 6, 1),
        ("/tmp/foo/bar/fff/job-8.hurl", 9, 5),
        ("/tmp/foo/bar/job-9.hurl", 5, 10),
        ("/tmp/foo/bar/job-10.hurl", 4, 3),
    ];
    let last_seq = jobs.len() -1;
    let jobs = jobs
        .into_iter()
        .enumerate()
        .map(|(seq, (name, duration, count))| {
            Job::new(name, Duration::from_secs(duration), count, seq, last_seq)
        })
        .collect::<Vec<_>>();
    let worker_count = 5;
    let mut runner = ParallelRunner::new(worker_count);
    let _ = runner.run(&jobs);
}
