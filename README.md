# rust_sorting

This is a collection of sorting code written in Rust, which highlights some differences between Rust 
and other languages especially when it comes to things like ownership.


## How to Run Code

`main.rs` includes some code that runs the simplest benchmark possible, just creates a vector of size x and then 
runs several of the sorting algoritms on it, printing out how long it took to sort and making some assertions.

You can reproduce the results by simply running `cargo run` in the terminal and should see something like the output below:

> serial sort results using built-in sort, is sorted: true, time elapsed: 3026 milliseconds
>
> parallel sort results using built-in sort, is sorted: true, time elapsed: 1091 milliseconds
>
> serial mergesort results, is sorted: true, time elapsed: 12348 milliseconds
>
> parallel mergesort results, is sorted: true, time elapsed: 5640 milliseconds

# How to Run Tests

`cargo test`
