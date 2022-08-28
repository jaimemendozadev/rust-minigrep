use minigrep::Config;
use std::env; // This function returns an iterator of the command line argu- ments that were given to minigrep
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    let run_result = minigrep::run(&config);

    if let Err(e) = run_result {
        process::exit(1)
    }
}
