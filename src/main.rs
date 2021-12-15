use std::env;
use std::process;

mod cli;
mod days;

fn main() {
    cli::run();
    run_days();
}

fn run_days() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!("\nPlease slect a solution to run."),
        2 => match args[1].parse::<u32>() {
            Ok(n) => {
                let (one, two) = get_day(n);
                println!("\nSolution for day {}.", n);
                println!("\nPart a: {:?}", one());
                println!("\nPart b: {:?}", two());
            }
            _ => println!("Please enter a valid day. This must be a number."),
        },
        3 => {
            let (one, two) = match args[1].parse::<u32>() {
                Ok(n) => get_day(n),
                _ => {
                    println!("Please enter a valid day. This must be a number.");
                    return;
                }
            };
            let part = &args[2];

            match part.as_str() {
                "a" => one(),
                "b" => two(),
                _ => println!("Only parts \"a\" and \"b\" are valid"),
            }
        }
        _ => println!("Why so many args?"),
    }
}

fn get_day(day: u32) -> (fn(), fn()) {
    match day {
        1 => days::day_01::run(),
        _ => {
            println!("No solution for that day");
            process::exit(1)
        }
    }
}
