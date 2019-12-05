use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::exit;
use xmcd_rs::Reader;
use xmcd_rs::{bail, error, Error, Mode};

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
struct Opt {
    /// Simulation mode.
    mode: Mode,
    /// Optional path to input file; if not supplied will read from stdin
    input: Option<PathBuf>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        let mut e: &dyn std::error::Error = &e;
        while let Some(source) = e.source() {
            eprintln!("  - caused by: {}", source);
            e = source;
        }
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    let stdin = io::stdin();

    let input = match opt.input {
        Some(path) => {
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            Reader::File(reader)
        }
        None => {
            let guard = stdin.lock();
            Reader::Stdin(guard)
        }
    };

    let xas = xmcd_rs::xas::Xas::new(input)?;
    println!("{:?}", xas);

    Ok(())
}
