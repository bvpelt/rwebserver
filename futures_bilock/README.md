# Locking

Future also includes a locking mechanism
- futures::sync::BiLock
- Future-aware mutex
- Arbitrates sharing a resource between two owners
- BiLock is only for two futures

```bash
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
   Compiling futures_bilock v0.1.0 (/home/bvpelt/Develop/rwebserver/futures_bilock)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:57:16.875Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:57:16.876Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:57:16.876Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:57:16.876Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:57:16.876Z INFO  futures_bilock > "pong"
 2026-07-29T16:57:16.876Z DEBUG futures_bilock > Stop  receiver
 2026-07-29T16:57:16.876Z INFO  futures_bilock > Ready
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:57:18.073Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:57:18.073Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:57:18.073Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:57:18.073Z INFO  futures_bilock > "pong"
 2026-07-29T16:57:18.073Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:57:18.073Z DEBUG futures_bilock > Stop  receiver
 2026-07-29T16:57:18.073Z INFO  futures_bilock > Ready
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:57:19.009Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:57:19.009Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:57:19.009Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:57:19.009Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:57:19.009Z INFO  futures_bilock > "pong"
 2026-07-29T16:57:19.009Z DEBUG futures_bilock > Stop  receiver
 2026-07-29T16:57:19.009Z INFO  futures_bilock > Ready
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:57:19.838Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:57:19.838Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:57:19.838Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:57:19.838Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:57:19.838Z INFO  futures_bilock > "pong"
 2026-07-29T16:57:19.838Z DEBUG futures_bilock > Stop  receiver
 2026-07-29T16:57:19.838Z INFO  futures_bilock > Ready
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:57:20.635Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:57:20.635Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:57:20.635Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:57:20.635Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:57:20.635Z INFO  futures_bilock > "ping"
 2026-07-29T16:57:20.635Z DEBUG futures_bilock > Stop  receiver
 2026-07-29T16:57:20.635Z INFO  futures_bilock > Ready
 ```