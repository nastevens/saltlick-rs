// Copyright (c) 2019, Nick Stevens <nick@bitcurry.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A library for encrypting and decrypting file streams using libsodium.
//!
//! This library provides a Rust implementation of the saltlick binary file
//! format, which is itself a format for encrypting and decrypting files using
//! strong elliptic curve cryptography. See the [saltlick spec] for details
//! about the motivation and implementation of the file format itself.
//!
//! Both low-level and high-level APIs are provided. The low-level API requires
//! manually updating an encrypter with chunks of plaintext and receiving
//! ciphertext, or updating a decrypter with chunks of ciphertext and receiving
//! plaintext. The current high-level API implements Rust's [`Read`] and
//! [`Write`] traits to provide a simple to use way to read and write files.
//!
//! [saltlick spec]: https://github.com/saltlick-crypto/saltlick-spec
//! [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
//! [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
//!
//! # Usage
//!
//! First, add this to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! saltlick = "0.2"
//! ```
//!
//! Next:
//!
//! ```
//! use saltlick::{
//!     read::SaltlickDecrypter,
//!     write::SaltlickEncrypter,
//!     SaltlickError,
//! };
//! use std::{
//!     error::Error,
//!     fs::File,
//!     io::{self, Cursor, Read, Write},
//! };
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // Generate a new public/secret keypair
//!     let (public, secret) = saltlick::gen_keypair();
//!
//!     // Writing data to a stream
//!     let writer = Vec::new();
//!     let mut stream = SaltlickEncrypter::new(public.clone(), writer);
//!     stream.write_all(b"I have a secret for you")?;
//!     let ciphertext = stream.finalize()?;
//!
//!     // Reading data back from stream
//!     let reader = Cursor::new(ciphertext);
//!     let mut stream = SaltlickDecrypter::new(public.clone(), secret.clone(), reader);
//!     let mut output = String::new();
//!     stream.read_to_string(&mut output)?;
//!     assert_eq!("I have a secret for you", output);
//!
//!     // Save public and private keys as PEM format
//!     let public_pem = public.to_pem();
//!     let secret_pem = secret.to_pem();
//!
//!     Ok(())
//! }
//! ```
//!
//! # Generating Keys
//!
//! In addition to generating keys programmatically, it is possible to generate
//! compliant key files with OpenSSL 1.1.0 or newer:
//!
//! ```sh
//! openssl genpkey -algorithm x25519 > secret.pem
//! openssl pkey -in secret.pem -pubout > public.pem
//! ```

pub mod bufread;
pub mod crypter;
pub mod read;
pub mod write;

mod error;
mod key;
mod state;
mod sync;
mod version;

pub use self::{
    error::{SaltlickError, SaltlickKeyIoError},
    key::{gen_keypair, PublicKey, SecretKey, PUBLICKEYBYTES, SECRETKEYBYTES},
    version::Version,
};
