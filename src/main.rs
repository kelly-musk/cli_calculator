// use chumsky::combinator::Lazy;
use clap::{Parser, Subcommand};
// use egui::{epaint::tessellator::path, mutex::Mutex, util::History};
use std::{io::{self, BufRead, Write}, process};

// static HISTORY: Lazy<Mutex<History>> = Lazy::new(||
//     let path = "history.txt";
//     let vec = load_history_from_file(path).unwrap_or_default();
//     Mutex::new(HISTORY {vec, path.to_string()})
// );


// struct History{
//     vec: Vec<f64>,
//     path: String
// }

// calculator info
#[derive(Debug, Parser)]
#[command(name = "calculator")]
#[command(about = "A fast and simple calculator ")]
#[command(author = "Kelly Musk")]
pub struct CliCalculator {
    #[command(subcommand)]
    operations: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Repl,
    /// addition 
    #[command(name = "add")]
    Add { a: f64, b: f64 },
    /// substraction
    #[command(name = "sub")]
    Sub { a: f64, b: f64 },
    /// multiplication
    #[command(name = "mul")]
    Mul { a: f64, b: f64 },
    /// division 
    #[command(name = "div")]
    Div { a: f64, b: f64 },
}
fn main() {
    let calculator = CliCalculator::parse();

    let operations = calculator.operations.unwrap_or_else(|| {
        CliCalculator::parse_from(["Calculator", "--help"]);
        std::process::exit(0)
    });

    match operations {
        Commands::Repl => run_repl(),
        Commands::Add { a, b } => println!("{}", a + b),
        Commands::Sub { a, b } => println!("{}", a - b),
        Commands::Mul{ a, b } => println!("{}", a * b),
        Commands::Div{ a, b } => {
            if b == 0.0 {
                println!("Division Error not divisible by zero");
                process::exit(1);
            } else {
                println!("{}", a / b)
            }
        }
    }
}



fn run_repl(){
    println!("Welcom my calculator app built with repl crate");
    println!("Name of author is kelly Musk");
    println!("commands guide through this app are exit, add, sub, div");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();


    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        let input = match lines.next() {
            Some(Ok(line)) => line.trim().to_string(),
            Some(Err(e)) => {
                eprintln!("Input error {}", e);
                continue;
            }
            None => break,
        };
        println!("{}", input);
        if input.is_empty(){
            continue;
        }
        if matches!(input.as_str(), "exit" | "quit"){
            println!("Goodbye!");
            break;
        }
    
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            eprintln!("usage; <cmd> <num1> <num2>");
            continue;
        }
     let cmd = parts[0];
     let a = match parts[1].parse::<f64>() {
         Ok(n) => n,
         Err(_) => {
            eprintln!("Invalid number {}", parts[1]);
            continue;
         }
     };

        let b = match parts[2].parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid number {}", parts[2]);
                continue;
            }
        };
        match cmd {
            "add" => println!("{}", a + b),
            "sub" => println!("{}", a - b),
            "mul" => println!("{}", a * b),
            "div" => {
                if b == 0.0 {
                    println!("Error: division by zero!")
                } else {
                    println!("{}", a / b)
                }
            }
            _=> eprintln!("UNKNOWN COMMAND USE: {}. add/sub/mul/div", cmd)
        }
    }
}