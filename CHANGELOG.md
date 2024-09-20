# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

# Unreleased

Nothing.

# 0.6.0 (20 Sep 2024)

- **updated**: [rusqlite] version to `0.32`.
- **added**: `bundled` feature corresponding to `rusqlite/bundled` feature.
- **added**: Implement `From<rusqlite::Connection>` for `Connection`.
- **removed**: async `Connection::from` implementation instead added above.

# 0.5.1 (13 Feb 2024)

- **updated:** To latest [rusqlite] version (`0.31`).
- **added**: Reexported all names from `rusqlite` crate.

# 0.5.0 (25 Nov 2023)

- **updated:** To latest [rusqlite] version (`0.30`).
- **added:** Added `params` macro export from `rusqlite` crate.
- **breaking:** `Connection::call` now takes `tokio_rusqlite::Result` instead
  of `rusqlite::Result`.
- **added:** Added `Error::Other` variant for application
  specific errors.
- **added:** Added `Connection::call_unwrap` method.

# 0.4.0 (3 April 2023)

- **added:** Added `Connection::close` method.
- **added:** Added `tokio_rusqlite::Error` type.
- **breaking:** All `Connection` methods now return `Result<_, tokio_rusqlite::Error>`.
- **updated:** To latest [rusqlite] version (`0.29`).

# 0.3.0 (16 Sep 2022)

- **updated:** To latest [rusqlite] version (`0.28`).

# 0.2.0 (13 July 2022)

- **changed:** Now using unbounded `crossbeam-channel` instead of bounded
  `std::sync::mpsc` channel internally.
- **changed:** Channel send errors in background database thread are now
  ignored instead of panicking.

# 0.1.0 (25 April 2022)

- Initial release.

[rusqlite]: https://crates.io/crates/rusqlite
[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
