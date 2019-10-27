use std::env;
use std::process;

fn main() {
    //    let mut args = env::args();
    //    args.next();
    //    let path = args.next().unwrap_or_else(|| {
    //        eprintln!("Not enough arguments");
    //        process::exit(1);
    //    });
    //
    if let Err(e) = strand_match::run() {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
