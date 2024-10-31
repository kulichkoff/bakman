use clap::Parser;

use bakman::model::Cli;
use bakman::run;

fn main() {
    let cli = Cli::parse();

    run(&cli);
}
