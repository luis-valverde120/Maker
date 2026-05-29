/*
 * This is a cli to handle structs and folder architecture a preconfigure all for proyect
 */
use clap::Parser;
use maker::cli::handlers;
use maker::cli::args::Cli;


fn main() {
    let cli = Cli::parse();

    let _ = handlers::handler_command(cli);
}
