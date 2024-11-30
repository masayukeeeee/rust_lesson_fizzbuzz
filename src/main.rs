use clap::Parser;
extern crate colored;
use colored::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Action to perform, one of: while, if, for, match")]
    action: String,
    #[arg(short, long, help = "Number of maximum values of FizzBuzz.", default_value_t = 100)]
    max: u32,
    #[arg(
        short,
        long,
        help = "condition implementation, onf of if and match",
        default_value_t = String::from("if"))
    ]
    condition: String,
    #[arg(short, long, help = "first divisor", default_value_t = 3)]
    fd: u32,
    #[arg(short, long, help = "second divisor", default_value_t = 5)]
    sd: u32,
}

fn is_divisible_by(x: u32, divisor: u32) -> bool {
    x % divisor == 0
}

fn print_fizz() {
    print!("{}", "Fizz".yellow())
}

fn print_buzz() {
    print!("{}", "Buzz".blue())
}

fn print_fizzbuzz() {
    print!("{}", "FizzBuzz".bold().red())
}

fn fizzbuzz_printer_if(x: u32, a: u32, b: u32) {
    if is_divisible_by(x, a) && is_divisible_by(x, b) {
        print_fizzbuzz();
    } else if is_divisible_by(x, a) {
        print_fizz();
    } else if is_divisible_by(x, b) {
        print_buzz();
    } else {
        print!("{}", x);
    }
}

fn fizzbuzz_printer_match(x: u32, a: u32, b: u32) {
    match (x % a, x % b) {
        (0, 0) => print_fizzbuzz(),
        (0, _) => print_fizz(),
        (_, 0) => print_buzz(),
        _ => print!("{}", x),
    }
}

fn fizzbuzz_printer(x: u32, condition: String, a: u32, b: u32) {
    // If the number is divisible by a, print "Fizz"
    // If the number is divisible by b, print "Buzz"
    // If the number is divisible by a and b, print "FizzBuzz"
    // Otherwise, print the number
    if condition == "if" {
        fizzbuzz_printer_if(x, a, b);
    } else if condition == "match" {
        fizzbuzz_printer_match(x, a, b);
    } else {
        fizzbuzz_printer_if(x, a, b);
    }
}
    
fn main() {
    // let args: Vec<String> = env::args().collect();
    // let program_name = &args[0];
    // println!("Program name: {}", program_name);
    // println!("Number of arguments: {}", args.len());
    // for arg in &args[1..] {
    //     println!("Argument: {}", arg);
    // }
    let args = Args::parse();
    
    println!("Action: {}", args.action);
    println!("Max value: {}", args.max);
    println!("Condition: {}", args.condition);

    match args.action.as_str() {
        "while" => {
            // while
            let mut i = 1;
            while i <= args.max {
                fizzbuzz_printer(i, args.condition.clone(), args.fd, args.sd);
                if i == args.max {
                    println!();
                } else {
                    print!(" => ");
                }
                i += 1;
            }
        },
        "for" => {
            // for
            for i in 1..=args.max {
                fizzbuzz_printer(i, args.condition.clone(), args.fd, args.sd);
                if i == args.max {
                    println!();
                } else {
                    print!(" => ");
                }
            }
        },
        _ => {
            println!("Invalid action. Executed with while loop.");
        }
    }
}

