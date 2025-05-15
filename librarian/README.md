# reco-librarian
This is an unpublished crate used to generate files for [`reco`](https://crates.io/crates/reco), using
[lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).

Before running, commit any changes in `src/book`, as that directory will be deleted and replaced.
Additionally, `reco-librarian` requires a `GITHUB_TOKEN` environment variable, which is used to download the artifacts
from workflow runs in [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).
I use a [PAT](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
with zero permissions.
You can create a `.env` file to store it.

If you're worried about security, you can check the `get_archive_and_commit.rs` file and verify
that the token is only being used in retrieving the artifact archive.