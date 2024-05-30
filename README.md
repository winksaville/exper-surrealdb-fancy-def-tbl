# exper-surrealdb-fancy-def-tbl

Experiment with SurrealDB fancy `DEFINE TABLE` that
[matiu on SurrealDB discord](https://discord.com/channels/902568124350599239/1014970959461105664/1245640979667484703)
was having. Here is the code:
```rust
let surql = "DEFINE TABLE amt_item SCHEMAFULL CHANGEFEED 365d INCLUDE ORIGINAL PERMISSIONS NONE;";
db.query(surql).await?;
```

The error was:
```bash
Db(InvalidQuery(RenderedError { text: "Failed to parse query at line 1 column 8 expect
       │ ed query to end", snippets: [Snippet { source: "DEFINE TABLE amt_item SCHEMAFULL CHANGEFEED 365d INCLUDE ORIGINAL PERMISSIONS", tru
       │ ncation: End, location: Location { line: 1, column: 8 }, offset: 7, length: 1, explain: Some("perhaps missing a semicolon on the pr
       │ evious statement?") }] }))
```

Which was similar to the trouble I was having with `DEFINE DATABASE` and I suggested adding "sql2"
see below.

## Clone repo, build and run

```bash
git clone github.com:winksaville/exper-surrealdb-fancy-def-tbl
cd exper-surrealdb-fancy-def-tbl
```

Use `cargo build` and/or `cargo run`

## Experiments

### Using surrealdb crate

With version 1.5.0 or 1.5.1 but with no "sql2" feature:
```toml
surrealdb = { version = "1.5.1", features = ["kv-mem"] } # FAILS
```

Compilation fails with the following:
```bash
wink@3900x 24-05-30T17:03:22.534Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
$ cargo run
   Compiling proc-macro2 v1.0.84
   Compiling unicode-ident v1.0.12
   Compiling autocfg v1.3.0
...
   Compiling cedar-policy v2.4.2
   Compiling surrealdb-core v2.0.0-1.5.1
   Compiling surrealdb-core v1.4.2
   Compiling surrealdb v1.5.1
   Compiling exper-surrealdb-fancy-def-tbl v0.1.0 (/home/wink/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 53.12s
     Running `target/debug/exper-surrealdb-fancy-def-tbl`
Error: Parse error: Failed to parse query at line 1 column 8 expected query to end
  |
1 | DEFINE TABLE amt_item SCHEMAFULL CHANGEFEED 365d INCLUDE ORIGINAL PERMISSIONS...
  |        ^ perhaps missing a semicolon on the previous statement?

Error: Db(InvalidQuery(RenderedError { text: "Failed to parse query at line 1 column 8 expected query to end", snippets: [Snippet { source: "DEFINE TABLE amt_item SCHEMAFULL CHANGEFEED 365d INCLUDE ORIGINAL PERMISSIONS", truncation: End, location: Location { line: 1, column: 8 }, offset: 7, length: 1, explain: Some("perhaps missing a semicolon on the previous statement?") }] }))
wink@3900x 24-05-30T17:04:24.395Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
```

If you add feature "sql2":
```toml
surrealdb = { version = "1.5.1", features = ["kv-mem", "sql2"] }`
```

It compiles and runs:
```bash
wink@3900x 24-05-30T17:05:45.508Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
$ cargo clean
     Removed 2847 files, 2.4GiB total
wink@3900x 24-05-30T17:05:50.012Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
$ cargo run
   Compiling proc-macro2 v1.0.84
   Compiling unicode-ident v1.0.12
   Compiling autocfg v1.3.0
...
   Compiling cedar-policy v2.4.2
   Compiling surrealdb-core v2.0.0-1.5.1
   Compiling surrealdb-core v1.4.2
   Compiling surrealdb v1.5.1
   Compiling exper-surrealdb-fancy-def-tbl v0.1.0 (/home/wink/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 53.94s
     Running `target/debug/exper-surrealdb-fancy-def-tbl`
Successfully created DB
[src/main.rs:32:13] &response = Response {
    client: Surreal {
        router: OnceLock(
            Router {
                sender: Sender,
                last_id: 0,
                features: {
                    LiveQueries,
                    Backup,
                },
            },
        ),
        engine: PhantomData<surrealdb::api::engine::any::Any>,
    },
    results: {
        0: (
            Stats {
                execution_time: Some(
                    437.08µs,
                ),
            },
            Ok(
                None,
            ),
        ),
    },
    live_queries: {},
}
wink@3900x 24-05-30T17:06:45.937Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
```

### Using a local clone/fork of surrealdb

Here I've locally cloned surrealdb to `~/prgs/SurrealDB/surrealdb`.

Using "tip of tree" of surrealdb (i.e. version 2.0.0) and "sql2":
```toml
surrealdb = { path = "../surrealdb/lib", features = ["kv-mem", "sql2"] } # FAILS
```

It fails because feature "sql2" is not valid.
```bash
wink@3900x 24-05-30T17:35:52.570Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
$ cargo clean
     Removed 2849 files, 2.5GiB total
wink@3900x 24-05-30T17:35:59.976Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
$ cargo run
    Updating crates.io index
error: failed to select a version for `surrealdb`.
    ... required by package `exper-surrealdb-fancy-def-tbl v0.1.0 (/home/wink/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl)`
versions that meet the requirements `*` are: 2.0.0

the package `exper-surrealdb-fancy-def-tbl` depends on `surrealdb`, with features: `sql2` but `surrealdb` does not have these features.


failed to select a version for `surrealdb` which could resolve this conflict
wink@3900x 24-05-30T17:36:03.101Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
```

But if you now remove "sql2":
```toml
surrealdb = { path = "../surrealdb/lib", features = ["kv-mem"] } # OK
```

It works:
```bash
wink@3900x 24-05-30T17:36:03.101Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
$ cargo clean
     Removed 0 files
wink@3900x 24-05-30T17:38:01.226Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
$ cargo run
    Updating crates.io index
   Compiling proc-macro2 v1.0.84
   Compiling unicode-ident v1.0.12
   Compiling autocfg v1.3.0
...
   Compiling cedar-policy v2.4.2
   Compiling surrealdb-core v2.0.0 (/home/wink/prgs/SurrealDB/surrealdb/core)
   Compiling surrealdb v2.0.0 (/home/wink/prgs/SurrealDB/surrealdb/lib)
   Compiling exper-surrealdb-fancy-def-tbl v0.1.0 (/home/wink/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 51.16s
     Running `target/debug/exper-surrealdb-fancy-def-tbl`
wink: Define: Table(DefineTableStatement { id: None, name: Ident("amt_item"), drop: false, full: true, view: None, permissions: Permissions { select: None, create: None, update: None, delete: None }, changefeed: Some(ChangeFeed { expiry: 31536000s, store_diff: true }), comment: None, if_not_exists: false, kind: Any })
Successfully created DB
[src/main.rs:32:13] &response = Response {
    client: Surreal {
        router: OnceLock(
            Router {
                sender: Sender,
                last_id: 0,
                features: {
                    LiveQueries,
                    Backup,
                },
            },
        ),
        engine: PhantomData<surrealdb::api::engine::any::Any>,
    },
    results: {
        0: (
            Stats {
                execution_time: Some(
                    514.827µs,
                ),
            },
            Ok(
                None,
            ),
        ),
    },
    live_queries: {},
}
wink@3900x 24-05-30T17:38:54.095Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
```

Also ChatGPT4o informed me that you can use `cargo add surrealdb --path ~/prgs/SurrealDB/surrealdb/lib --features kv-mem`
to add a local rust library package to an app.
```bash
cargo add surrealdb --path ~/prgs/SurrealDB/surrealdb/lib --features kv-mem
```

That results in the apps `Cargo.toml` having a dependency:
```toml
surrealdb = { version = "2.0.0", path = "../surrealdb/lib", features = ["kv-mem"] } # OK
```

And the `cargo run` is just as if you'd manually installed:
```bash
wink@3900x 24-05-30T18:30:21.993Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
$ cargo clean
     Removed 4107 files, 5.5GiB total
wink@3900x 24-05-30T18:30:25.683Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
$ cargo run
   Compiling proc-macro2 v1.0.84
   Compiling unicode-ident v1.0.12
   Compiling autocfg v1.3.0
...
   Compiling cedar-policy-validator v2.4.2
   Compiling cedar-policy v2.4.2
   Compiling surrealdb-core v2.0.0 (/home/wink/prgs/SurrealDB/surrealdb/core)
   Compiling surrealdb v2.0.0 (/home/wink/prgs/SurrealDB/surrealdb/lib)
   Compiling exper-surrealdb-fancy-def-tbl v0.1.0 (/home/wink/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 50.31s
     Running `target/debug/exper-surrealdb-fancy-def-tbl`
wink: Define: Table(DefineTableStatement { id: None, name: Ident("amt_item"), drop: false, full: true, view: None, permissions: Permissions { select: None, create: None, update: None, delete: None }, changefeed: Some(ChangeFeed { expiry: 31536000s, store_diff: true }), comment: None, if_not_exists: false, kind: Any })
Successfully created DB
[src/main.rs:32:13] &response = Response {
    client: Surreal {
        router: OnceLock(
            Router {
                sender: Sender,
                last_id: 0,
                features: {
                    Backup,
                    LiveQueries,
                },
            },
        ),
        engine: PhantomData<surrealdb::api::engine::any::Any>,
    },
    results: {
        0: (
            Stats {
                execution_time: Some(
                    408.065µs,
                ),
            },
            Ok(
                None,
            ),
        ),
    },
    live_queries: {},
}
wink@3900x 24-05-30T18:31:19.383Z:~/prgs/SurrealDB/exper-surrealdb-fancy-def-tbl (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
