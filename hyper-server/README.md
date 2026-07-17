# Hyper

Hyper is a:
- well-known rust-based HTTP framework
- with two distinct components:
    - one for writing servers
    - one for writing clients

## Building

For production
```bash
cargo build --release
```

After building running
```bash
$ RUST_LOG=debug target/release/hyper-server
Server running on http://127.0.0.1:8080
 2026-07-17T17:59:32.210Z INFO  hyper_server > Server running on http://127.0.0.1:8080
 2026-07-17T17:59:39.247Z WARN  hyper_server > path /aap not found
 2026-07-17T17:59:42.251Z WARN  hyper_server > path /080/app not found
 2026-07-17T17:59:59.047Z INFO  hyper_server > server working and responding
```

## Benchmark

Install the [apache benchmark (ab) tool](https://httpd.apache.org/docs/2.4/programs/ab.html)
```bash
$ sudo apt install apache2-utils
$ ab
ab: wrong number of arguments
Usage: ab [options] [http[s]://]hostname[:port]/path
Options are:
    -n requests     Number of requests to perform
    -c concurrency  Number of multiple requests to make at a time
    -t timelimit    Seconds to max. to spend on benchmarking
                    This implies -n 50000
    -s timeout      Seconds to max. wait for each response
                    Default is 30 seconds
    -b windowsize   Size of TCP send/receive buffer, in bytes
    -B address      Address to bind to when making outgoing connections
    -p postfile     File containing data to POST. Remember also to set -T
    -u putfile      File containing data to PUT. Remember also to set -T
    -T content-type Content-type header to use for POST/PUT data, eg.
                    'application/x-www-form-urlencoded'
                    Default is 'text/plain'
    -v verbosity    How much troubleshooting info to print
    -w              Print out results in HTML tables
    -i              Use HEAD instead of GET
    -x attributes   String to insert as table attributes
    -y attributes   String to insert as tr attributes
    -z attributes   String to insert as td or th attributes
    -C attribute    Add cookie, eg. 'Apache=1234'. (repeatable)
    -H attribute    Add Arbitrary header line, eg. 'Accept-Encoding: gzip'
                    Inserted after all normal header lines. (repeatable)
    -A attribute    Add Basic WWW Authentication, the attributes
                    are a colon separated username and password.
    -P attribute    Add Basic Proxy Authentication, the attributes
                    are a colon separated username and password.
    -X proxy:port   Proxyserver and port number to use
    -V              Print version number and exit
    -k              Use HTTP KeepAlive feature
    -d              Do not show percentiles served table.
    -S              Do not show confidence estimators and warnings.
    -q              Do not show progress when doing more than 150 requests
    -l              Accept variable document length (use this for dynamic pages)
    -g filename     Output collected data to gnuplot format file.
    -e filename     Output CSV file with percentages served
    -r              Don't exit on socket receive errors.
    -m method       Method name
    -h              Display usage information (this message)
    -I              Disable TLS Server Name Indication (SNI) extension
    -Z ciphersuite  Specify SSL/TLS cipher suite (See openssl ciphers)
    -f protocol     Specify SSL/TLS protocol
                    (SSL2, TLS1, TLS1.1, TLS1.2, TLS1.3 or ALL)
    -E certfile     Specify optional client certificate chain and private key

```

Workflow
- start server
- start benchmark

### start server

```bash
$ RUST_LOG=debug target/release/hyper-server
```

### start benchmark

```bash
# 
# test with 100 concurrent clients and a total of 1000 request
$ ab -n 1000 -c 100 http://localhost:8080/data
This is ApacheBench, Version 2.3 <$Revision: 1903618 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 100 requests
Completed 200 requests
Completed 300 requests
Completed 400 requests
Completed 500 requests
Completed 600 requests
Completed 700 requests
Completed 800 requests
Completed 900 requests
Completed 1000 requests
Finished 1000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /data
Document Length:        4 bytes

Concurrency Level:      100
Time taken for tests:   1.144 seconds
Complete requests:      1000
Failed requests:        0
Total transferred:      79000 bytes
HTML transferred:       4000 bytes
Requests per second:    874.38 [#/sec] (mean)
Time per request:       114.367 [ms] (mean)
Time per request:       1.144 [ms] (mean, across all concurrent requests)
Transfer rate:          67.46 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    1   0.8      1       4
Processing:   100  102   1.3    102     106
Waiting:      100  102   0.9    102     105
Total:        101  103   1.8    103     110

Percentage of the requests served within a certain time (ms)
  50%    103
  66%    104
  75%    105
  80%    105
  90%    106
  95%    107
  98%    108
  99%    108
 100%    110 (longest request)
```

After using the hyper-server-fase

```bash
$ RUST_LOG=debug target/release/hyper-server
Master Process: Spawning server across 14 cores...
 2026-07-17T18:35:47.148Z INFO  hyper_server > Master Process: Spawning server across 14 cores...
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #3 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #8 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #1 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #2 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #5 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #7 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #0 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #4 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #9 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #11 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #6 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #10 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #12 is online and listening
 2026-07-17T18:35:47.149Z INFO  hyper_server > Worker thread for Core #13 is online and listening
 2026-07-17T18:36:02.044Z INFO  hyper_server > [Thread ThreadId(8)] processed /data successfully
...
 2026-07-17T18:36:03.072Z INFO  hyper_server > [Thread ThreadId(4)] processed /data successfully
 2026-07-17T18:36:03.072Z INFO  hyper_server > [Thread ThreadId(13)] processed /data successfully
 2026-07-17T18:36:03.072Z INFO  hyper_server > [Thread ThreadId(13)] processed /data successfully
 2026-07-17T18:36:03.072Z INFO  hyper_server > [Thread ThreadId(6)] processed /data successfully
 2026-07-17T18:36:03.072Z INFO  hyper_server > [Thread ThreadId(7)] processed /data successfully
 2026-07-17T18:36:03.072Z INFO  hyper_server > [Thread ThreadId(8)] processed /data successfully
 2026-07-17T18:36:03.072Z INFO  hyper_server > [Thread ThreadId(2)] processed /data successfully

$ ab -n 1000 -c 100 http://localhost:8080/data
This is ApacheBench, Version 2.3 <$Revision: 1903618 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 100 requests
Completed 200 requests
Completed 300 requests
Completed 400 requests
Completed 500 requests
Completed 600 requests
Completed 700 requests
Completed 800 requests
Completed 900 requests
Completed 1000 requests
Finished 1000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /data
Document Length:        4 bytes

Concurrency Level:      100
Time taken for tests:   1.130 seconds
Complete requests:      1000
Failed requests:        0
Total transferred:      79000 bytes
HTML transferred:       4000 bytes
Requests per second:    884.61 [#/sec] (mean)
Time per request:       113.044 [ms] (mean)
Time per request:       1.130 [ms] (mean, across all concurrent requests)
Transfer rate:          68.25 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    1   0.8      0       4
Processing:   100  102   0.9    101     106
Waiting:      100  101   0.6    101     104
Total:        100  102   1.4    102     108
WARNING: The median and mean for the initial connection time are not within a normal deviation
        These results are probably not that reliable.
WARNING: The median and mean for the processing time are not within a normal deviation
        These results are probably not that reliable.

Percentage of the requests served within a certain time (ms)
  50%    102
  66%    102
  75%    103
  80%    103
  90%    104
  95%    105
  98%    106
  99%    106
 100%    108 (longest request)
```