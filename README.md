# Shodan Favicon Preimage generator

**Generate a favicon that results in any target hash on Shodan**

This tool implements a brute force method of generating any [preimage](https://en.wikipedia.org/wiki/Preimage_attack) for the [Shodan `http.favicon.hash`](https://blog.shodan.io/deep-dive-http-favicon/) algorithm. You can, for example, make your site show up when searching for a specific hash of your choosing:

![Shodan search result for `http.favicon.hash:1337` showing only jorianwoltjer.com](https://github.com/JorianWoltjer/shodan-favicon-preimage/assets/26067369/52917489-fc72-4d03-81bb-42cfc6ae07e1)

Read more about the method and implementation in my blog post here:  
["How I got a Shodan Favicon Hash = 1337"](https://jorianwoltjer.com/blog/p/coding/shodan-favicon-preimage)

## Installation

First, [install Rust](https://www.rust-lang.org/tools/install), the programming language this project is made it to build it from source. Then these commands turn it into an optimized executable:

```bash
git clone https://github.com/JorianWoltjer/shodan-favicon-preimage.git && cd shodan-favicon-preimage
cargo build --release
./target/release/shodan-favicon-preimage --help
```

## Usage

```
Usage: shodan-favicon-preimage [OPTIONS] <INPUT> [TARGET]

Arguments:
  <INPUT>   File to compute hash on
  [TARGET]  The target hash to find (32 bits) [default: 1337]

Options:
  -o, --output <OUTPUT>  Output file to store the base64 encoded content [default: output.b64]
  -h, --help             Print help
```

Take any existing `.ico` file, and pass it as the first argument. Optionally, choose a different target hash than the default 1337:

```Shell
$ shodan-favicon-preimage favicon.ico 31337
Aligning input...
State: 2363324422 @ 20948 bytes
Starting search...
SUCCESS: Found hash=31337 for input 4107189463 ("17zO9A==\n")
Wrote result to "output.b64"
Decode it using `base64 -di "output.b64" > output.ico`
```

The `output.b64` file will be the encoded form that MurmurHash3's into your target hash, but you can decode it into a regular `.ico` file too. Then [`verify.py`](verify.py) can be used to check if the hash is indeed correct:

```Shell
$ base64 -di "output.b64" > output.ico
$ ./verify.py output.ico 
31337
```
