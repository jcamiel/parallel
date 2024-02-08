use crate::job::Job;
use crate::runner::WorkerState;
use crate::worker::{Progress, Worker};
use colored::Colorize;

pub struct Term {
    lines: usize,
}

impl Term {
    pub fn new() -> Self {
        Term { lines: 0 }
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

    /// Prints the progress bar with the workers `states`.
    pub fn print_progress(
        &mut self,
        states: &[(Worker, WorkerState)],
        completed: usize,
        count: usize,
    ) {
        self.lines = 0;

        // Computes maximum size of the string "[current request] / [nb of request]" to
        // left align th column.
        let max = states
            .iter()
            .map(|(_, state)| match state {
                WorkerState::Idle => 0,
                WorkerState::Running(_, Progress { end, .. }) => {
                    ((*end as f64).log10() as usize) + 1
                }
            })
            .max()
            .unwrap();
        let max_width = 2 * max + 1;

        let percent = (completed as f64 * 100.0 / count as f64) as usize;
        eprintln!("Executed files: {completed}/{count} ({percent}%)");
        self.lines += 1;

        for (_, state) in states.iter() {
            match state {
                WorkerState::Idle => {}
                WorkerState::Running(job, progress) => {
                    let name = job.name.bold();
                    let state = String::from("Running").cyan().bold();
                    let cur = progress.cur;
                    let end = progress.end;
                    let progress_str = progress_string(cur, end);
                    let requests = format!("{cur}/{end}");
                    let padding = " ".repeat(max_width - requests.len());
                    eprintln!("{progress_str} {requests}{padding} {name}: {state}");
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
    format!("[{completed}>{void}]")
}
