# git-commits

Experiment with getting commits while using revwalk.

This is a simple program that takes a git repository path and one or two commits.
If only one commit is give it then prints all commits from that point to the
initial commit in the repo. If there are two commits the second commit and
afterwards are not listed.

Note it is "properly" handling "unrelated-histories" as is present in the
merge commit `78d0d64a2.. Add merge-cmds` as the parent[1] of that merge
was merged using `git merge xxx --allow-unrelated-histories` and ends
with a "root" commit (commits with no parents) at
`a666969b6.. feat: Initial Commit`.

See ChatGPT40: https://chatgpt.com/share/6754e310-1ae4-800c-85f0-f918f30ff873 

I have a couple other things to investigate:
* Get the list of the file paths that were changed in a commit
  and checking if they are in a specific sub-directory.
* Better understand `git2::Revwalk::hide`. The source code imples a
  single commit is hidden but actually it hides all subsequent parents,
  which is what I want, but that is not what the doc says to me.
  ```
      /// Mark a commit as not of interest to this revwalk.
      pub fn hide(&mut self, oid: Oid) -> Result<(), Error> {
          unsafe {
              try_call!(raw::git_revwalk_hide(self.raw(), oid.raw()));
          }
          Ok(())
      }
  ```



# Run

Hereis an example of running the program `get-commits` with two commit ids:

```sh
wink@3900x 24-12-11T19:21:49.345Z:~/prgs/rust/myrepos/expr-rust-git2/get-commits (get-commits)
$ RUST_LOG=info cargo run .. 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9 985279818e22f81bbcd6860f74df34fca3f04c49
   Compiling get-commits v0.1.0 (/home/wink/prgs/rust/myrepos/expr-rust-git2/get-commits)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.26s
     Running `/home/wink/prgs/rust/myrepos/expr-rust-git2/target/debug/get-commits .. 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9 985279818e22f81bbcd6860f74df34fca3f04c49`
[2024-12-11T19:22:02.545984204Z INFO  get_commits   62  1] main:+
[2024-12-11T19:22:02.546014851Z INFO  get_commits   80  1] arg: 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9
[2024-12-11T19:22:02.546021113Z INFO  get_commits   80  1] arg: 985279818e22f81bbcd6860f74df34fca3f04c49
[2024-12-11T19:22:02.546027084Z INFO  get_commits    6  1] get_commits_between:+ repo_path: .., oid_strings: ["8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9", "985279818e22f81bbcd6860f74df34fca3f04c49"]
[2024-12-11T19:22:02.554727146Z INFO  get_commits    8  1] get_commits_between: Repository::open result: is_ok=true
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
[2024-12-11T19:22:02.555284721Z INFO  get_commits   48  1] get_commits_between:- repo_path: .., oid_strings: ["8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9", "985279818e22f81bbcd6860f74df34fca3f04c49"]
[2024-12-11T19:22:02.555315649Z INFO  get_commits   87  1] main:-
wink@3900x 24-12-11T19:22:02.559Z:~/prgs/rust/myrepos/expr-rust-git2/get-commits (get-commits)
```

And here with one commit id:

```sh
wink@3900x 24-12-11T19:22:23.936Z:~/prgs/rust/myrepos/expr-rust-git2/get-commits (get-commits)
$ RUST_LOG=info cargo run .. 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `/home/wink/prgs/rust/myrepos/expr-rust-git2/target/debug/get-commits .. 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9`
[2024-12-11T19:23:47.559807839Z INFO  get_commits   62  1] main:+
[2024-12-11T19:23:47.559831033Z INFO  get_commits   80  1] arg: 8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9
[2024-12-11T19:23:47.559837635Z INFO  get_commits    6  1] get_commits_between:+ repo_path: .., oid_strings: ["8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9"]
[2024-12-11T19:23:47.567227099Z INFO  get_commits    8  1] get_commits_between: Repository::open result: is_ok=true
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
commit id: 985279818e22f81bbcd6860f74df34fca3f04c49, summary: 'docs: Updated/Created README.md files', parent.len: 1, parents: [Commit { id: 1ac0ff8b841d5336575122a833f00d7d0798ba2b, summary: "Merge pull request #1 from winksaville/actually-iterate-over-commits-not-files" }]
commit id: 1ac0ff8b841d5336575122a833f00d7d0798ba2b, summary: 'Merge pull request #1 from winksaville/actually-iterate-over-commits-not-files', parent.len: 2, parents: [Commit { id: bc5d57ccaa2116fa447a1e86a58d160473817d6d, summary: "feat: Initial Commit" }, Commit { id: 86e22f70e8cbae19a41827450d308e1c98c882fb, summary: "feat: Support root or subdirectories" }]
commit id: 86e22f70e8cbae19a41827450d308e1c98c882fb, summary: 'feat: Support root or subdirectories', parent.len: 1, parents: [Commit { id: 949749479f6f28f2882ead0b315fb7c00ceb412c, summary: "feat: Iterate over commits" }]
commit id: 949749479f6f28f2882ead0b315fb7c00ceb412c, summary: 'feat: Iterate over commits', parent.len: 1, parents: [Commit { id: fb776b0b1d4a11aefbf0d4fa538a87c50dfa02e1, summary: "chore: Add more logging" }]
commit id: fb776b0b1d4a11aefbf0d4fa538a87c50dfa02e1, summary: 'chore: Add more logging', parent.len: 1, parents: [Commit { id: 063e78daf9b97ffea0a1eec02da348a1454b864a, summary: "feat: Add logging using my custom_logger" }]
commit id: 063e78daf9b97ffea0a1eec02da348a1454b864a, summary: 'feat: Add logging using my custom_logger', parent.len: 1, parents: [Commit { id: 8de6748c19d0a5d5b9f17bc29734a93dfe63cee7, summary: "feat: Add parameters to iterate-over-subdir to take parameters" }]
commit id: 8de6748c19d0a5d5b9f17bc29734a93dfe63cee7, summary: 'feat: Add parameters to iterate-over-subdir to take parameters', parent.len: 1, parents: [Commit { id: 83ac0639ae82f157b949b9cfa7883ca13ea46451, summary: "feat: Add iterate-over-subdir" }]
commit id: 83ac0639ae82f157b949b9cfa7883ca13ea46451, summary: 'feat: Add iterate-over-subdir', parent.len: 1, parents: [Commit { id: bc5d57ccaa2116fa447a1e86a58d160473817d6d, summary: "feat: Initial Commit" }]
commit id: bc5d57ccaa2116fa447a1e86a58d160473817d6d, summary: 'feat: Initial Commit', parent.len: 0, parents: []
[2024-12-11T19:23:47.567792640Z INFO  get_commits   48  1] get_commits_between:- repo_path: .., oid_strings: ["8a4c3ac5960b2b879d5e5aeeca4ca2a0aea493b9"]
[2024-12-11T19:23:47.567826865Z INFO  get_commits   87  1] main:-
wink@3900x 24-12-11T19:23:47.571Z:~/prgs/rust/myrepos/expr-rust-git2/get-commits (get-commits)
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
