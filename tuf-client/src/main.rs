use clap::Parser;
use std::path::PathBuf;
use tough;
use tough::{ExpirationEnforcement, FilesystemTransport, RepositoryLoader};
use url::Url;

#[derive(Parser, Debug)]
#[command(author,
    version,
    long_about = None)]
/// TUF example client
struct Args {
    #[arg(short, long, help = "The path to the local TUF repository")]
    tuf_repo_dir: PathBuf,
}

fn main() {
    println!("TUF client example...");
    let args = Args::parse();
    let root = args.tuf_repo_dir;
    let metadata_dir = Url::from_file_path(root.join("metadata")).unwrap();
    let targets_dir = Url::from_file_path(root.join("targets")).unwrap();
    println!("metadata dir: {}", &metadata_dir);
    println!("targets dir: {}", &targets_dir);
}
