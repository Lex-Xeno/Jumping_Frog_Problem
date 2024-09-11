use crossterm::event::{self, poll, read, Event};
use std::{fs::File, io::Write, sync::mpsc, thread, time::Duration};

pub struct FrogManager {
    total_frogs: usize,
}

impl FrogManager {
    /// Create a new frog manager
    pub fn new(total_frogs: usize) -> FrogManager {
        FrogManager { total_frogs }
    }

    /// Starts simulating the given number of frogs
    pub fn simulate(&mut self, threads: usize) {
        let (result_trans, result_rec) = mpsc::channel();
        let mut workers = (Vec::with_capacity(threads), Vec::with_capacity(threads));
        let mut results =
            File::create("results.csv").expect("Failed to create 'results.csv' file!");
        results
            .write_all("Total Jumps,Furthest distance\n".as_bytes())
            .expect("Failed to write to 'results.csv'!");

        for id in 0..threads {
            let result_trans = result_trans.clone();
            let (jobs_trans, jobs_rec) = mpsc::channel();
            workers.1.push(jobs_trans);

            workers.0.push(thread::spawn(move || {
                while jobs_rec.recv().unwrap_or_else(|error| {
                    panic!(
                        "Worker {} failed to receive job signal! Error: {}",
                        id, error
                    )
                }) {
                    let mut frog = Frog::start(None);

                    while frog.jump().is_none() {}
                    result_trans.send((id, frog)).unwrap_or_else(|error| {
                        panic!(
                            "Worker {} failed to send finished frog! Error: {}",
                            id, error
                        )
                    });
                }
            }))
        }
        drop(result_trans);

        for worker in &workers.1 {
            worker.send(true).unwrap();
        }

        cliclack::log::info("Press <Enter> to finish simulating the current frogs and exit.")
            .unwrap();
        let bar = cliclack::progress_bar(self.total_frogs as u64);
        bar.start("Simulating frogs");

        for _ in 0..self.total_frogs - threads {
            let (worker_id, frog) = result_rec.recv().unwrap();
            bar.inc(1);

            if poll(Duration::from_micros(50)).unwrap() {
                match read().unwrap() {
                    Event::Key(key) => match key.code {
                        event::KeyCode::Enter => {
                            break;
                        }
                        _ => continue,
                    },
                    _ => continue,
                }
            }

            results
                .write_all(frog.csv_results().as_bytes())
                .expect("Failed to write to 'results.csv'!");
            workers.1[worker_id].send(true).unwrap();
        }
        bar.stop("Simulating finished!");

        for worker in &workers.1 {
            worker.send(false).unwrap();
        }

        let bar = cliclack::progress_bar(threads as u64);
        bar.start("Waiting for the last frogs, this might take a while...");
        for worker in workers.0 {
            worker.join().unwrap();
            bar.inc(1);
        }
        bar.stop("Remaining frogs have finished!");

        while let Ok((_, frog)) = result_rec.recv() {
            results
                .write_all(frog.csv_results().as_bytes())
                .expect("Failed to write to 'results.csv'!");
        }
    }
}

use indoc::indoc;
use rand::random;

/// Represents a frog
/// # Fields
/// - `id` is an ID given to each frog to distinguish them from one and other if multiple frogs are used
/// - `position` represents the lilly pad that the frog is currently on
/// - `jumps` is the number of jumps the frog has made
/// - `distance` - is the number of the furthest lilly pad that the frog has jumped to
pub struct Frog {
    id: Option<usize>,
    position: isize,
    jumps: usize,
    distance: isize,
    heading: FrogHeading,
}

/// Enums that represent which side of the center the frog jumped to
pub enum FrogHeading {
    Left,
    Right,
}

impl std::fmt::Display for FrogHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrogHeading::Left => write!(f, "left"),
            FrogHeading::Right => write!(f, "right"),
        }
    }
}
impl std::clone::Clone for FrogHeading {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for FrogHeading {}

impl Frog {
    /// Creates a new frog, sets its heading, and gives it an ID number
    pub fn start(id: Option<usize>) -> Frog {
        let heading = if random() {
            FrogHeading::Left
        } else {
            FrogHeading::Right
        };

        Frog {
            id,
            position: 1,
            jumps: 1,
            distance: 1,
            heading,
        }
    }
    /// Makes the frog jump
    pub fn jump(&mut self) -> Option<()> {
        self.position += if random() { -1 } else { 1 };
        self.jumps += 1;

        if self.distance < self.position {
            self.distance = self.position
        }

        if self.position == 0 {
            Some(())
        } else {
            None
        }
    }

    /// Returns the frogs's ID, total number of jumps, heading, and current position
    pub fn status(&self) -> String {
        if let Some(id) = self.id {
            format!(
                "Frog {} has taken {} jumps to the {} and is sitting on lilly pad {}",
                id, self.jumps, self.heading, self.position
            )
        } else {
            format!(
                "This frog has taken {} jumps to the {} and is sitting on lilly pad {}",
                self.jumps, self.heading, self.position
            )
        }
    }
    /// Returns the frog's ID, total number of jumps, the heading, and the furthest lilly pad the frog jumped to
    pub fn result(&self) -> String {
        if let Some(id) = self.id {
            format!(
                "Frog {} took a total of {} jumps to the {} and made it to lilly pad {} at the furthest",
                id, self.jumps, self.heading, self.distance
            )
        } else {
            format!(
                "This frog took a total of {} jumps to the {} and made it to lilly pad {} at the furthest",
                self.jumps, self.heading, self.distance
            )
        }
    }
    /// Returns the frog's data in the for of a csv entry
    pub fn csv_results(&self) -> String {
        let distance = match self.heading {
            FrogHeading::Left => -self.distance,
            FrogHeading::Right => self.distance,
        };

        if let Some(id) = self.id {
            format!("{},{},{}\n", id, self.jumps, distance)
        } else {
            format!("{},{}\n", self.jumps, distance)
        }
    }
}

// Getters
impl Frog {
    /// Returns the position of the frog
    pub fn position(&self) -> isize {
        self.position
    }
    /// Returns the total number of jumps of the frog
    pub fn jumps(&self) -> usize {
        self.jumps
    }
    /// Returns which side of the center that the frog jumped to
    pub fn heading(&self) -> FrogHeading {
        self.heading
    }
}

impl std::fmt::Debug for Frog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = if let Some(id) = self.id {
            format!("Some({})", id)
        } else {
            "None".to_string()
        };

        write!(
            f,
            indoc! {
            "frog:
                id: {},
                position: {},
                jumps: {},
                distance: {},
                heading: {},"
            },
            id, self.position, self.jumps, self.distance, self.heading,
        )
    }
}
