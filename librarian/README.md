# librarian
This is an unpublished crate used to generate files for [`reco`](https://crates.io/crates/reco), using
[lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).

Run in the root `reco` directory, with `cargo run -p librarian`.
The `src/book_gen` directory will be deleted and replaced.
Only commit changes if `librarian` prints `success` and `reco` compiles.
Additionally, `librarian` requires a `GITHUB_TOKEN` environment variable, which is used to download the artifacts
from workflow runs in [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).
I use a [PAT](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
with zero permissions.
You can create a `.env` file to store it.