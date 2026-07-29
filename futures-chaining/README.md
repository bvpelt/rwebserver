# Futures

Chaingin

Design
- run two task which have a different execution time. (player_one and player_two)
- run a seperate timeout thread to stop blocking
- determine what happened:
    - player_one won
    - player_two won
    - a timeout occured

## Running

```bash
bvpelt@uranus:~/Develop/rwebserver/futures-chaining$ RUST_LOG=info cargo run
   Compiling futures-chaining v0.1.0 (/home/bvpelt/Develop/rwebserver/futures-chaining)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures-chaining`
 2026-07-29T13:58:13.252Z INFO  futures_chaining > Launching futures-example version: 0.1
 2026-07-29T13:58:14.254Z INFO  futures_chaining > player_one won! (took: 1.001473714s)
bvpelt@uranus:~/Develop/rwebserver/futures-chaining$ RUST_LOG=info cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures-chaining`
 2026-07-29T13:58:15.141Z INFO  futures_chaining > Launching futures-example version: 0.1
 2026-07-29T13:58:16.143Z INFO  futures_chaining > player_one won! (took: 1.001587544s)
bvpelt@uranus:~/Develop/rwebserver/futures-chaining$ RUST_LOG=info cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures-chaining`
 2026-07-29T13:58:17.177Z INFO  futures_chaining > Launching futures-example version: 0.1
 2026-07-29T13:58:19.178Z INFO  futures_chaining > player_one won! (took: 2.001351993s)
bvpelt@uranus:~/Develop/rwebserver/futures-chaining$ RUST_LOG=info cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures-chaining`
 2026-07-29T13:58:20.170Z INFO  futures_chaining > Launching futures-example version: 0.1
 2026-07-29T13:58:21.171Z INFO  futures_chaining > player_two won! (took: 1.000375444s)

 ```