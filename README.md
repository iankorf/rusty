RUSTY
=====

Personal notes on the Rust programming language

https://doc.rust-lang.org/stable/rust-by-example/index.html

## Installation and Quick Start ##

Install

	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Update

	rustup update

Cargo

	cargo build
	cargo run
	cargo test
	cargo doc
	cargo publish

Hello World

	cargo new hello-rust
	cd hello-rust
	cargo run

Dependencies are added to `Cargo.toml`. Next time you do a `cargo build`, they are downloaded.

## Comments ##

Comments are modern C-style with doc-string support.

	// line comment
	/* block comment */
	/// doc-string following
	//! doc-string enclosing
	//* also? */ 

Need more info and examples for doc-strings

-----

## Printing ##

Formatting and printing are handled by several macros.

+ `format!`
+ `print!`
+ `println!`
+ `eprint!`
+ `eprintln!`

Syntax is very similar to python's `string.format()`.

## Variables ##

The usual variable types for ints and floats exist and have descriptive names.

+ int: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
+ float: f32, f64
+ char: 4-byte unicode
+ bool: true, false
+ tuples
+ arrays
+ strings?
+ dictionaries?

By default, variables are not mutable

	let     i = 1;    // immutable int32
	let mut f = 3.14; // mutable f64

Variable types can be stated in prefix, suffix, inferred, or default

	let i = 1; // default
	let i32 i = 1;

A tuple mixes types

	let tup = (i, f, true);
	println!("stuff: {:?}", tup);

Single tuples require a trailing comma

	let nt = (iu32);  // not a tuple
	let yt = (iu32,); // is a tuple because of the trailing comma

Tuples can be unpacked or indexed

	let (a, b, c) = tup;
	println!("{} {}", a, tup.0);

An array is a collection of objects of the same type of FIXED length. A 