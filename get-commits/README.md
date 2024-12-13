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

This looks pretty good but there is at least one problem,
merge commits are not handled properly if there is an associated
change. I beleive this happens when git detects there is a confict
and the user resolves it during the merge. This is what I think is
happening and in this converstion with
[ChatGPT40](https://chatgpt.com/share/675c90d2-65f4-800c-9652-69a22ea29f33)
it agreed with me.

The other signficcant thing I learned is that `git show <sha1>` shows
the changes for a single commit! For example; here I showing the output
for the merge commit `a9b91ea50655ace5067da8dec488681c9d53ebae` which does
resolve a conficit and the changes are shown. But if you look at the output
from my get-commits below you will see that the changes are **not** shown.

I'm going to investigate this further by implementing a `git show` command
using git2-rs in this workspace.
```
wink@3900x 24-12-13T19:30:46.674Z:~/prgs/rpi-pico/myrepos/rp-hal (wip-release-rp235x-hal-v0.3.0)
$ git --no-pager show a9b91ea50655ace5067da8dec488681c9d53ebae
commit a9b91ea50655ace5067da8dec488681c9d53ebae
Merge: 2987210 c0679ca
Author: Jan Niehusmann <jan@gondor.com>
Date:   Sat Feb 24 13:08:44 2024 +0100

    Merge pull request #776 from jannic/as_ptr
    
    Use as_ptr() to retrieve pointer to register

diff --cc rp2040-hal/src/adc.rs
index b5188cd,d12a748..b7d22fc
--- a/rp2040-hal/src/adc.rs
+++ b/rp2040-hal/src/adc.rs
@@@ -791,22 -694,8 +791,22 @@@ impl<'a, Word> AdcFifo<'a, Word> 
      /// The [`DmaReadTarget`] returned by this function can be used to initiate DMA transfers
      /// reading from the ADC.
      pub fn dma_read_target(&self) -> DmaReadTarget<Word> {
-         DmaReadTarget(self.adc.device.fifo() as *const _ as u32, PhantomData)
+         DmaReadTarget(self.adc.device.fifo().as_ptr() as u32, PhantomData)
      }
 +
 +    /// Trigger a single conversion
 +    ///
 +    /// Ignored unless in [`AdcFifoBuilder::manual_trigger`] mode.
 +    pub fn trigger(&mut self) {
 +        self.adc.device.cs().modify(|_, w| w.start_once().set_bit());
 +    }
 +
 +    /// Check if ADC is ready for the next conversion trigger
 +    ///
 +    /// Only useful in [`AdcFifoBuilder::manual_trigger`] mode.
 +    pub fn is_ready(&self) -> bool {
 +        self.adc.device.cs().read().ready().bit_is_set()
 +    }
  }
  
  impl<'a> AdcFifo<'a, u16> {
wink@3900x 24-12-13T20:09:40.205Z:~/prgs/rpi-pico/myrepos/rp-hal (wip-release-rp235x-hal-v0.3.0)
```

# Run

Here is an example of running the program `get-commits` with two commit ids, using one
increases the lines to the initial commit:

```sh
wink@3900x 24-12-13T19:58:42.212Z:~/prgs/rust/myrepos/expr-rust-git2/get-commits (gpt4o-take-on-handling-merge-commits)
$ RUST_LOG=info cargo run -- ~/prgs/rpi-pico/myrepos/rp-hal a9b91ea50655ace5067da8dec488681c9d53ebae c11fed9b0dc11f19dc19f89e0c0b79e09c2f65e3
   Compiling get-commits v0.1.0 (/home/wink/prgs/rust/myrepos/expr-rust-git2/get-commits)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.88s
     Running `/home/wink/prgs/rust/myrepos/expr-rust-git2/target/debug/get-commits /home/wink/prgs/rpi-pico/myrepos/rp-hal a9b91ea50655ace5067da8dec488681c9d53ebae c11fed9b0dc11f19dc19f89e0c0b79e09c2f65e3`
[2024-12-13T19:59:01.622551742Z INFO  get_commits  195  1] main:+
[2024-12-13T19:59:01.622584193Z INFO  get_commits  157  1] get_commits:+ repo_path: /home/wink/prgs/rpi-pico/myrepos/rp-hal, oid_strings: ["a9b91ea50655ace5067da8dec488681c9d53ebae", "c11fed9b0dc11f19dc19f89e0c0b79e09c2f65e3"]
Diff: merged_baseline_tree vs commit_tree
Diff: merged_baseline_tree vs commit_tree END
commit id: a9b91ea50655ace5067da8dec488681c9d53ebae, summary: 'Merge pull request #776 from jannic/as_ptr', parent.len: 2, modified_count: 0, non_matching_count: 0, parents: [29872106d82b0457dffaf688c89c78d8bc3a904d, c0679ca582ea355181db5daf5145d1dd1de5ac2b]
Diff: parent vs commit_tree
 diff --git a/rp2040-hal/src/adc.rs b/rp2040-hal/src/adc.rs
index 3894bcc..d12a748 100644
--- a/rp2040-hal/src/adc.rs
+++ b/rp2040-hal/src/adc.rs
 @@ -694,7 +694,7 @@ impl<'a, Word> AdcFifo<'a, Word> {
     /// The [`DmaReadTarget`] returned by this function can be used to initiate DMA transfers
     /// reading from the ADC.
     pub fn dma_read_target(&self) -> DmaReadTarget<Word> {
-        DmaReadTarget(self.adc.device.fifo() as *const _ as u32, PhantomData)
+        DmaReadTarget(self.adc.device.fifo().as_ptr() as u32, PhantomData)
     }
 }
 
 diff --git a/rp2040-hal/src/pio.rs b/rp2040-hal/src/pio.rs
index c38e7dd..b54840a 100644
--- a/rp2040-hal/src/pio.rs
+++ b/rp2040-hal/src/pio.rs
 @@ -1432,7 +1432,7 @@ unsafe impl<SM: ValidStateMachine> ReadTarget for Rx<SM> {
 
     fn rx_address_count(&self) -> (u32, u32) {
         (
-            unsafe { &*self.block }.rxf(SM::id()) as *const _ as u32,
+            unsafe { &*self.block }.rxf(SM::id()).as_ptr() as u32,
             u32::MAX,
         )
     }
 @@ -1626,7 +1626,7 @@ unsafe impl<SM: ValidStateMachine> WriteTarget for Tx<SM> {
 
     fn tx_address_count(&mut self) -> (u32, u32) {
         (
-            unsafe { &*self.block }.txf(SM::id()) as *const _ as u32,
+            unsafe { &*self.block }.txf(SM::id()).as_ptr() as u32,
             u32::MAX,
         )
     }
 diff --git a/rp2040-hal/src/spi.rs b/rp2040-hal/src/spi.rs
index 4da1abc..8ba70ad 100644
--- a/rp2040-hal/src/spi.rs
+++ b/rp2040-hal/src/spi.rs
 @@ -518,7 +518,7 @@ macro_rules! impl_write {
 
             fn rx_address_count(&self) -> (u32, u32) {
                 (
-                    self.device.sspdr() as *const _ as u32,
+                    self.device.sspdr().as_ptr() as u32,
                     u32::MAX,
                 )
             }
 @@ -541,7 +541,7 @@ macro_rules! impl_write {
 
             fn tx_address_count(&mut self) -> (u32, u32) {
                 (
-                    self.device.sspdr() as *const _ as u32,
+                    self.device.sspdr().as_ptr() as u32,
                     u32::MAX,
                 )
             }
 diff --git a/rp2040-hal/src/uart/reader.rs b/rp2040-hal/src/uart/reader.rs
index 0a86431..3a32121 100644
--- a/rp2040-hal/src/uart/reader.rs
+++ b/rp2040-hal/src/uart/reader.rs
 @@ -244,7 +244,7 @@ unsafe impl<D: UartDevice, P: ValidUartPinout<D>> ReadTarget for Reader<D, P> {
     }
 
     fn rx_address_count(&self) -> (u32, u32) {
-        (self.device.uartdr() as *const _ as u32, u32::MAX)
+        (self.device.uartdr().as_ptr() as u32, u32::MAX)
     }
 
     fn rx_increment(&self) -> bool {
         // below `FCS.THRESH`, which requires `FCS.THRESH` not to be 0.

...

-        while self.len() > 0 {
-            self.read_from_fifo();
-        }
+        self.clear();
         // disable fifo, reset threshold to 0 and disable DMA
         self.adc
             .device
 @@ -696,6 +777,20 @@ impl<'a, Word> AdcFifo<'a, Word> {
     pub fn dma_read_target(&self) -> DmaReadTarget<Word> {
         DmaReadTarget(self.adc.device.fifo() as *const _ as u32, PhantomData)
     }
+
+    /// Trigger a single conversion
+    ///
+    /// Ignored unless in [`AdcFifoBuilder::manual_trigger`] mode.
+    pub fn trigger(&mut self) {
+        self.adc.device.cs().modify(|_, w| w.start_once().set_bit());
+    }
+
+    /// Check if ADC is ready for the next conversion trigger
+    ///
+    /// Only useful in [`AdcFifoBuilder::manual_trigger`] mode.
+    pub fn is_ready(&self) -> bool {
+        self.adc.device.cs().read().ready().bit_is_set()
+    }
 }
 
 impl<'a> AdcFifo<'a, u16> {
Diff: parent vs commit_tree END
commit id: 1f341fd4936c4b41933ec669d97fe0a084fb1bc6, summary: 'Allow free-running ADC mode without FIFO', parent.len: 1, modified_count: 5, non_matching_count: 0, parents: [c11fed9b0dc11f19dc19f89e0c0b79e09c2f65e3]
  file: rp2040-hal/examples/adc.rs
  file: rp2040-hal/examples/adc_fifo_dma.rs
  file: rp2040-hal/examples/adc_fifo_irq.rs
  file: rp2040-hal/examples/adc_fifo_poll.rs
  file: rp2040-hal/src/adc.rs
[2024-12-13T19:59:01.641518575Z INFO  get_commits  181  1] get_commits:- repo_path: /home/wink/prgs/rpi-pico/myrepos/rp-hal, oid_strings: ["a9b91ea50655ace5067da8dec488681c9d53ebae", "c11fed9b0dc11f19dc19f89e0c0b79e09c2f65e3"]
[2024-12-13T19:59:01.641635595Z INFO  get_commits  210  1] main:-
wink@3900x 24-12-13T19:59:01.645Z:~/prgs/rust/myrepos/expr-rust-git2/get-commits (gpt4o-take-on-handling-merge-commits)
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
