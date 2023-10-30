use anyhow::{
    bail,
    Result,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "akanec", author, version, about, long_about = None)]
struct Args {
    /// Input file path
    input: String,
    /// Output file path
    #[arg(short, long, default_value = "./a.ll")]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match akaneclib::compile(&args.input, &args.output) {
        Ok(()) => Ok(()),
        Err(errs) => {
            let err_count = errs.len();
            for e in errs {
                eprintln!("{}", e);
            }
            if err_count == 1 {
                bail!("{} error found.", err_count);
            }
            else {
                bail!("{} errors found.", err_count);
            }
        }
    }
}
