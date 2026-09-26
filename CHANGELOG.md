# Changelog

## 0.1.0

- `#[derive(Deserialize, Links)]` + `load()` reads every SST link into one struct; serde maps fields to link names.
- Link shapes for `App`, `Bucket`, `Dynamo`, `Function`, `Realtime`, `Router`, and `Secret` (no `Debug`).
