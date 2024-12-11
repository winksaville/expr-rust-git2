# git-commits

Experiment with getting commits while using revwalk.

This is a simple program that takes a git repository path and two commits
and then prints the commits between the two commits. A very important thing
is happening is that commit: `78d0d64a2..` below is a merge commit where
parent[1]], `bc776c430..` ends with a root commit i.e. not parents because
I imported with the `git merge xxx --allow-unrelated-histories`.

```
commit id: 78d0d64a2f08455772c47e1fb5df80d7bf2f1958, summary: 'Add merge-cmds', ...
```

See ChatGPT40: https://chatgpt.com/share/6754e310-1ae4-800c-85f0-f918f30ff873 

# Run

Hereis an example of running the program `get-commits`:

```sh
wink@3900x 24-12-11T01:40:12.976Z:~/prgs/rust/myrepos/expr-rust-git2/get-commits (get-commits)
$ RUST_LOG=info cargo run .. 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9 985279818e22f81bbcd6860f74df34fca3f04c49
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `/home/wink/prgs/rust/myrepos/expr-rust-git2/target/debug/get-commits .. 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9 985279818e22f81bbcd6860f74df34fca3f04c49`
[2024-12-11T01:40:21.495893896Z INFO  get_commits   63  1] main:+
[2024-12-11T01:40:21.495918091Z INFO  get_commits   81  1] arg: 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9
[2024-12-11T01:40:21.495923862Z INFO  get_commits   81  1] arg: 985279818e22f81bbcd6860f74df34fca3f04c49
[2024-12-11T01:40:21.495929222Z INFO  get_commits    6  1] get_commits_between:+ repo_path: .., oid_strings: ["8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9", "985279818e22f81bbcd6860f74df34fca3f04c49"]
[2024-12-11T01:40:21.503311554Z INFO  get_commits    8  1] get_commits_between: Repository::open result: is_ok=true
commit id: 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9, summary: 'chore: Cleanup clippy warnings', parent.len: 1, parents: [Commit { id: 7c302d4b9ea64bd6c14da5b5c56bfe1396967fee, summary: "feat: merge-cmds running" }]
commit id: 7c302d4b9ea64bd6c14da5b5c56bfe1396967fee, summary: 'feat: merge-cmds running', parent.len: 1, parents: [Commit { id: 78d0d64a2f08455772c47e1fb5df80d7bf2f1958, summary: "Add merge-cmds" }]
commit id: 78d0d64a2f08455772c47e1fb5df80d7bf2f1958, summary: 'Add merge-cmds', parent.len: 2, parents: [Commit { id: bc776c430404342d2754bd880406b7af56abb618, summary: "feat: Share dependencies" }, Commit { id: 6fabd32f84253674cd6c5f122ed5998201018062, summary: "restructure-into-merge-cmds" }]
commit id: 6fabd32f84253674cd6c5f122ed5998201018062, summary: 'restructure-into-merge-cmds', parent.len: 1, parents: [Commit { id: 0c26ea66d172b7f6f4e22b4bd12bbf2a401eb18c, summary: "feat: Tag as v0.1.0" }]
commit id: 0c26ea66d172b7f6f4e22b4bd12bbf2a401eb18c, summary: 'feat: Tag as v0.1.0', parent.len: 1, parents: [Commit { id: cfeec0b233856b86e547a213a09206be0283c86c, summary: "feat: Add a main.rs that uses various merge_basexxx fn of git2" }]
commit id: cfeec0b233856b86e547a213a09206be0283c86c, summary: 'feat: Add a main.rs that uses various merge_basexxx fn of git2', parent.len: 1, parents: [Commit { id: a666969b6d98f74eb66cbc99f220ec57ca366044, summary: "feat: Initial Commit" }]
commit id: a666969b6d98f74eb66cbc99f220ec57ca366044, summary: 'feat: Initial Commit', parent.len: 0, parents: []
commit id: bc776c430404342d2754bd880406b7af56abb618, summary: 'feat: Share dependencies', parent.len: 1, parents: [Commit { id: 971896646014c778239028407c0848ea631e0b65, summary: "Merge pull request #2 from winksaville/explr-merge-base" }]
commit id: 971896646014c778239028407c0848ea631e0b65, summary: 'Merge pull request #2 from winksaville/explr-merge-base', parent.len: 2, parents: [Commit { id: 985279818e22f81bbcd6860f74df34fca3f04c49, summary: "docs: Updated/Created README.md files" }, Commit { id: ed61a78b4fd91b6b767a3d92261b1f273e58988f, summary: "feat: Add explr-merge-base" }]
commit id: ed61a78b4fd91b6b767a3d92261b1f273e58988f, summary: 'feat: Add explr-merge-base', parent.len: 1, parents: [Commit { id: 985279818e22f81bbcd6860f74df34fca3f04c49, summary: "docs: Updated/Created README.md files" }]
[2024-12-11T01:40:21.503862899Z INFO  get_commits   88  1] main:-
wink@3900x 24-12-11T01:40:21.507Z:~/prgs/rust/myrepos/expr-rust-git2/get-commits (get-commits)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
