use clap::Parser;
use minesweeper::annotate::*;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    #[arg(short, long)]
    rows: u8,

    #[arg(short,long)]
    cols: u8,

    #[arg(short,long)]
    minefield: String
}

fn main() {
    let args = Args::parse();
    let rows = args.rows;
    let cols = args.cols;
    let minefield = &[args.minefield.as_str()];

    println!("Minefield dimension rows*cols: {}", rows*cols);
    println!("Minefield dimension: {}", args.minefield.len());


    check_correctness(rows, cols, args.minefield.as_str());

    set_rows_and_cols(rows, cols);

    let result = annotate(minefield);

    println!("The result is {:?}", &result)
}

fn check_correctness(rows: u8, cols: u8, minefield: &str) {
    if rows*cols != minefield.len().try_into().unwrap() {
        panic!("Not correct length");
    }
}