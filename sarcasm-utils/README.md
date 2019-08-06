# sarcasm-utils

SaRcAsM tExT creation and validation library.

This library provides utilities for creating and validating the capitalization style known as
SaRcAsM tExT. It is the library behind the command line utility `sarcasm` which provides a
convenient interface for day-to-day use, including automatic copying to the clipboard.

All steps have been taken to make the library production ready. It has full logging via the
`log` crate and code should be panic free unless otherwise noted. If a panic is found in a
non-labeled function, this is a bug.

Feature requests, pull requests, and issues are all welcome.

## Example

For more details, see [`encode_sarcasm`] and [`is_sarcasm`].

```rust,edition2018
use sarcasm_utils::{StartingCase, is_sarcasm, IsSarcasm};
use sarcasm_utils::encode_sarcasm;

// Encoding SaRcAsM
let encoded: String = encode_sarcasm("Hello!", StartingCase::Uppercase);
assert_eq!(encoded, "HeLlO!");

// Decoding SaRcAsM
let decoded: IsSarcasm = is_sarcasm(&encoded);
assert_eq!(decoded, IsSarcasm::Yes(StartingCase::Uppercase));

// Matching on the type of SaRcAsM
match decoded {
    IsSarcasm::Yes(StartingCase::Uppercase) => println!("UpPeRcAsE sArCaSm!"),
    IsSarcasm::Yes(StartingCase::Lowercase) => println!("lOwErCaSe SaRcAsM!"),
    IsSarcasm::No => println!("No sarcasm here!"),
    IsSarcasm::TooShort => println!("Can't tell if there's sarcasm!"),
}
```

## Why?

Because.

## Is this a joke?

No.

## Really?

Okay, yes the purpose is silly, but this was an exersize in writing a library in rust
that is designed to be robust yet easy to use. Also, it does provide some (minimal) use for
someone who wants to do this kind of thing. I will do my best to maintain and attend to this
library, as well as expand it with new "useful" features in the future.

In short: maybe?

## Be Nice

Don't use this library to be jerks to people. It ruins the fun.

License: MIT
