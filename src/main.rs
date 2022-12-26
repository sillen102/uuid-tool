use clap::Parser;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of uuids to generate
    #[arg(short, long, default_value_t = 1)]
    number: u8,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.number {
        println!("{}", Uuid::new_v4());
    }
}
