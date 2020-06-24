mod lib;

use colored_diff;
use std::fs;
use std::path;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pyproject-toml-prettify",
    about = "Pretify pyproject.toml file."
)]
struct Opt {
    /// Check if modifications are needed and output the diff to stdout.
    #[structopt(long)]
    check: bool,

    /// Path to `pyproject.toml`
    #[structopt(long, short, parse(from_os_str), default_value = "./pyproject.toml")]
    file: path::PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let input = fs::read_to_string(&opt.file).expect(&format!(
        "Cannot find `pyproject.toml` at {}",
        opt.file.to_str().unwrap()
    ));

    let output = lib::prettify(&input);

    if input == output {
        println!(
            "Nothing to do with `pyproject.toml` at {}",
            opt.file.to_str().unwrap()
        );
        return
    }

    match opt.check {
        true => {
            println!(
                "{}",
                colored_diff::PrettyDifference {
                    expected: &input,
                    actual: &output,
                }
            );
            process::exit(1);
        }
        false => {
            fs::write(&opt.file, output).expect(&format!(
                "Cannot update `pyproject.toml` at {}",
                opt.file.to_str().unwrap()
            ));
            println!(
                "Successfully updated `pyproject.toml` at {}",
                opt.file.to_str().unwrap()
            )
        }
    }
}
