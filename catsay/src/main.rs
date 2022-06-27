extern crate structopt;
use structopt::StructOpt;
extern crate colored;
use colored::*;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value="Meow!")]
    message: String,
    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}
fn main() {
    let options = Options::from_args();
    let message = options.message;

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    let eye = if options.dead { "x" } else { "o" };

    println!("{}", message.bright_yellow().underline()
        .on_purple());
    println!(" \\");
    println!("  \\");
    println!("    /\\_/\\");
    println!("   ( {eye} {eye} )", eye=eye.red().bold());
    println!("   =( I )=");
    }