pub mod parallelization;
use parallelization::parallel::{directory_reader, file_reader};
use clap::Parser;
use std::path::PathBuf;
use clap::command;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct BrokenUrl {
    directory: PathBuf,
}

fn main() {
    let args = BrokenUrl::parse();
    let directory_path = args.directory;

    if !(directory_path.exists()) {
        panic!("Please enter a valid path");
    }

    //Print statement for formatting a table
    println!(
        "{0: <120} | {1: <15} | {2:10}",
        "Path", "Status", "Status Code"
    );

    //If the path entered is just a single file, it is sent directly to be read
    if directory_path.is_file() {
        file_reader(directory_path);
        return;
    }

    directory_reader(directory_path);
}
