# sarcasm

A library ([`sarcasm-utils`](sarcasm-utils/README.md)) and command line tool (`sarcasm`) for creating and validating SaRcAsM tExT.

The command line tool can be used to easily create and validate SaRcAsM tExT from the comfort of your command line.

Advanced features include the results automatically being copied to the clipboard for easy pasting into
your chat message box or CMS of choice. Offers full unicode support as offered by rust's standard library.

- [Usage](#usage)
- [Install](#install)
- [All Options](#all-options)
- [Why?](#why)
- [Be Nice](#be-nice)

## Usage

The command line interface intends to be as friendly to users and to scripts as possible.

### Create SaRcAsM

```
$ sarcasm Hello World Sarcasm Style!
HeLlO WoRlD SaRcAsM StYlE!
```

Add `-u` to make UpPeRcAsE sArCaSm (default).

```
$ sarcasm -u Uppercase Sarcasm!
UpPeRcAsE SaRcAsM!
```

Add `-l` to make lOwErCaSe sArCaSm.

```
$ sarcasm -l Uppercase Sarcasm!
lOwErCaSe sArCaSm!
```

### Validate SaRcAsM

Add `--check` to validate an input string for correctness.

```
$ sarcasm --check HeLlO WoRlD SaRcAsM StYlE!
Yes, this is UpPeRcAsE sArCaSm!
# Returns 0
```

```
$ sarcasm --check HeLlO WoRlD SaRcAsM StYlE!
Yes, this is lOwErCaSe SaRcAsM!
# Returns 0
```

```
$ sarcasm --check Hello World Sarcasm Style!
No, this is not sarcasm!
# Returns 1
```

```
$ sarcasm --check !!
I can't tell if this is sarcasm!
# Returns 2
```

#### Edge Cases

All non-alphanumeric characters will be ignored in the input. As a result,
the SaRcAsM pattern only has to hold within groups of alphanumeric characters.
This means the starting letter of any "secondary" alphanumeric groups can be any case.

While the SaRcAsM creator is well defined in the capitalization throughout the whole string,
the validator needs to be able to deal with many possible styles of writing SaRcAsM text.

 - `AbC DeF` and `AbC dEf` -> SaRcAsM.
 - `Ab-Cd` and `Ab-cD` -> SaRcAsM.
 - `A` -> SaRcAsM.
 - `!!` -> Too Short (no alphanumeric characters to judge).
 - `A!A` and `A!a` -> SaRcAsM.

## Install

There are currently two ways to install `sarcasm` onto your computer.

### Binary Releases

Releases are available for windows, mac, and linux via [github releases](https://github.com/cwfitzgerald/sarcasm-rs/releases).
Put the release in your `PATH` and you're off to the races.

### Cargo

Releases are also available through cargo if you are a rust developer. 

```
cargo install sarcasm
```

## All Options

```
sarcasm 0.1.0
Connor Fitzgerald <connorwadefitzgerald@gmail.com>
A tool to generate and validate SaRcAsM TeXt.

While the program itself is a bit pointless, it is designed to be a fully robust program and library that you can use
anywhere in real projects, if you had any actual need.

USAGE:
    sarcasm.exe [FLAGS] [OPTIONS] [text]...

FLAGS:
    -c, --check        Check input text for sarcasm type
        --clipboard    Force clipboard use
    -h, --help         Prints help information
    -l, --lowercase    Output lowercase sArCaSm tExT
    -u, --uppercase    Output uppercase SaRcAsM TeXt
    -V, --version      Prints version information
    -v, --verbose      Log output level (-v, -vv, -vvv)

OPTIONS:
        --log <log_file>     Full verbosity log file. Appends to end. Always sends warnings and errors to stderr.
    -o, --output <output>    Output to a file instead of standard out.

ARGS:
    <text>...    Text to process
```

## Why?

Because.

### Is this a joke?

No.

### Really?

Okay, yes the purpose is silly, but this was an exercise in writing a library in rust
that is designed to be robust yet easy to use. Also, it does provide some (minimal) use for
someone who wants to do this kind of thing. I will do my best to maintain and attend to this
library, as well as expand it with new "useful" features in the future.

In short: maybe?

## Be Nice

Don't use this tool to be jerks to people. It ruins the fun.

License: MIT
