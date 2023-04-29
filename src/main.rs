use std::io::{BufRead, BufReader, BufWriter, Write};
mod args;
use args::RangleArgs;
use clap::Parser;

#[derive(Debug)]
pub enum RangleError {
    TangleHeaderMissing,
    InvalidCodeBlock,
}

fn main() {
    let args = RangleArgs::parse();
    let target_file = satisfy_requirements(&args.file).expect("Crash and burn");
    tangle(&args.file, &target_file);
}

fn satisfy_requirements(filename: &str) -> Result<String, RangleError> {
    let file = std::fs::File::open(filename).expect("No file found");
    let reader = BufReader::new(file);
    let mut target_file = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('*') {
            println!("I'm in the starts with * line");
            return Err(RangleError::TangleHeaderMissing);
        }
        if line.starts_with("#+PROPERTY:") {
            let mut words = line.split_whitespace();
            match words.find(|w| w == &":tangle") {
                Some(_) => target_file = words.next().unwrap().to_owned(),
                None => {
                    println!("I'm in the starts with property line");
                    return Err(RangleError::TangleHeaderMissing);
                }
            }
            break;
        }
    }

    Ok(target_file)
}

fn tangle(source_filename: &str, target_filename: &str) {
    let target_file = std::fs::File::create(target_filename).expect("Cannot create file");
    let source_file = std::fs::File::open(source_filename).expect("No file found");
    let mut writer = BufWriter::new(target_file);
    let reader = BufReader::new(source_file);
    let mut can_write = false;
    for line in reader.lines() {
        let line = line.unwrap();
        if line
            .trim_start()
            .to_ascii_lowercase()
            .starts_with("#+begin_src")
        {
            can_write = true;
            continue;
        }
        if line
            .trim_start()
            .to_ascii_lowercase()
            .starts_with("#+end_src")
        {
            can_write = false;
            writeln!(writer, "").unwrap();
        }
        if can_write {
            writeln!(writer, "{}", line).unwrap();
        };
    }
    writer.flush().unwrap();
}
