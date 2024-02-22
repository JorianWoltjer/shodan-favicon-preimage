use std::fs;

use b64::ToBase64;
use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use shodan_favicon_preimage::{
    cli::Args,
    murmur3::{get_intermediate, murmur3_32},
    BASE64_CONFIG,
};

fn main() {
    let args = Args::parse();

    let mut input = fs::read(&args.input).unwrap();

    let mut encoded_input;
    println!("Aligning input...");
    loop {
        encoded_input = input.to_base64(BASE64_CONFIG);

        if encoded_input.contains('=') || encoded_input.len() % 4 != 0 {
            input.push(0); // null byte
            continue;
        } else {
            break;
        }
    }
    let (state, processed) =
        get_intermediate(encoded_input.as_bytes()).expect("input must be a multiple of 4 bytes");

    println!("State: {state} @ {processed} bytes");
    println!("Starting search...");

    let n = (0..u32::MAX).into_par_iter().find_any(|n| {
        let guess = n.to_le_bytes().to_base64(BASE64_CONFIG) + "\n";

        let hash = murmur3_32(guess.as_bytes(), state, processed);

        hash == args.target
    });

    if let Some(n) = n {
        let guess = n.to_le_bytes().to_base64(BASE64_CONFIG) + "\n";
        println!(
            "SUCCESS: Found hash={} for input {n} ({guess:?})",
            args.target
        );

        let output = encoded_input + &guess;

        fs::write(&args.output, output.clone()).unwrap();
        println!("Wrote result to {:?}", args.output);
        println!(
            "Decode it using `base64 -di {:?} > output.ico`",
            args.output
        );

        assert_eq!(murmur3_32(output.as_bytes(), 0, 0), args.target);
    } else {
        println!("FAIL: No hash found. Try altering your input slightly.");
    }
}
