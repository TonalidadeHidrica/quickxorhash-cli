use std::{
    io::{BufReader, Read},
    path::PathBuf,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use clap::Parser;
use fs_err::File;
use quickxorhash::QuickXorHash;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let f = File::open(args.input)?;
    let mut reader = BufReader::new(f);

    let mut buffer = [0u8; 0x100000];
    let mut hasher = QuickXorHash::new();
    loop {
        match reader.read(&mut buffer)? {
            0 => break,
            n => hasher.update(&buffer[..n]),
        }
    }

    let res = hasher.finalize();
    let res = BASE64_STANDARD.encode(res);
    println!("{res}");

    Ok(())
}

#[derive(Parser)]
struct Args {
    input: PathBuf,
}
