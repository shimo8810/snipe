use serde::{Deserialize, Serialize};
use snipe::args::Args;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::process;

#[derive(Serialize, Deserialize)]
struct Snipet {
    prefix: String,
    body: Vec<String>,
    description: String,
}

fn run() -> Result<(), String> {
    let args = Args::parse();
    let reader: Box<dyn BufRead> = match args.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(file) => {
            let file = fs::File::open(file);
            let file = file.map_err(|e| e.to_string())?;
            Box::new(BufReader::new(file))
        }
    };

    let mut body = vec![];
    for line in reader.lines() {
        body.push(line.map_err(|e| e.to_string())?);
    }

    let snip = Snipet {
        prefix: args.prefix.clone(),
        body,
        description: args.description,
    };

    println!(
        "\"{}\": {},",
        args.prefix,
        serde_json::to_string_pretty(&snip).map_err(|e| e.to_string())?
    );

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };
}
