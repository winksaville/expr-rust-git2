# expr-rust-git2-merge-base-cmds

Experiment with how the various rust git2 merge_base commands work

# Run

From the root of the project:
```sh
wink@3900x 24-12-04T17:52:41.070Z:~/prgs/rust/myrepos/expr-rust-git2 (explr-merge-base)
$ cargo run --bin explr-merge-base . f1082199ecc345ad28a99f7de8b151de6951405f 86e22f70e8cbae19a41827450d308e1c98c882fb
   Compiling explr-merge-base v0.1.0 (/home/wink/prgs/rust/myrepos/expr-rust-git2/explr-merge-base)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/explr-merge-base . f1082199ecc345ad28a99f7de8b151de6951405f 86e22f70e8cbae19a41827450d308e1c98c882fb`
86e22f70e8cbae19a41827450d308e1c98c882fb
wink@3900x 24-12-04T17:52:58.924Z:~/prgs/rust/myrepos/expr-rust-git2 (explr-merge-base)
```

Or from within the explr-merge-base directory:
```sh
wink@3900x 24-12-04T17:53:33.384Z:~/prgs/rust/myrepos/expr-rust-git2 (explr-merge-base)
$ cd explr-merge-base
wink@3900x 24-12-04T17:53:38.463Z:~/prgs/rust/myrepos/expr-rust-git2/explr-merge-base (explr-merge-base)
$ cargo run ../ f1082199ecc345ad28a99f7de8b151de6951405f 86e22f70e8cbae19a41827450d308e1c98c882fb
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `/home/wink/prgs/rust/myrepos/expr-rust-git2/target/debug/explr-merge-base ../ f1082199ecc345ad28a99f7de8b151de6951405f 86e22f70e8cbae19a41827450d308e1c98c882fb`
86e22f70e8cbae19a41827450d308e1c98c882fb
wink@3900x 24-12-04T17:53:52.398Z:~/prgs/rust/myrepos/expr-rust-git2/explr-merge-base (explr-merge-base)
```
