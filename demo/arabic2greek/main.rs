use arithmos::GreekNumeral;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    number: u32,
}

fn main() {
    let args = Cli::parse();

    let num: GreekNumeral = GreekNumeral::new(args.number).unwrap();
    println!("{}", num);
}
