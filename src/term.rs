use crate::job::Job;
use crate::runner::WorkerState;
use crate::worker::Worker;
use colored::Colorize;

pub struct Term {
    lines: usize,
}

impl Term {
    pub fn new() -> Self {
        Term {
            lines: 0,
        }
    }

    pub fn clear_progress(&self) {
        (0..self.lines).for_each(|_| eprint!("\x1B[1A\x1B[K"));
    }

    pub fn print_finished(&self, job: &Job) {
        let requests_count = job.requests_count;
        let job = job.name.bold();
        let state = String::from("Success").green().bold();
        eprintln!("{job}: {state} ({requests_count} request(s) in x ms)");
    }

    pub fn print_progress(&mut self, states: &[(Worker, WorkerState)]) {
        self.lines = 0;
        for (i, (_, state)) in states.iter().enumerate() {
            match state {
                // WorkerState::Idle => {
                //     eprintln!("#{} -: Idle", i + 1);
                //     self.lines += 1;
                // },
                WorkerState::Idle => { },
                WorkerState::Running(job, progress) => {
                    let name = job.name.bold();
                    let state = String::from("Running").cyan().bold();
                    let worker_id = i + 1;
                    let seq = job.seq + 1;
                    let last_seq = job.last_seq + 1;
                    let cur = progress.cur;
                    let end = progress.end;
                    let progress_str = progress_string(cur, end);
                    eprintln!("#{worker_id} {progress_str} {name}: {state} [{seq}/{last_seq}]");
                    // eprintln!("#{worker_id} {name}: {state} [{seq}/{last_seq}] {progress_str}");
                    self.lines += 1;
                }
            }
        }
    }
}


fn progress_string(cur: usize, end: usize) -> String {
    const WIDTH: usize = 24;
    let progress = (cur - 1) as f64 / end as f64;
    let col = (progress * WIDTH as f64) as usize;
    let completed = if col > 0 {
        "=".repeat(col)
    } else {
        String::new()
    };
    let void = " ".repeat(WIDTH - col - 1);
    format!("[{completed}>{void}] {cur}/{end}")
}