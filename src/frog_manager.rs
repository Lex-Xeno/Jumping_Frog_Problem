use frog::*;
use std::{fs, io::Write, thread};
pub struct FrogManager {
    group_size: usize,
    total_groups: usize,
    frogs: Vec<Frog>,
}

impl FrogManager {
    pub fn new(group_size: usize, total_groups: usize) -> FrogManager {
        FrogManager {
            group_size,
            total_groups,
            frogs: Vec::with_capacity(group_size * total_groups),
        }
    }

    pub fn simulate(&mut self) {
        let bar = cliclack::progress_bar(self.total_groups as u64);
        bar.start("Simulating groups");
        for a in 0..self.total_groups {
            let mut sims = Vec::with_capacity(self.group_size);

            for id in 1..=self.group_size {
                let mut frog = Frog::start(a * self.group_size + id);

                sims.push(thread::spawn(|| -> Frog {
                    while frog.jump().is_none() {
                        // println!("{}", frog.status());
                    }
                    // println!("{}", frog.status());
                    frog
                }));
            }

            for sim in sims {
                self.frogs.push(sim.join().unwrap());
            }
            bar.inc(1);
        }
        bar.stop("Simulating finished!");
    }

    pub fn export(&self, spreadsheet_name: &str) {
        let mut spreadsheet_name = spreadsheet_name.to_string();
        spreadsheet_name.push_str(".csv");
        let spreadsheet_name = spreadsheet_name.as_str();

        let mut results = String::with_capacity(self.group_size * self.total_groups * (4 * 3) + 37); // 37 accounts for the title of the columns TODO: create algorithm that calculates this more precisely

        results.push_str("Frog ID,Total Jumps,Furthest distance\n");
        self.frogs
            .iter()
            .for_each(|frog| results.push_str(&frog.csv_results()));

        fs::File::create(spreadsheet_name)
            .expect("Failed to create spreadsheet")
            .write_all(results.as_bytes())
            .expect("Failed to write to spreadsheet");
    }
}

mod frog {
    use indoc::indoc;
    use rand::random;

    /// Represents a frog
    /// # Fields
    /// - `id` is an ID given to each frog to distinguish them from one and other if multiple frogs are used
    /// - `position` represents the lilly pad that the frog is currently on
    /// - `jumps` is the number of jumps the frog has made
    /// - `distance` - is the number of the furthest lilly pad that the frog has jumped to
    pub struct Frog {
        id: usize,
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
        pub fn start(id: usize) -> Frog {
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
            format!(
                "Frog {} has taken {} jumps to the {} and is sitting on lilly pad {}",
                self.id, self.jumps, self.heading, self.position
            )
        }
        /// Returns the frog's ID, total number of jumps, the heading, and the furthest lilly pad the frog jumped to
        pub fn result(&self) -> String {
            format!(
                    "Frog {} took a total of {} jumps to the {} and made it to lilly pad {} at the furthest",
                    self.id, self.jumps, self.heading, self.distance
                )
        }
        /// Returns the frog's data in the for of a csv entry
        pub fn csv_results(&self) -> String {
            let distance = match self.heading {
                FrogHeading::Left => -self.distance,
                FrogHeading::Right => self.distance,
            };
            format!("{},{},{}\n", self.id, self.jumps, distance)
        }
    }

    // Getters
    impl Frog {
        /// Returns the ID of the frog
        pub fn id(&self) -> usize {
            self.id
        }
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
            write!(
                f,
                indoc! {
                "frog: {}
                    jumps: {}
                    lilly pad: {}
                    furthest distance: {}
                    heading: {}",
                },
                self.id, self.jumps, self.position, self.distance, self.heading,
            )
        }
    }
}
