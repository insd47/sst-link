# sst-link

Typed SST resource links for Rust.

Declare one struct per function that mirrors its `link` array, derive `Links`, and load it once at startup. Every field
is required, so a missing link fails at cold start with its name — not halfway through a request.

```ts
// sst.config.ts
const key = new sst.Secret('Key');
const assets = new sst.aws.Bucket('Assets');

new sst.aws.Function('Api', { handler: '.api', runtime: 'rust', link: [key, assets] });
```

```rust
use sst_link::{Bucket, Links, Secret};

#[derive(Links)]
struct Resources {
    key: Secret,     // link "Key"
    assets: Bucket,  // link "Assets"
}

let resources = Resources::load()?;
```

## Naming

A field reads the link named after it in PascalCase (`key` → `Key`, `registry_key` → `RegistryKey`). When a link's name
doesn't follow its field, name it explicitly:

```rust
#[derive(Links)]
struct Resources {
    #[links(name = "RouterStorage")]
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

#[derive(Links)]
struct Resources {
    registry: Registry,
}
```

`Secret` deliberately has no `Debug`, so a secret can't end up in a log by accident.

## Errors

`load` fails with `sst_link::Error`:

- `Missing(name)` — the function has no link with that name.
- `Shape { name, source }` — the link exists but doesn't deserialize into the field's type.
- `Resource(_)` — SST's resource payload couldn't be read or decrypted.

## License

MIT
