use std::io;
use std::io::Read;  // the trait Read provides read_exact
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("Hello, world!");

    let mut inp = io::stdin();
    let mut buffer : [u8; 1] = [0];

    if let Err(e) = inp.read_exact(&mut buffer) {
        eprintln!("read_exact error: {}", e);
        return ExitCode::from(1);
    }
    print!("read {}\n", buffer.len());

    ExitCode::from(0)
}
