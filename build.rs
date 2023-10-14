use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/.env");
   
    let reader = BufReader::new(File::open("src/.env")?);
    for line in reader.lines() {
        let line = line?;
        let pair = line.split_once("=").ok_or(anyhow!("Invalid .env file"))?;
        println!("cargo:rustc-env={}={}", pair.0, pair.1.trim_matches('"'));
    }

    Ok(())
}