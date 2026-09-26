# Changelog

## 0.1.0

- `#[derive(Links)]` loads every SST link a struct declares in one call; field names map to link names in PascalCase,
  `#[links(name = "…")]` overrides one.
- Link shapes for `App`, `Bucket`, `Dynamo`, `Function`, `Realtime`, `Router`, and `Secret` (no `Debug`).
- `Error` names the missing or misshapen link.
