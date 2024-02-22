use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "shodan-favicon-preimage")]
pub struct Args {
    /// File to compute hash on
    pub input: PathBuf,

    /// The target hash to find (32 bits)
    #[clap(default_value = "1337")]
    pub target: u32,

    /// Output file to store the base64 encoded content
    #[clap(short, long, default_value = "output.b64")]
    pub output: PathBuf,
}
