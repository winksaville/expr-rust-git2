# expr-rust-git2/iterate-over-subdir

A simple crate that iterates over the subdirectories of a repo including the root directory.

## Compile

From root of the workspace:
```bash
wink@3900x 24-11-30T18:52:01.465Z:~/prgs/rust/myrepos/expr-rust-git2 (main)
$ cargo build -p iterate-over-subdir
   Compiling proc-macro2 v1.0.92
   Compiling unicode-ident v1.0.14
   Compiling libc v0.2.167
   ...
   Compiling idna v1.0.3
   Compiling url v2.5.4
   Compiling git2 v0.19.0
   Compiling iterate-over-subdir v0.1.0 (/home/wink/prgs/rust/myrepos/expr-rust-git2/iterate-over-subdir)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.24s
wink@3900x 24-11-30T18:53:13.167Z:~/prgs/rust/myrepos/expr-rust-git2 (main)
```

You can also cd into the `iterate-over-subdir` directory and run `cargo build` there.
```bash
wink@3900x 24-11-30T18:56:27.138Z:~/prgs/rust/myrepos/expr-rust-git2 (main)
$ cargo clean
     Removed 0 files
wink@3900x 24-11-30T18:56:29.015Z:~/prgs/rust/myrepos/expr-rust-git2 (main)
$ cd iterate-over-subdir/
wink@3900x 24-11-30T18:56:35.403Z:~/prgs/rust/myrepos/expr-rust-git2/iterate-over-subdir (main)
$ cargo build
   Compiling proc-macro2 v1.0.92
   Compiling unicode-ident v1.0.14
   Compiling libc v0.2.167
   Compiling shlex v1.3.0
   ...
   Compiling idna v1.0.3
   Compiling url v2.5.4
   Compiling git2 v0.19.0
   Compiling iterate-over-subdir v0.1.0 (/home/wink/prgs/rust/myrepos/expr-rust-git2/iterate-over-subdir)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.32s
wink@3900x 24-11-30T18:56:49.252Z:~/prgs/rust/myrepos/expr-rust-git2/iterate-over-subdir (main)
```

## Run

Run from the root of the workspace:
```bash
wink@3900x 24-11-30T18:53:13.167Z:~/prgs/rust/myrepos/expr-rust-git2 (main)
$ cargo run -p iterate-over-subdir . /
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/iterate-over-subdir . /`
1ac0ff8b841d5336575122a833f00d7d0798ba2b: Merge pull request #1 from winksaville/actually-iterate-over-commits-not-files
86e22f70e8cbae19a41827450d308e1c98c882fb: feat: Support root or subdirectories
949749479f6f28f2882ead0b315fb7c00ceb412c: feat: Iterate over commits
fb776b0b1d4a11aefbf0d4fa538a87c50dfa02e1: chore: Add more logging
063e78daf9b97ffea0a1eec02da348a1454b864a: feat: Add logging using my custom_logger
8de6748c19d0a5d5b9f17bc29734a93dfe63cee7: feat: Add parameters to iterate-over-subdir to take parameters
83ac0639ae82f157b949b9cfa7883ca13ea46451: feat: Add iterate-over-subdir
bc5d57ccaa2116fa447a1e86a58d160473817d6d: feat: Initial Commit
wink@3900x 24-11-30T18:54:53.487Z:~/prgs/rust/myrepos/expr-rust-git2 (main)
$ cargo run -p iterate-over-subdir . iterate-over-subdir/
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/iterate-over-subdir . iterate-over-subdir/`
1ac0ff8b841d5336575122a833f00d7d0798ba2b: Merge pull request #1 from winksaville/actually-iterate-over-commits-not-files
86e22f70e8cbae19a41827450d308e1c98c882fb: feat: Support root or subdirectories
949749479f6f28f2882ead0b315fb7c00ceb412c: feat: Iterate over commits
fb776b0b1d4a11aefbf0d4fa538a87c50dfa02e1: chore: Add more logging
063e78daf9b97ffea0a1eec02da348a1454b864a: feat: Add logging using my custom_logger
8de6748c19d0a5d5b9f17bc29734a93dfe63cee7: feat: Add parameters to iterate-over-subdir to take parameters
83ac0639ae82f157b949b9cfa7883ca13ea46451: feat: Add iterate-over-subdir
wink@3900x 24-11-30T18:55:02.863Z:~/prgs/rust/myrepos/expr-rust-git2 (main)
```

Or, as with build, you can cd into the `iterate-over-subdir` directory and run `cargo run` there.
```
wink@3900x 24-11-30T19:12:41.824Z:~/prgs/rust/myrepos/expr-rust-git2/iterate-over-subdir (main)
$ cargo run ../ /
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `/home/wink/prgs/rust/myrepos/expr-rust-git2/target/debug/iterate-over-subdir ../ /`
1ac0ff8b841d5336575122a833f00d7d0798ba2b: Merge pull request #1 from winksaville/actually-iterate-over-commits-not-files
86e22f70e8cbae19a41827450d308e1c98c882fb: feat: Support root or subdirectories
949749479f6f28f2882ead0b315fb7c00ceb412c: feat: Iterate over commits
fb776b0b1d4a11aefbf0d4fa538a87c50dfa02e1: chore: Add more logging
063e78daf9b97ffea0a1eec02da348a1454b864a: feat: Add logging using my custom_logger
8de6748c19d0a5d5b9f17bc29734a93dfe63cee7: feat: Add parameters to iterate-over-subdir to take parameters
83ac0639ae82f157b949b9cfa7883ca13ea46451: feat: Add iterate-over-subdir
bc5d57ccaa2116fa447a1e86a58d160473817d6d: feat: Initial Commit
wink@3900x 24-11-30T19:14:55.875Z:~/prgs/rust/myrepos/expr-rust-git2/iterate-over-subdir (main)
$ cargo run ../ iterate-over-subdir
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/wink/prgs/rust/myrepos/expr-rust-git2/target/debug/iterate-over-subdir ../ iterate-over-subdir`
1ac0ff8b841d5336575122a833f00d7d0798ba2b: Merge pull request #1 from winksaville/actually-iterate-over-commits-not-files
86e22f70e8cbae19a41827450d308e1c98c882fb: feat: Support root or subdirectories
949749479f6f28f2882ead0b315fb7c00ceb412c: feat: Iterate over commits
fb776b0b1d4a11aefbf0d4fa538a87c50dfa02e1: chore: Add more logging
063e78daf9b97ffea0a1eec02da348a1454b864a: feat: Add logging using my custom_logger
8de6748c19d0a5d5b9f17bc29734a93dfe63cee7: feat: Add parameters to iterate-over-subdir to take parameters
83ac0639ae82f157b949b9cfa7883ca13ea46451: feat: Add iterate-over-subdir
wink@3900x 24-11-30T19:14:57.511Z:~/prgs/rust/myrepos/expr-rust-git2/iterate-over-subdir (main)
```
