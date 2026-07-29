# Synchronisation

## Running

```bash
bvpelt@uranus:~/Develop/rwebserver/futures_ping_pong$ RUST_LOG=debug cargo run
   Compiling futures_ping_pong v0.1.0 (/home/bvpelt/Develop/rwebserver/futures_ping_pong)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_ping_pong`
 2026-07-29T16:21:16.102Z INFO  futures_ping_pong > Launching futures-example version: 0.1
 2026-07-29T16:21:16.102Z DEBUG futures_ping_pong > Start receiver
 2026-07-29T16:21:18.104Z DEBUG futures_ping_pong > Sender ready, signal to receiver to close the channel
 2026-07-29T16:21:18.104Z INFO  futures_ping_pong > "pong"
 2026-07-29T16:21:18.104Z DEBUG futures_ping_pong > Stop  receiver
 2026-07-29T16:21:18.104Z INFO  futures_ping_pong > Ready
bvpelt@uranus:~/Develop/rwebserver/futures_ping_pong$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_ping_pong`
 2026-07-29T16:21:25.819Z INFO  futures_ping_pong > Launching futures-example version: 0.1
 2026-07-29T16:21:25.819Z DEBUG futures_ping_pong > Start receiver
 2026-07-29T16:21:27.821Z DEBUG futures_ping_pong > Sender ready, signal to receiver to close the channel
 2026-07-29T16:21:27.821Z INFO  futures_ping_pong > "ping"
 2026-07-29T16:21:27.821Z DEBUG futures_ping_pong > Stop  receiver
 2026-07-29T16:21:27.821Z INFO  futures_ping_pong > Ready
bvpelt@uranus:~/Develop/rwebserver/futures_ping_pong$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_ping_pong`
 2026-07-29T16:21:29.645Z INFO  futures_ping_pong > Launching futures-example version: 0.1
 2026-07-29T16:21:29.645Z DEBUG futures_ping_pong > Start receiver
 2026-07-29T16:21:33.647Z DEBUG futures_ping_pong > Sender ready, signal to receiver to close the channel
 2026-07-29T16:21:33.647Z INFO  futures_ping_pong > "ping"
 2026-07-29T16:21:33.647Z DEBUG futures_ping_pong > Stop  receiver
 2026-07-29T16:21:33.647Z INFO  futures_ping_pong > Ready
bvpelt@uranus:~/Develop/rwebserver/futures_ping_pong$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_ping_pong`
 2026-07-29T16:21:34.646Z INFO  futures_ping_pong > Launching futures-example version: 0.1
 2026-07-29T16:21:34.647Z DEBUG futures_ping_pong > Start receiver
 2026-07-29T16:21:37.648Z DEBUG futures_ping_pong > Sender ready, signal to receiver to close the channel
 2026-07-29T16:21:37.648Z INFO  futures_ping_pong > "pong"
 2026-07-29T16:21:37.648Z DEBUG futures_ping_pong > Stop  receiver
 2026-07-29T16:21:37.648Z INFO  futures_ping_pong > Ready
bvpelt@uranus:~/Develop/rwebserver/futures_ping_pong$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_ping_pong`
 2026-07-29T16:21:38.669Z INFO  futures_ping_pong > Launching futures-example version: 0.1
 2026-07-29T16:21:38.669Z DEBUG futures_ping_pong > Start receiver
 2026-07-29T16:21:41.670Z DEBUG futures_ping_pong > Sender ready, signal to receiver to close the channel
 2026-07-29T16:21:41.670Z INFO  futures_ping_pong > "ping"
 2026-07-29T16:21:41.670Z DEBUG futures_ping_pong > Stop  receiver
 2026-07-29T16:21:41.671Z INFO  futures_ping_pong > Ready
bvpelt@uranus:~/Develop/rwebserver/futures_ping_pong$ 
```