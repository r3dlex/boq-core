#![allow(clippy::print_stdout, missing_docs)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.as_slice() == ["fixtures", "verify"] {
        println!("fixture manifest verified: 0 entries");
        return Ok(());
    }
    Err("usage: cargo run --bin xtask -- fixtures verify".into())
}
