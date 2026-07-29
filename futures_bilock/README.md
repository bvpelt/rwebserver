# Locking

Future also includes a locking mechanism
- futures::sync::BiLock
- Future-aware mutex
- Arbitrates sharing a resource between two owners
- BiLock is only for two futures

```bash
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:49:52.129Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:49:52.129Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:49:52.130Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:49:52.130Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:49:52.130Z INFO  futures_bilock > "pong"
 2026-07-29T16:49:52.130Z DEBUG futures_bilock > Stop  receiver
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:49:52.980Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:49:52.980Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:49:52.980Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:49:52.980Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:49:52.980Z INFO  futures_bilock > "pong"
 2026-07-29T16:49:52.981Z DEBUG futures_bilock > Stop  receiver
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:49:53.675Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:49:53.675Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:49:53.675Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:49:53.675Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:49:53.675Z INFO  futures_bilock > "ping"
 2026-07-29T16:49:53.675Z DEBUG futures_bilock > Stop  receiver
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:49:54.410Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:49:54.410Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:49:54.410Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:49:54.410Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:49:54.410Z INFO  futures_bilock > "pong"
 2026-07-29T16:49:54.410Z DEBUG futures_bilock > Stop  receiver
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:49:55.178Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:49:55.178Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:49:55.178Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:49:55.178Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:49:55.178Z INFO  futures_bilock > "ping"
 2026-07-29T16:49:55.178Z DEBUG futures_bilock > Stop  receiver
bvpelt@uranus:~/Develop/rwebserver/futures_bilock$ RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/bvpelt/Develop/rwebserver/target/debug/futures_bilock`
 2026-07-29T16:49:55.917Z INFO  futures_bilock > Launching futures-example version: 0.1
 2026-07-29T16:49:55.917Z DEBUG futures_bilock > Start receiver
 2026-07-29T16:49:55.917Z INFO  futures_bilock > Value of lock: 1
 2026-07-29T16:49:55.917Z DEBUG futures_bilock > Sender ready, signal to receiver to close the channel
 2026-07-29T16:49:55.917Z INFO  futures_bilock > "pong"
 2026-07-29T16:49:55.918Z DEBUG futures_bilock > Stop  receiver
 ```