#!/usr/bin/env python3
import argparse
import mmh3
import base64

args = argparse.ArgumentParser()
args.add_argument('file', help='File to get the Shodan hash of')
args = args.parse_args()

if __name__ == "__main__":
    with open(args.file, "rb") as f:
        data = f.read()
    encoded = base64.encodebytes(data)
    hash = mmh3.hash(encoded)
    print(hash)
