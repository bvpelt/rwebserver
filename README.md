# rwebserver
Tutorial netwerk programming

Using https://www.youtube.com/playlist?list=PLTgRMOcmRb3M2fbwAgclKI0yt4uVchwZH

- [Introduction Rust](https://www.youtube.com/watch?v=bnK-b9auvZ4&list=PLTgRMOcmRb3M2fbwAgclKI0yt4uVchwZH&index=2)
- [TCP and UDP using Rust](https://www.youtube.com/watch?v=RJS6wMMwiA8&list=PLTgRMOcmRb3M2fbwAgclKI0yt4uVchwZH&index=3)
- [Data serialisation, deserialisation and parsing](https://www.youtube.com/watch?v=2s9YDff5KNo&list=PLTgRMOcmRb3M2fbwAgclKI0yt4uVchwZH&index=4)
- [Application level protocols](https://www.youtube.com/watch?v=148TzBtLXSc&list=PLTgRMOcmRb3M2fbwAgclKI0yt4uVchwZH&index=5)


## Serde
Serde is the de-facto standard way of (de)serialisation of data in rust


https://youtu.be/2s9YDff5KNo?si=F7RQ7DP7ZlZqo-zR&t=383

## Application level protocols
There are several protocols
- RPC

### RPC
Remote Procedure Calls

There are a number of implementation, a popular one is gRPC introduced by Google and later moved to opensource.
It offers high performance RPC over internet scale networks.
And is used in a number of project such as Kubernetes.

RPC can use Protocol Buffers.

*Protocol Buffers*
- a set of mechanisms to build language and platform neutral
- Exchange structured data between applications
- To describe the structured data  - define an Interface Definition Language (IDL)
- Compiler generated code
