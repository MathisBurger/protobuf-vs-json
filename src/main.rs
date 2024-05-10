use convert::ProtobufImpl;
use generate::RandomGeneratorTrait;
use prost::Message;
use protobufVsJson::protobuf::data::{LargeUser, MidUser, SmallStructure};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::{io::Write, time::Instant};
use structures::{large, mid, small};

mod convert;
mod generate;
mod structures;

fn main() -> std::io::Result<()> {
    //let serdeable = large::User::generate_random_amount(1);
    //let converted: Vec<LargeUser> = large::User::convert_all(serdeable.clone());
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open("./json_lg.json")?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    let mut now = Instant::now();
    let u8Slice: &[u8] = &content;
    let _: large::User = serde_json::from_slice(u8Slice).unwrap();
    let mut elapsed = now.elapsed();
    println!("JSON: {:.4?}", elapsed);
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open("./protobuf_lg")?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    now = Instant::now();
    let sl: &[u8] = &content;
    LargeUser::decode(sl)?;
    elapsed = now.elapsed();
    println!("Protobuf: {:.4?}", elapsed);
    Ok(())
}
