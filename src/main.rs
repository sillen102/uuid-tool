use clap::Parser;
use uuid::Uuid;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of uuids to generate
    #[arg(short, long, default_value_t = 1)]
    number: u16,
    /// Copy generated uuids to clipboard
    #[arg(short, long, default_value_t = false)]
    copy_to_clipboard: bool,
}

fn main() {
    let args = Args::parse();
    let mut uuids : Vec<Uuid> = Vec::new();
    let mut uuids_string = String::new();

    for count in 0..args.number {
        let uuid = Uuid::new_v4();
        uuids.push(uuid);
        uuids_string.push_str(uuid.to_string().as_str());

        if is_not_last(&args, count) {
            uuids_string.push_str("\n");
        }
    }

    println!("{}", uuids_string);

    if args.copy_to_clipboard {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(uuids_string).unwrap();
        println!("Copied to clipboard");
    }
}

fn is_not_last(args: &Args, count: u16) -> bool {
    (count + 1) < args.number
}
