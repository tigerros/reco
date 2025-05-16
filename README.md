# reco
**R**ust **E**ncyclopaedia of **C**hess **O**penings.

This crate is an implementation of the [ECO](https://en.wikipedia.org/wiki/Encyclopaedia_of_Chess_Openings) standard using [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.

Contains:
- Types to represent each part of an ECO entry.
- ECO entries as constants, sourced from [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).

No-std and no-alloc compatible.

## Features
- `book`: adds ECO entries as constants. Because there's a lot of them, it might worsen compilation performance.
- `alloc`: adds the `OpeningOwned` struct, which is an owned version of the `Opening` struct.
- `serde`: enables serde support for applicable types. **All implementations are derived with no parameters. This includes types that have a `FromStr` and `Display` implementation.**

## Sourcing
The ECO entries are generated using the unpublished `librarian` crate.
To make sure they are up to date, there's a workflow that runs `librarian` every day and makes a PR if any changes are detected.

See [`librarian`'s README](https://github.com/tigerros/reco/tree/master/librarian/README.md) for information about running.

## Cloning
If you clone this repository, **exclude the `src/book` directory from your IDE.**
Only open the files in there with a well optimized text editor.