**Not released yet due to a crucial feature in shakmaty not being published yet ([shakmaty/#92](https://github.com/niklasf/shakmaty/pull/92)).**

[![tests](https://img.shields.io/github/actions/workflow/status/tigerros/reco/test.yml?label=tests)](https://github.com/tigerros/reco/actions/workflows/test.yml)
[![clippy](https://img.shields.io/github/actions/workflow/status/tigerros/reco/clippy.yml?label=clippy)](https://github.com/tigerros/reco/actions/workflows/clippy.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/reco)](https://app.codecov.io/gh/tigerros/reco/)
[![docs.rs](https://img.shields.io/docsrs/reco?logo=docs.rs&label=docs.rs)](https://docs.rs/reco/)
[![crates.io](https://img.shields.io/crates/v/reco?logo=rust)](https://crates.io/crates/reco)

# reco
**R**ust **E**ncyclopaedia of **C**hess **O**penings.

This crate is an implementation of the [ECO](https://en.wikipedia.org/wiki/Encyclopaedia_of_Chess_Openings) database using [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.

Contains:
- Types to represent each part of an ECO entry.
- ECO entries as statics, sourced from [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).

no-std and no-alloc compatible.

## Features
- `alloc`: adds functionality which requires the `alloc` crate.
- `std` (default): adds functionality which requires `std`. Highly recommended if `std` is available. Most notably, it adds a lazily evaluated `book::SETUP_TO_LINE` static that maps `shakmaty::Setup`s to `Line`s drastically improving opening lookup time.
- `book`: adds ECO entries as statics.
- `book-flattened`: adds a compile time evaluated `book::FLATTENED` static that flattens the tree structure of `book::ALL`.
- `serde`: adds [`serde`](https://crates.io/crates/serde) support for applicable types. **All implementations are derived with no parameters. This includes types that have a `FromStr` and `Display` implementation.**
- `proptest`: adds some [`proptest`](https://crates.io/crates/proptest) support. Requires `std`.

## Sourcing
The ECO entries are generated using the unpublished `librarian` crate.
To make sure they are up to date, there's a workflow that runs `librarian` every day and makes a PR if any changes are detected.

See [`librarian`'s README](https://github.com/tigerros/reco/blob/master/librarian/README.md) for information about running.

## Safety
`reco` declares `#![forbid(unsafe_code)]`.

## Cloning
If you clone this repository, **exclude the `src/book_gen` directory from your IDE.**
Open the files in there with a well optimized text editor.