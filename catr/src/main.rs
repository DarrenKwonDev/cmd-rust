use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
// this comment will be 'about'
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    
    #[arg(short = 'n', long = "number")]
    number_lines: bool,
    
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

fn run(args: Args) -> Result<()> {
    if args.number_lines && args.number_nonblank_lines {
        panic!("use both is illegal");
    }

    let mut line_num = 1;

    for filename in args.files {

        match open(&filename) {
            
            Ok(reader) => {
                

                for ln in reader.lines() {
                    match ln {
                        Ok(line) => {
                            

                            if args.number_lines {
                                println!("{line_num:6}\t{line}");
                                line_num += 1;
                            } else if args.number_nonblank_lines {
                                if line.is_empty() {
                                    println!();
                                } else {
                                    println!("{line_num:6}\t{line}");
                                    line_num += 1;
                                }
                            } else {
                                println!("{line}");
                            }
                        },
                        Err(e) => eprintln!("{filename}: Error reading line: {e}"),
                    }
                }
            }
            Err(e) => eprintln!("{filename}: {e}"),
        }
    }
    
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
