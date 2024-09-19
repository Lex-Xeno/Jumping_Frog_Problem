use anyhow::Result;
use jumping_frog_problem::*;
use std::{fs::File, io::Write};

fn main() -> Result<()> {
    cliclack::intro("Welcome to the frog jumping problem simulator")?;

    let config = get_config();
    let (tot_frog, tot_thread) = (config.total_frogs, config.threads);
    let mut result_file = File::create("results.csv").handle("Failed to create the results file");

    let (mut handlers, result_rec) = Handler::new_group(tot_thread);
    handlers.iter().enumerate().for_each(|(id, handler)| {
        handler
            .issue()
            .unwrap_or_else(|_| panic!("Handler {} has hung up its channel!", id))
    });

    let progress = cliclack::progress_bar(tot_frog as u64);
    progress.start("Simulating frogs...");
    for _ in 0..(tot_frog - tot_thread) {
        let (id, result) = result_rec
            .recv()
            .expect("All the handlers have hung up their channels!");

        result_file
            .write_all(result.as_bytes())
            .handle("Failed to write to the results file!");

        handlers[id]
            .issue()
            .unwrap_or_else(|_| panic!("Handler {} has hung up its channel!", id));

        progress.inc(1);
    }

    handlers.iter().enumerate().for_each(|(id, handler)| {
        handler.stop().unwrap_or_else(|_| {
            panic!(
                "Handler {} has hung up its channel and didn't receive the stop signal!",
                id
            )
        })
    });

    for _ in 0..tot_thread {
        let (_, result) = result_rec
            .recv()
            .expect("All the handlers have hung up their channels!");

        result_file
            .write_all(result.as_bytes())
            .handle("Failed to write to the results file!");
        progress.inc(1);
    }

    handlers
        .drain(0..handlers.len())
        .enumerate()
        .for_each(|(id, handler)| {
            handler
                .wait()
                .unwrap_or_else(|err| panic!("Handler {} panicked with message: {:?}", id, err));
        });
    progress.stop("All frogs have been simulated!");

    cliclack::outro("See the 'results.csv' spreadsheet for the output")?;

    Ok(())
}
