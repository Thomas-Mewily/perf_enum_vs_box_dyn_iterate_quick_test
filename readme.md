A quick a silly test to compare the performance of enum vs Box Dyn.

The test was run in release mode :
```bash
cargo run --release
```


| Sequential Version             | Time       |
|--------------------------------|------------|
| iter enum sum os               | 97.088 ms  |
| iter dyn box sum os            | 1079.2 ms  |


| Parallel Version               | Time       |
|--------------------------------|------------|
| iter enum sum os par           | 54.208 ms  |
| iter dyn box sum os par time   | 218.22 ms  |


```
init the animal list of 200000000 animals...
done !
Gnuplot not found, using plotters backend

Warning: It is not recommended to reduce nresamples below 1000.
Benchmarking iter enum sum os
Benchmarking iter enum sum os: Warming up for 3.0000 s
Benchmarking iter enum sum os: Collecting 10 samples in estimated 22.226 s (220 iterations)
Benchmarking iter enum sum os: Analyzing
iter enum sum os        time:   [95.158 ms 97.088 ms 98.386 ms]
                        change: [-1.7379% +0.1423% +1.8927%] (p = 0.90 > 0.05)
                        No change in performance detected.

Benchmarking iter dyn box sum os
Benchmarking iter dyn box sum os: Warming up for 3.0000 s
Benchmarking iter dyn box sum os: Collecting 10 samples in estimated 21.002 s (20 iterations)
Benchmarking iter dyn box sum os: Analyzing
iter dyn box sum os     time:   [1.0471 s 1.0792 s 1.1164 s]
                        change: [+0.3337% +3.5472% +5.8196%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking iter enum sum os par
Benchmarking iter enum sum os par: Warming up for 3.0000 s
Benchmarking iter enum sum os par: Collecting 10 samples in estimated 21.967 s (385 iterations)
Benchmarking iter enum sum os par: Analyzing
iter enum sum os par    time:   [52.012 ms 54.208 ms 56.039 ms]
                        change: [-3.5353% -0.0047% +2.9177%] (p = 0.96 > 0.05)
                        No change in performance detected.

Benchmarking iter dyn box sum os par
Benchmarking iter dyn box sum os par: Warming up for 3.0000 s
Benchmarking iter dyn box sum os par: Collecting 10 samples in estimated 24.575 s (110 iterations)
Benchmarking iter dyn box sum os par: Analyzing
iter dyn box sum os par time:   [215.79 ms 218.22 ms 221.26 ms]
                        change: [-4.6417% -1.1243% +2.2051%] (p = 0.48 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Done
```