# Remote Execution Service implemented in Rust

## Project Configuration
1. Use `cargo new` or `cargo init` command to initialize cargo project
2. Use `cargo install cargo-raze` to download raze that can generate BUILD files for bazel
3. If you plan to use any crates as bazel targets add this snippet:
```
load("//cargo:crates.bzl", "raze_fetch_remote_crates")

# Note that this method's name depends on your gen_workspace_prefix setting.
# `raze` is the default prefix.
raze_fetch_remote_crates()
```
4. Use `cargo raze` to generate bazel BUILD file for cargo crates.
** More about cargo raze can be found [here](https://github.com/google/cargo-raze) **
5. Now you can build it using bazel `bazel build //...`
