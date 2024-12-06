# expr-rust-git2-merge-base-cmds

Experiment with how the various rust git2 merge_base commands work

# Run

```sh
wink@3900x 24-12-06T19:33:32.221Z:~/prgs/rust/myrepos/explr-rust-git2-merge-base-cmds (main)
$ cargo run . a666969b6d98f74eb66cbc99f220ec57ca366044 a666969b6d98f74eb66cbc99f220ec57ca366044
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/explr-git2-merge . a666969b6d98f74eb66cbc99f220ec57ca366044 a666969b6d98f74eb66cbc99f220ec57ca366044`
merge_base result: a666969b6d98f74eb66cbc99f220ec57ca366044
merge_base_many result: a666969b6d98f74eb66cbc99f220ec57ca366044
merge_bases result: OidArray([a666969b6d98f74eb66cbc99f220ec57ca366044])
merge_base_octopus result: a666969b6d98f74eb66cbc99f220ec57ca366044
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
