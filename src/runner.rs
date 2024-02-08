use crate::job::{Job, JobResult};
use crate::term::Term;
use crate::worker::{Progress, Worker, WorkerId, WorkerMessage};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::Instant;

pub enum WorkerState {
    Idle,
    Running(Job, Progress),
}

pub struct ParallelRunner {
    workers: Vec<(Worker, WorkerState)>,
    rx: Receiver<WorkerMessage>,
    _clock: Instant,
}

impl ParallelRunner {
    /// Creates a new parallel runner, with `worker_count`worker.
    pub fn new(worker_count: usize) -> Self {
        // Create the channel to communicate from worker to job runner (job runner is running on
        // main thread)
        let (tx, rx) = mpsc::channel();
        let clock = Instant::now();
        let workers = (0..worker_count)
            .map(|i| {
                let worker = Worker::new(WorkerId::from(i), tx.clone());
                let state = WorkerState::Idle;
                (worker, state)
            })
            .collect();
        ParallelRunner {
            workers,
            rx,
            _clock: clock,
        }
    }

    /// Runs a list of `jobs`.
    pub fn run(&mut self, jobs: &[Job]) -> Vec<JobResult> {
        // Construct our jobs queue
        let mut jobs = jobs.iter().rev().collect::<Vec<_>>();
        let jobs_count = jobs.len();

        // Initiate the runner, fill our workers
        for i in 0..self.workers.len() {
            let job = jobs.pop().unwrap();
            self.workers[i].0.run(job);
        }

        // Start the message pup
        let mut results = vec![];
        let mut term = Term::new();

        for msg in self.rx.iter() {
            term.clear_progress();
            match msg {
                WorkerMessage::Running(id, job, progress) => {
                    self.workers[id.0].1 = WorkerState::Running(job, progress);
                }
                WorkerMessage::Finished(id, job, result) => {
                    results.push(result);

                    term.print_finished(&job);

                    // Run a new job?
                    let job = jobs.pop();
                    match job {
                        Some(job) => self.workers[id.0].0.run(job),
                        None => {
                            self.workers[id.0].1 = WorkerState::Idle;
                            // Do we have received all the result?
                            if results.len() == jobs_count {
                                break;
                            }
                        }
                    }
                }
            }

            term.print_progress(&self.workers, results.len(), jobs_count)
        }
        results
    }
}
