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
    let filtered: Vec<String> = args.filter(|a| !a.starts_with("filtered")).collect();
    println!("filtered: {:?}", filtered);
    let args = Args::parse_from(filtered);
    println!("parsed args = {:#?}", args);
}
