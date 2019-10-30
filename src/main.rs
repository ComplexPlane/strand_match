use std::process;

fn main() {
    if let Err(err) = strand_match::run() {
        println!("{}, {}", err.as_fail(), err.backtrace());
        process::exit(1);
    }
}
