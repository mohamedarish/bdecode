# bendecode

An decoder for Bencode to rust standard format

[![Documentation](https://docs.rs/bendecode/badge.svg)](https://docs.rs/bendecode/0.1.3/bendecode/index.html)
[![test](https://github.com/mohamedarish/bendecode/actions/workflows/test.yml/badge.svg)](https://github.com/mohamedarish/bendecode/actions/workflows/test.yml)
[![lint](https://github.com/mohamedarish/bendecode/actions/workflows/lint.yml/badge.svg)](https://github.com/mohamedarish/bendecode/actions/workflows/lint.yml)
[![test](https://github.com/mohamedarish/bendecode/actions/workflows/build.yml/badge.svg)](https://github.com/mohamedarish/bendecode/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/bendecode?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fbendecode)](https://crates.io/crates/bendecode)
[![Crates.io](https://img.shields.io/crates/l/bendecode?link=https%3A%2F%2Fgithub.com%2Fmohamedarish%2Fbendecode%2Fblob%2Fmain%2FLICENCE)](https://github.com/mohamedarish/bendecode/blob/master/LICENSE)

The BitTorrent specification v1 is used to parse the .torrent file
As v2 is currently not being used actively and instead a hybrid of both is used

More about the torrent file specification can be read [here](https://en.wikipedia.org/wiki/Torrent_file#File_structure)

The file structure used in this project can be found [here](https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure)

## Installation

Add `bendecode` as a dependency in your `Cargo.toml`:

```toml
    bendecode = "0.1.3"
```

Or use cargo to add the latest version to your dependencies

```sh
    cargo add bendecode
```

## Usage Guide

bendecode holds your hand through the whole decoding process by handling everything

```rs
    use bendecode::torrent::Torrent;
    let file = 
        std::fs::read_to_string("torrent_file.torrent").expect("Cannot read the file");

    // The file may not always be read using read_to_string
    // as torrent files sometimes contain invalid UTF-8
    // Hence it is recommended to read the file as bytes by using fs::read()
    // and then making a &str by joining the Vec<u8> into a &str

    let torrent = Torrent::from(file).expect("Cannot parse torrent");
```

## Contributing

Contributions re greatly appreciated! Read [CONTRIBUTING.md](./CONTRIBUTING.md) to know the contributing guidelines.
