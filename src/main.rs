use jumping_frog_problem::{frog_manager::*, *};

const PRINT_ERROR: &str = "Cliclack failed to print to stdout";

fn main() {
    cliclack::intro("Welcome to the frog jumping problem simulator").expect(PRINT_ERROR);

    let raw_config = read_config();
    let config = parse_config(raw_config);

    let mut manager = FrogManager::new(config.group_size, config.total_groups);
    manager.simulate();
    manager.export("results");

    cliclack::outro("See the 'results.csv' spreadsheet for the output").expect(PRINT_ERROR);
}
