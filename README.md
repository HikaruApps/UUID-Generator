# 🆔 UUID-Generator

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Build](https://img.shields.io/badge/build-passing-success?style=flat-square)

FlexID is a lightweight, zero-dependency (except rand), and flexible unique ID generator for Rust. It allows you to generate random identifiers with custom lengths and character sets using a fluent Builder pattern.

It is a perfect alternative to UUID when you need shorter, URL-safe, or custom-formatted identifiers (like NanoID).

## ✨ Features

* Flexible Length: Generate IDs of any length (default is 21).
* Custom Charsets: Use numbers, hex, standard alphabets, or your own custom symbols.
* Secure: Uses rand::thread_rng for cryptographically strong random generation.
* Fluent API: Easy-to-use Builder pattern implementation.
* Thread Safe: Designed to be used across threads.

## 📦 Installation

Add this to your Cargo.toml:

`toml
[dependencies]
rand = "0.8"
# If this is a local module, no other dependencies are needed.
