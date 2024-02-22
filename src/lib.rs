pub mod cli;
pub mod murmur3;

pub const BASE64_CONFIG: b64::Config = b64::Config {
    char_set: b64::CharacterSet::Standard,
    newline: b64::Newline::LF,
    pad: true,
    line_length: Some(76),
};
