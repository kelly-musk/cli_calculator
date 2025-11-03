use clap::{Parser, Subcommand};
use std::process;

// calculator info
#[derive(Debug, Parser)]
#[command(name = "calculator")]
#[command(about = "A fast and simple calculator ")]
struct CliCalculator {
    #[command(subcommand)]
    operations: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
    let clicalculator = CliCalculator::parse();

    match clicalculator.operations {
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
