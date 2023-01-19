use clap::Parser;
use olpc_cjson::CanonicalFormatter;
use ring::digest::{digest, SHA256};
use serde::Serialize;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author,
    version,
    long_about = None)]
/// Parses a TUF json key and prints out the key_id for it.
struct Args {
    #[arg(short, long, help = "The json to be parse")]
    json: String,
}

fn main() {
    //let value = json!({"b": 12, "a": "qwerty"});
    let args = Args::parse();
    let json: Value = serde_json::from_str(&args.json).unwrap();
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, CanonicalFormatter::new());
    json.serialize(&mut ser).unwrap();
    let hash = digest(&SHA256, &buf);
    println!("key_id: {:?}", &hash);
}
