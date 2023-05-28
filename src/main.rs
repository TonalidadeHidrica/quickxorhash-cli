use std::{
    fmt::Write,
    io::{BufReader, Read},
    path::PathBuf,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use clap::Parser;
use fs_err::File;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use quickxorhash::QuickXorHash;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let f = File::open(args.input)?;
    let len = f.metadata()?.len();
    let bar = args.progress.then(|| {
        let bar = ProgressBar::new(len);
            bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
        bar
    });
    let mut reader = BufReader::new(f);

    let mut buffer = [0u8; 0x100000];
    let mut hasher = QuickXorHash::new();
    loop {
        match reader.read(&mut buffer)? {
            0 => break,
            n => {
                hasher.update(&buffer[..n]);
                if let Some(bar) = bar.as_ref() {
                    bar.inc(n as _);
                }
            }
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
    #[clap(long)]
    progress: bool,
}
