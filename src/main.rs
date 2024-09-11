use jumping_frog_problem::*;

const PRINT_ERROR: &str = "Cliclack failed to print to stdout";

fn main() {
    cliclack::intro("Welcome to the frog jumping problem simulator").expect(PRINT_ERROR);

    let raw_config = read_config();
    let config = parse_config(raw_config);

    let mut manager = FrogManager::new(config.total_frogs);
    manager.simulate(config.threads);

    cliclack::outro("See the 'results.csv' spreadsheet for the output").expect(PRINT_ERROR);
}
