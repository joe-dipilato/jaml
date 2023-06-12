use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

#[derive(Debug)]
#[derive(Clone)]
struct Arguments {
    filename: *mut Path
}

fn help_msg() {
    // print the help message
    let args: Vec<String> = env::args().collect();
    let name = &args[0];
    println!("Usage: {name} <PATH>");
}

fn parse_args() -> Arguments {
    // parse the command line args
    let args: Vec<String> = env::args().collect();

    let argv = &args[1..];
    if argv.len() == 0 {
        println!("ERROR: Invalid arguments");
        help_msg();
        std::process::exit(exitcode::DATAERR);
    }
    println!("My path is {}.", args[0]);
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    let filename = Path::new(&args[1].to_owned()).to_owned();
    let result = Arguments { filename: &mut *filename.to_owned() };
    return result.to_owned();
}

fn main() {
    let args = parse_args(); 
    println!("{args:#?}");
    // let filename = *args.filename;
    // println!("{filename}");
    
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
    else
    {
        println!("dead");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
