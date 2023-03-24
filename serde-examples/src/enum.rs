use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Intoto {
    spec: String,
    #[serde(rename = "public_key")]
    public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HashedRecord {
    signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    #[serde(flatten)]
    body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
enum Body {
    #[serde(rename = "intoto")]
    Intoto(Intoto),
    #[serde(rename = "hashedrecord")]
    HashedRecord(HashedRecord),
}

fn main() {
    let data = r#"{
        "api_version": 1,
        "kind": "intoto",
        "spec": "some spec...",
        "public_key": "some public key"
    }"#;

    let entry: LogEntry = serde_json::from_str(data).unwrap();
    println!("{:?}", entry);
    match entry.body {
        Body::Intoto(value) => {
            println!("spec: {:?}", value.spec);
            println!("public_key: {:?}", value.public_key);
        }
        _ => println!("other...."),
    }

    let data = r#"{
        "kind": "hashedrecord",
        "signature": "some signature..."
    }"#;

    let entry: LogEntry = serde_json::from_str(data).unwrap();
    println!("{:?}", entry);

    //let message: Message = serde_json::from_str(data).unwrap();
    //process_message(&message);
}
