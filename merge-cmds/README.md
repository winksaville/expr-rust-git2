# expr-rust-git2-merge-base-cmds

Experiment with how the various rust git2 merge_base commands work

# Run

Hereis an example of running the program:

```sh
wink@3900x 24-12-07T23:57:48.059Z:~/prgs/rust/myrepos/expr-rust-git2/merge-cmds (explr-rust-git2-merge-cmds)
$ RUST_LOG=info cargo run --bin merge-cmds ../ 985279818e22f81bbcd6860f74df34fca3f04c49 86e22f70e8cbae19a41827450d308e1c98c882fb
   Compiling merge-cmds v0.1.0 (/home/wink/prgs/rust/myrepos/expr-rust-git2/merge-cmds)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s
     Running `/home/wink/prgs/rust/myrepos/expr-rust-git2/target/debug/merge-cmds ../ 985279818e22f81bbcd6860f74df34fca3f04c49 86e22f70e8cbae19a41827450d308e1c98c882fb`
[2024-12-07T23:58:42.323687864Z INFO  merge_cmds   91  1] main:+
[2024-12-07T23:58:42.323718601Z INFO  merge_cmds  110  1] arg: 985279818e22f81bbcd6860f74df34fca3f04c49
[2024-12-07T23:58:42.323723511Z INFO  merge_cmds  110  1] arg: 86e22f70e8cbae19a41827450d308e1c98c882fb
[2024-12-07T23:58:42.323729271Z INFO  merge_cmds    6  1] merge_cmds:+ repo_path: ../, oid_strings: ["985279818e22f81bbcd6860f74df34fca3f04c49", "86e22f70e8cbae19a41827450d308e1c98c882fb"]
[2024-12-07T23:58:42.331202427Z INFO  merge_cmds    8  1] merge_cmds Repository::open result: is_ok=true
[2024-12-07T23:58:42.331233485Z INFO  merge_cmds   57  1] merge_cmds call merge_base oid1: 985279818e22f81bbcd6860f74df34fca3f04c49, oid2: 86e22f70e8cbae19a41827450d308e1c98c882fb
[2024-12-07T23:58:42.331481531Z INFO  merge_cmds   59  1] merge_cmds merge_base result: Ok(86e22f70e8cbae19a41827450d308e1c98c882fb)
[2024-12-07T23:58:42.331491139Z INFO  merge_cmds   61  1] merge_cmds call merge_base_many oids: [985279818e22f81bbcd6860f74df34fca3f04c49, 86e22f70e8cbae19a41827450d308e1c98c882fb]
[2024-12-07T23:58:42.331504193Z INFO  merge_cmds   63  1] merge_cmds merge_base_many result: Ok(86e22f70e8cbae19a41827450d308e1c98c882fb)
[2024-12-07T23:58:42.331509764Z INFO  merge_cmds   65  1] merge_cmds call merge_bases oid1: 985279818e22f81bbcd6860f74df34fca3f04c49 oid2: 86e22f70e8cbae19a41827450d308e1c98c882fb
[2024-12-07T23:58:42.331520985Z INFO  merge_cmds   67  1] merge_cmds merge_bases result: Ok(OidArray([86e22f70e8cbae19a41827450d308e1c98c882fb]))
[2024-12-07T23:58:42.331527587Z INFO  merge_cmds   69  1] merge_cmds call merge_base_otopus oids: [985279818e22f81bbcd6860f74df34fca3f04c49, 86e22f70e8cbae19a41827450d308e1c98c882fb]
[2024-12-07T23:58:42.331538708Z INFO  merge_cmds   71  1] merge_cmds merge_base_octopus result: Ok(86e22f70e8cbae19a41827450d308e1c98c882fb)
[2024-12-07T23:58:42.331544369Z INFO  merge_cmds   73  1] merge_cmds call merge_bases_many oids: [985279818e22f81bbcd6860f74df34fca3f04c49, 86e22f70e8cbae19a41827450d308e1c98c882fb]
[2024-12-07T23:58:42.331555359Z INFO  merge_cmds   75  1] merge_cmds merge_bases_many result: Ok(OidArray([86e22f70e8cbae19a41827450d308e1c98c882fb]))
[2024-12-07T23:58:42.331561270Z INFO  merge_cmds   77  1] merge_cmds:-
[2024-12-07T23:58:42.331586728Z INFO  merge_cmds  117  1] main:-
wink@3900x 24-12-07T23:58:42.335Z:~/prgs/rust/myrepos/expr-rust-git2/merge-cmds (explr-rust-git2-merge-cmds)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
