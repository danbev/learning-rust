use clap::Parser;

#[derive(Debug, Parser)]
#[command(author,
      version,
      long_about = None)]
struct Args {
    #[arg(short, long, help = "Some argument...")]
    something: String,
}

fn main() {
    let mut args = std::env::args();
    let args = args.by_ref();
    println!("args: {:?}", args);
    let args = Args::parse();
    println!("parsed args = {:#?}", args);
}
