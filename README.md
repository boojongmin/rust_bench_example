

### criterion
 - [crate](https://crates.io/crates/criterion)
 - [doc](https://bheisler.github.io/criterion.rs/book/index.html)

### 

Warmup
Every Criterion.rs benchmark iterates the benchmarked function automatically for a configurable warmup period (by default, for three seconds). For Rust function benchmarks, this is to warm up the processor caches and (if applicable) file system caches.

Collecting Samples
Criterion iterates the function to be benchmarked with a varying number of iterations to generate an estimate of the time taken by each iteration. The number of samples is configurable. It also prints an estimate of the time the sampling process will take based on the time per iteration during the warmup period.


### html sample report

[sample report](https://bheisler.github.io/criterion.rs/book/user_guide/html_report/report/index.html)



### command

```shell
$ cargo bench --bench fibonacci


Gnuplot not found, using plotters backend
fib 20                  time:   [10.595 µs 10.636 µs 10.681 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
  
$ .\target\criterion\report\index.html

```

[gnuplot](http://www.gnuplot.info/)



### compare

```shell

$ cargo bench --bench compare_fibonacci
$ .\target\criterion\report\index.html
```



```shell

$ cargo bench --bench stack_vs_heap_invalid
$ cargo bench --bench stack_vs_heap_blackbox
$ cargo bench --bench stack_vs_heap_custom_criterion 

$ .\target\criterion\report\index.html
```



