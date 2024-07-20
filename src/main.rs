use jumping_frog_problem::*;
use std::{fs, thread};

const SPREADSHEET_NAME: &str = "results.csv";
const PRINT_ERROR: &str = "Cliclack failed to print to stdout";

fn main() {
    cliclack::intro("Welcome to the frog jumping problem simulator").expect(PRINT_ERROR);

    let group_size: usize = cliclack::input("How many frogs per group do you want to simulate?")
        .placeholder("Enter a positive integer")
        .validate(|input: &String| match input.trim().parse::<usize>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Please enter a positive integer!"),
        })
        .interact()
        .expect("Failed to make prompt interactive");
    let total_groups: usize = cliclack::input("How many groups do you want to simulate?")
        .placeholder("Enter a positive integer")
        .validate(|input: &String| match input.trim().parse::<usize>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Please enter a positive integer!"),
        })
        .interact()
        .expect("Failed to make prompt interactive");

    let mut sim_results = Vec::with_capacity(group_size * total_groups);
    let mut csv_output = String::with_capacity(group_size * total_groups * (4 * 3) + 37);
    /*
    THREADS * GROUPS gets the total number of frogs
    4*3 accounts for the csv output data values (frog ID, jumps, and furthest distances)
    37 accounts for the title of the columns
    */

    let progress = cliclack::ProgressBar::new(total_groups as u64);
    progress.start("Simulating groups");
    for a in 0..total_groups {
        let mut sims = Vec::with_capacity(group_size);

        for id in 1..=group_size {
            let mut frog = Frog::start(a * group_size + id);

            sims.push(thread::spawn(|| -> Frog {
                while frog.jump().is_none() {
                    // println!("{}", frog.status());
                }
                // println!("{}", frog.status());
                frog
            }));
        }

        for sim in sims {
            sim_results.push(sim.join().unwrap());
        }
        progress.inc(1);
    }
    progress.stop("Simulating finished!");

    // println!();
    csv_output.push_str("Frog ID,Total Jumps,Furthest distance\n");
    sim_results
        .iter()
        .for_each(|frog| csv_output.push_str(&frog.csv_results()));

    _ = fs::File::create(SPREADSHEET_NAME).expect("Failed to create spreadsheet");

    fs::write(SPREADSHEET_NAME, csv_output).expect("Failed to write to spreadsheet");

    cliclack::outro("See the 'results.csv' spreadsheet for the output").expect(PRINT_ERROR);
}
