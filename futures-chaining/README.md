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
RUST_LOG=info cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures-chaining`
 2026-07-24T18:38:21.822Z INFO  futures_chaining > Launching futures-example version: 0.1
 2026-07-24T18:38:23.824Z INFO  futures_chaining > Player "player_one" took 2.001615202s
 2026-07-24T18:38:23.824Z INFO  futures_chaining > Player "player_two" took 1.001324929s
 2026-07-24T18:38:23.824Z INFO  futures_chaining > player_two won (took: 1.001324929s vs 2.001615202s)
 ```