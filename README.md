<div align="center" id="top">
  <img src="./.assets/shelly.jpg" alt="Shelly logo" width=50% />

  <!-- <a href="https://shelly.netlify.app">Demo</a> -->
</div>

<h1 align="center">Shelly</h1>

<p align="center">
  <img alt="Github top language" src="https://img.shields.io/github/languages/top/y0k4i-1337/shelly?color=56BEB8">

  <img alt="Github language count" src="https://img.shields.io/github/languages/count/y0k4i-1337/shelly?color=56BEB8">

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/y0k4i-1337/shelly?color=56BEB8">

  <img alt="License" src="https://img.shields.io/github/license/y0k4i-1337/shelly?color=56BEB8">

  <!-- <img alt="Github issues" src="https://img.shields.io/github/issues/y0k4i-1337/shelly?color=56BEB8" /> -->

  <!-- <img alt="Github forks" src="https://img.shields.io/github/forks/y0k4i-1337/shelly?color=56BEB8" /> -->

  <!-- <img alt="Github stars" src="https://img.shields.io/github/stars/y0k4i-1337/shelly?color=56BEB8" /> -->
</p>

<p align="center">
  <a href="#about">About</a> &#xa0; | &#xa0;
  <a href="#features">Features</a> &#xa0; | &#xa0;
  <a href="#technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#starting">Starting</a> &#xa0; | &#xa0;
  <a href="#usage">Usage</a> &#xa0; | &#xa0;
  <a href="#license">License</a> &#xa0; | &#xa0;
  <a href="https://github.com/y0k4i-1337" target="_blank">Author</a>
</p>

<br>

## About

`Shelly` is an intuitive tool for encoding and transforming shellcode.

`Shelly` provides support for cryptographic transformations such as Caesar cipher
and XOR operations, and it generates output in formats compatible with C#,
Python, C, PowerShell, VBA, and more.

Designed for ease of use, `Shelly` allows you to customize encryption settings and seamlessly integrate encoded data into your applications or security assessments.

## Features

- **Raw Payload Processing**
  - Reads raw output from `msfvenom` and other tools.
  - Supports binary payload transformation and encoding.

- **Encryption & Encoding**
  - Optional Caesar cipher encryption with customizable key.
  - XOR encryption with a single-byte key.
  - *TODO: Supports multiple encoding formats:*
    - *Hexadecimal*
    - *Base64*
    - *Base85*

- **Language-Specific Output**
  - Generates output code for multiple languages:
    - C#
    - Python
    - C
    - PowerShell
    - VBA
  - Automatically includes decryption code in the generated output.

- **Customization**
  - Allows defining the output variable name (default: `buf`).
  - Provides comments on dependencies when required libraries are needed.

- **Cross-Platform Support**
  - Targets multiple architectures and operating systems:
    - Linux (`x86_64`, `i686`)
    - Windows (`x86_64`, `i686`)
    - macOS (`x86_64`, `aarch64`)

## Technologies

The following technologies relate to this project:

- [Rust Programming Language](https://www.rust-lang.org/)
- [XOR Encryption](https://en.wikipedia.org/wiki/XOR_cipher)
- [Caesar Cipher](https://en.wikipedia.org/wiki/Caesar_cipher)
- [msfvenom](https://www.offensive-security.com/metasploit-unleashed/msfvenom/)


## Requirements

Before starting :checkered_flag:, you need to have [Git](https://git-scm.com) and [Cargo](https://doc.rust-lang.org/cargo/) installed.

## Starting

```bash
# Clone this project
$ git clone https://github.com/y0k4i-1337/shelly

# Access
$ cd shelly

# Install dependencies and build
$ cargo build

# Run the project
$ cargo run -- -h
```

## Usage

```
shelly -h
Shellcode encryption for fun and profit

Usage: shelly [OPTIONS]

Options:
  -i, --input <INPUT>    Path to the input file. Use "-" for stdin
  -r, --rot <ROT>        Apply ROT cipher with the specified shift value [default: 0]
  -x, --xor <XOR>        Apply XOR cipher with the specified key value [default:
                         0x00]
  -f, --format <FORMAT>  Output format for the encrypted shellcode [default: csharp]
                         [possible values: csharp, python, c, psh, vba]
  -v, --var <VAR>        Name of the variable to store the encrypted shellcode
                         [default: buf]
  -h, --help             Print help
```

## License

This project is under license from MIT. For more details, see the [LICENSE](LICENSE.md) file.


Made with :heart: by <a href="https://github.com/y0k4i-1337" target="_blank">y0k4i</a>

&#xa0;

<a href="#top">Back to top</a>
