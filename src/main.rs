use std::io::{Read, Write};

use base64::engine::general_purpose as base64_engines;
use base64::Engine;

fn main() -> anyhow::Result<()> {
    let args = {
        use clap::Parser;
        #[derive(Parser)]
        /// Copy to clipboard using ANSI OSC52 sequence
        struct Args {
            /// File to read from; if not provided, stdin is read
            filename: Option<String>,
        }
        Args::parse()
    };

    let contents = {
        match args.filename {
            Some(filename) if !filename.is_empty() => std::fs::read(filename),
            _ => {
                let mut buf = vec![];
                std::io::stdin().read_to_end(&mut buf)?;
                Ok(buf)
            }
        }
    }?;

    let mut encoded_buf =
        vec![0u8; base64::encoded_len(contents.len(), true).expect("input too large")];

    let written = base64_engines::STANDARD
        .encode_slice(&contents, &mut encoded_buf)
        .expect("buffer overflow");

    {
        let mut stdout = std::io::stdout();
        stdout.write(&[b"\x1b]52;c;", &encoded_buf[..written], b"\x07"].concat())?;
        stdout.flush()?;
    }

    Ok(())
}
