# sst-link

Typed SST resource links for Rust.

Declare one struct per function that mirrors its `link` array, derive `Deserialize` and `Links`, and load it once at
startup. Every field is required, so a missing link fails at cold start — not halfway through a request.

```ts
// sst.config.ts
const key = new sst.Secret('Key');
const assets = new sst.aws.Bucket('Assets');

new sst.aws.Function('Api', { handler: '.api', runtime: 'rust', link: [key, assets] });
```

```rust
use serde::Deserialize;
use sst_link::{Bucket, Links, Secret};

#[derive(Deserialize, Links)]
#[serde(rename_all = "PascalCase")]
struct Resources {
    key: Secret,    // link "Key"
    assets: Bucket, // link "Assets"
}

let resources = Resources::load()?;
```

## Naming

`load` hands serde an object keyed by link name (`{ "Key": …, "Assets": … }`), so mapping fields to links is serde's
job: `rename_all = "PascalCase"` for the usual case, `rename` for the odd one, and `default`, `flatten`, or anything else
serde offers when you need it.

```rust
#[derive(Deserialize, Links)]
#[serde(rename_all = "PascalCase")]
struct Resources {
    #[serde(rename = "RouterStorage")]
    storage: Bucket,
}
```

## Shapes

A field can be any `serde::Deserialize` type. The shapes of SST's own components ship with this crate:

| Type | Component | Fields |
| --- | --- | --- |
| `App` | linked to every function | `name`, `stage` |
| `Bucket` | `sst.aws.Bucket` | `name` |
| `Dynamo` | `sst.aws.Dynamo` | `name` |
| `Function` | `sst.aws.Function` | `name` |
| `Realtime` | `sst.aws.Realtime` | `endpoint`, `authorizer` |
| `Router` | `sst.aws.Router` | `url` |
| `Secret` | `sst.Secret` | `value` |

Your own `sst.Linkable` (or a component's `getSSTLink()`) brings its own struct:

```rust
#[derive(serde::Deserialize)]
struct Registry {
    bucket: String,
    audience: String,
}

#[derive(Deserialize, Links)]
#[serde(rename_all = "PascalCase")]
struct Resources {
    registry: Registry,
}
```

`Secret` deliberately has no `Debug`, so a secret can't end up in a log by accident.

## Errors

`load` fails with `sst_link::Error`:

- `Deserialize(_)` — a link is missing or doesn't match its field's type. serde's message names the field
  (``missing field `Key` ``).
- `Resource(_)` — SST's resource payload couldn't be read or decrypted.

## Publishing

Bump `version` in the root `Cargo.toml` (and the `=` pin on `sst-link-derive` with it) and push to `main`.
[release-plz](https://release-plz.dev) publishes whatever version isn't on crates.io yet, then tags and releases it.

## License

MIT
