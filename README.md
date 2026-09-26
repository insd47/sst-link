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

`load` hands serde an object keyed by link name, so mapping fields to links is serde's job — `rename_all`, `rename`,
`default`, and `flatten` all work as usual.

## Shapes

A field can be any `Deserialize` type. The shapes of SST's own components ship with this crate:

| Type | Component | Fields |
| --- | --- | --- |
| `App` | linked to every function | `name`, `stage` |
| `Bucket` | `sst.aws.Bucket` | `name` |
| `Dynamo` | `sst.aws.Dynamo` | `name` |
| `Function` | `sst.aws.Function` | `name` |
| `Realtime` | `sst.aws.Realtime` | `endpoint`, `authorizer` |
| `Router` | `sst.aws.Router` | `url` |
| `Secret` | `sst.Secret` | `value` (no `Debug`, so it can't end up in a log) |

Your own `sst.Linkable` (or a component's `getSSTLink()`) brings its own struct:

```rust
#[derive(Deserialize)]
struct Registry {
    bucket: String,
    audience: String,
}
```

## License

MIT
