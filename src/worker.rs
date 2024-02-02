use crate::job::{Job, JobResult};
use std::sync::mpsc::Sender;
use std::thread;

#[derive(Copy, Clone, Debug)]
pub struct WorkerId(pub(crate) usize);

impl From<usize> for WorkerId {
    fn from(value: usize) -> Self {
        WorkerId(value)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Progress {
    pub cur: usize,
    pub end: usize,
}

impl Progress {
    fn new(end: usize) -> Self {
        Progress { cur: 0, end }
    }
}

pub struct Worker {
    worker_id: WorkerId,
    tx: Sender<WorkerMessage>,
}

pub enum WorkerMessage {
    Running(WorkerId, Job, Progress),
    Finished(WorkerId, Job, JobResult),
}

impl Worker {
    pub fn new(worker_id: WorkerId, tx: Sender<WorkerMessage>) -> Self {
        Worker { worker_id, tx }
    }

    pub fn run(&self, job: &Job) {
        let tx = self.tx.clone();
        let worker_id = self.worker_id;
        let job = job.clone();

        thread::spawn(move || {
            let requests_count = job.requests_count;
            let mut progress = Progress::new(requests_count);

            for _ in 0..requests_count {
                // Send progress message
                progress.cur += 1;
                _ = tx.send(WorkerMessage::Running(worker_id, job.clone(), progress));

                // Do some work...
                let delay = job.duration / requests_count as u32;
                thread::sleep(delay);
            }

            let result = JobResult::new();
            tx.send(WorkerMessage::Finished(worker_id, job, result))
        });
    }
}
