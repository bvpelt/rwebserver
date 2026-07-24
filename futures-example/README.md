# Futures

- The Futures crates is the backbone for rust's asynchronous programming
- A Future is a placeholder for the result of an operation
- The result of an operation can be
    - unavailable cause the proces is still running
    - available since the proces is finished
        - Ok the result is available
        - Error case something went wrong


## Running

```bash
RUST_LOG=info cargo run
warning: /home/bvpelt/Develop/rwebserver/serde-basic/Cargo.toml: version requirement `0.9.34+deprecated` for dependency `serde_yaml` includes semver metadata which will be ignored, removing the metadata is recommended to avoid confusion
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures-example`
 2026-07-18T17:18:07.604Z INFO  futures_example > Launching futures-example version: 0.1
Right before first call
Called check_prime_boxed
Called check_prime_impl_trait
Results are true and true
Called check_prime in another thread
Result from the last call: true
```