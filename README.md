

## GitHub Releases Download Counter

Small CLI util to count the number of downloads of all assets from a GitHub releases feed.

### Build
*requires Rust toolchain
`cargo build`

### Running
- `cargo run <github url>`
- `github-releases-download-counter <github url>` (if already built)

e.g.
```
target/debug/github-releases-download-counter https://github.com/jroddev/android-oss-release-tracker
Calculating Download Count for https://github.com/jroddev/android-oss-release-tracker
Total Downloads for all assets in all Releases: 249
```
