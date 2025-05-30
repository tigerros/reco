[![tests](https://img.shields.io/github/actions/workflow/status/tigerros/reco/test.yml?label=tests)](https://github.com/tigerros/reco/actions/workflows/test.yml)
[![clippy](https://img.shields.io/github/actions/workflow/status/tigerros/reco/clippy.yml?label=clippy)](https://github.com/tigerros/reco/actions/workflows/clippy.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/reco)](https://app.codecov.io/gh/tigerros/reco/)
[![docs.rs](https://img.shields.io/docsrs/reco?logo=docs.rs&label=docs.rs)](https://docs.rs/reco/)
[![crates.io](https://img.shields.io/crates/v/reco?logo=rust)](https://crates.io/crates/reco)

# reco
**R**ust **E**ncyclopaedia of **C**hess **O**penings.

This crate is an implementation of the [ECO](https://en.wikipedia.org/wiki/Encyclopaedia_of_Chess_Openings) standard using [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.

Contains:
- Types to represent each part of an ECO entry.
- ECO entries as constants, sourced from [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).

`#![no_std]` compatible. Doesn't allocate, but exposes `alloc::borrow::Cow`.

## Features
- `book`: adds ECO entries as constants. Because there's a lot of them, it might worsen compilation performance.
- `alloc`: adds the `OpeningOwned` struct, which is an owned version of the `Opening` struct.
- `serde`: enables serde support for applicable types. **All implementations are derived with no parameters. This includes types that have a `FromStr` and `Display` implementation.**

## Sourcing
The ECO entries are generated using the unpublished `librarian` crate.
To make sure they are up to date, there's a workflow that runs `librarian` every day and makes a PR if any changes are detected.

See [`librarian`'s README](https://github.com/tigerros/reco/tree/master/librarian/README.md) for information about running.

## Safety
`reco` declares `#![forbid(unsafe_code)]`.

## Cloning
If you clone this repository, **exclude the `src/book` directory from your IDE.**
Only open the files in there with a well optimized text editor.