use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str;
use std::str::FromStr;
use tough;
use tough::{ExpirationEnforcement, FilesystemTransport, RepositoryLoader, TargetName};
use url::Url;

#[derive(Parser, Debug)]
#[command(author,
    version,
    long_about = None)]
/// TUF example client
struct Args {
    #[arg(
        short,
        long,
        help = "The path to the trusted root.json metadata file shipped with the client. ",
        default_value = "tuf_client/root.json"
    )]
    trusted_root_json: PathBuf,

    #[arg(
        short,
        long,
        help = "The path to the TUF repository",
        default_value = "tuf_repo"
    )]
    repo_dir: PathBuf,

    #[arg(
        short,
        long,
        help = "The path to the TUF download directory",
        default_value = "tuf_client"
    )]
    download_dir: PathBuf,
}

fn main() {
    println!("TUF client example...");
    let args = Args::parse();
    let root = fs::canonicalize(args.repo_dir).unwrap();
    let metadata_dir = Url::from_file_path(root.join("metadata")).unwrap();
    let targets_dir = Url::from_file_path(root.join("targets")).unwrap();

    // Load the trusted root metadata json file which is shipped with the client
    // in some way.
    let trusted_root_json = File::open(&args.trusted_root_json).unwrap();

    // The directory where TUF metadata files will be stored.
    let download_dir = args.download_dir;
    if !download_dir.exists() {
        fs::create_dir(&download_dir)
            .expect(format!("Could not create directory {:?}", &download_dir).as_str());
    }

    let repository = RepositoryLoader::new(trusted_root_json, metadata_dir, targets_dir)
        .transport(FilesystemTransport)
        .expiration_enforcement(ExpirationEnforcement::Unsafe)
        .datastore(&download_dir)
        .load()
        .unwrap();

    println!("Tring to fetch artifact.txt from TUF repository");
    let artifact = repository
        .read_target(&TargetName::from_str("artifact.txt").unwrap())
        .unwrap();
    if let Some(mut a) = artifact {
        let mut buffer = Vec::new();
        a.read_to_end(&mut buffer).unwrap();
        println!(
            "Fetched artifact.text: {:?}",
            str::from_utf8(&buffer).unwrap()
        );
    }
}
