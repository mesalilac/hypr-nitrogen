// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    let mut builder = env_logger::Builder::new();

    if cli.verbose {
        builder.filter_level(log::LevelFilter::Info);
    }

    builder.init();

    hypr_nitrogen_lib::run()
}
