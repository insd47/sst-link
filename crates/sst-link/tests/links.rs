use serde::Deserialize;
use sst_link::{App, Bucket, Dynamo, Error, Function, Links, Realtime, Router, Secret};
use std::process::Command;

#[derive(Deserialize)]
struct Registry {
    audience: String,
}

#[derive(Links)]
struct Resources {
    app: App,
    table: Dynamo,
    judge: Function,
    realtime: Realtime,
    router: Router,
    key: Secret,
    registry: Registry,
    #[links(name = "RouterStorage")]
    storage: Bucket,
    r#type: Bucket,
}

const CHILD: &str = "SST_LINK_TEST_CHILD";

/// Resources are handed over the way SST hands them to a Lambda, extra fields included. The test re-runs itself so the
/// parent process's environment stays untouched.
#[test]
fn loads_every_link_at_once() {
    if std::env::var(CHILD).is_ok() {
        let resources = Resources::load().expect("links");

        assert_eq!(resources.app.stage, "production");
        assert_eq!(resources.table.name, "table");
        assert_eq!(resources.judge.name, "judge");
        assert_eq!(resources.realtime.authorizer, "authorizer");
        assert_eq!(resources.router.url, "https://example.com");
        assert_eq!(resources.key.value, "secret");
        assert_eq!(resources.registry.audience, "crates.kitpa.org");
        assert_eq!(resources.storage.name, "bucket");
        assert_eq!(resources.r#type.name, "type");
        return;
    }

    assert!(run(&[
        ("App", r#"{"name":"app","stage":"production"}"#),
        ("Table", r#"{"name":"table","type":"sst.aws.Dynamo"}"#),
        (
            "Judge",
            r#"{"name":"judge","url":"https://judge.lambda-url","type":"sst.aws.Function"}"#
        ),
        (
            "Realtime",
            r#"{"endpoint":"x.iot.ap-northeast-2.amazonaws.com","authorizer":"authorizer"}"#
        ),
        ("Router", r#"{"url":"https://example.com","type":"sst.aws.Router"}"#),
        ("Key", r#"{"value":"secret","type":"sst.sst.Secret"}"#),
        ("Registry", r#"{"audience":"crates.kitpa.org","organization":"1"}"#),
        ("RouterStorage", r#"{"name":"bucket","type":"sst.aws.Bucket"}"#),
        ("Type", r#"{"name":"type"}"#),
    ]));
}

#[test]
fn names_the_missing_link() {
    if std::env::var(CHILD).is_ok() {
        let error = Resources::load().err().expect("missing links must fail");

        assert!(matches!(&error, Error::Missing(name) if name == "App"), "{error}");
        return;
    }

    assert!(run(&[]));
}

#[test]
fn names_the_misshapen_link() {
    if std::env::var(CHILD).is_ok() {
        let error = Resources::load().err().expect("misshapen links must fail");

        assert!(matches!(&error, Error::Shape { name, .. } if name == "App"), "{error}");
        return;
    }

    assert!(run(&[("App", r#"{"name":"app"}"#)]));
}

fn run(links: &[(&str, &str)]) -> bool {
    let test = std::thread::current().name().expect("test name").to_string();
    let mut command = Command::new(std::env::current_exe().expect("test binary"));

    command.args([test.as_str(), "--exact", "--nocapture"]).env(CHILD, "1");

    for (name, value) in links {
        command.env(format!("SST_RESOURCE_{name}"), value);
    }

    command.status().expect("child test").success()
}
