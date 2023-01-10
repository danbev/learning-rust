use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(short, long, help = "Some argument...")]
    something: String,
    #[command(flatten)]
    manifest: clap_cargo::Manifest,
    #[command(flatten)]
    workspace: clap_cargo::Workspace,
    #[command(flatten)]
    features: clap_cargo::Features,
}

fn main() {
    let args = Args::parse();
    println!("args = {:#?}", args);
}
