use std::fs;
use std::env;
use std::process;
use clap::Parser;

#[derive(Parser)]
#[clap(name    = "AoC 2021 Rust d01")]
#[clap(author  = "nickb <pgnickb@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about   = "Solution to the day 1 of AoC 2021")]
struct Cli {
    #[clap(short, long)]
    /// Enable debug output
    debug: bool,
    #[clap(short, long)]
    /// Print this help and exit
    help: bool,
    #[clap(short, long)]
    /// Print version number and exit
    version: bool
    /// Input file
    file: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut options = Options::new();
    options.optflag("h", "help", "show this help, then exit");
    options.optflag("d", "debug", "output extra debugging information");
    options.reqopt("f", "", "input file name", "FILE");
    let cmd_args = match options.parse(&args[1..]) {
        Err(why) => {
            println!("Cannot parse command args :{}", why);
            print_usage_and_exit(&program, &options);
            return;
        },
        Ok(p) => p,
    };

    if cmd_args.opt_present("h") {
        print_usage_and_exit(&program, &options);
    }


}

/// Print program usage message and exit
fn print_usage_and_exit(program: &str, options: &Options){
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", options.usage(&brief));
    process::exit(0);
}

/// Print provided arguments
fn print_args(cmd_args: getopts::Matches){
    for opt in cmd_args
    {

    }
}
