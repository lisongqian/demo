#[macro_use(crate_authors)]
extern crate clap;

use clap::{Arg, Command};

fn main() {
    let app = Command::new("demo").arg(
        Arg::new("log-file")
            .long("log-file")
            .help("Log file. Standard error is used if not specified")
            .num_args(1)
            .default_value("/data/log/vmm.log")
            .group("logging"),
    );
    let cmd = app.get_matches();
    if let Some(file) = cmd.get_one::<String>("log-file") {
        println!("value: {file:?}")
    } else {
        println!("None");
    }
}
