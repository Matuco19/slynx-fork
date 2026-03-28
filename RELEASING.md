# Releasing Slynx

This document defines how Slynx should publish version tags and GitHub Releases.

## Current Public State

The repository currently has a public tag, `v0.0.1`, created on 2026-03-21.

At the time of writing, there is still no published GitHub Release entry for that tag. That means tags already exist in the repository, but the GitHub Releases page is still effectively empty.

## Goals

- keep releases predictable
- keep tags consistent
- make experimental releases explicit
- avoid one-off manual release flows

## Tag Format

Repository releases should use Semantic Versioning tags:

- `v0.0.1`
- `v0.1.0`
- `v0.2.3`
- `v0.2.3-rc.1`

Always prefix the version with `v`.

## When to Cut a Release

Create a release only from `main`, after:

1. the release scope is merged
2. CI is green
3. `CHANGELOG.md` has an entry for the release
4. maintainers agree on the version number

## Release Checklist

1. Update `CHANGELOG.md`
2. Review `README.md` and related docs if the release changes developer-facing behavior
3. Confirm CI is green on `main`
4. Create an annotated tag from `main`
5. Push the tag
6. Let GitHub Actions create the release
7. Review the generated notes and edit if needed

## Create the Tag

```bash
git checkout main
git pull --ff-only origin main
git tag -a v0.0.1 -m "Slynx v0.0.1"
git push origin v0.0.1
```

## GitHub Release Behavior

Pushing a `v*` tag triggers the release workflow in `.github/workflows/release.yml`.

That workflow:

- validates the workspace with `cargo test`, `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo build --release`
- creates a GitHub Release from the tag
- uses GitHub-generated release notes

For now, releases are expected to be source releases only. Binary assets can be added later when the project decides how it wants to ship official tooling.

## Experimental Status

Slynx is still experimental. Early releases should say that clearly in the release notes.

If a release is intended as an unstable preview, use a prerelease-style tag such as:

- `v0.0.1-alpha.1`
- `v0.0.1-beta.1`
- `v0.0.1-rc.1`

## Notes on Crate Versions

The repository tag is the public release identifier.

If maintainers want crate metadata to match the repository release, update the `version` fields in:

- `Cargo.toml`
- `common/Cargo.toml`
- `frontend/Cargo.toml`
- `middleend/Cargo.toml`

before cutting the tag.
