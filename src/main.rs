use sorting;
use sorting::helpers;
use std::time::Instant;

fn main() {
    let num_test_elements: usize = 100;
    let test_vec = helpers::random_vec(num_test_elements);
    let mut nums: Vec<i64> = vec![5, 3, 20, 1, 4];
    sorting::insertion_sort(&mut nums);
    let vec_is_sorted = helpers::is_sorted(&nums);
    // expect that returned value is: [1, 3, 4, 5, 20]
    assert!(vec_is_sorted);

    let start = Instant::now();
    let mut vec = test_vec.clone();
    sorting::serial_sort(&mut vec);
    let elapsed = start.elapsed();
    let vec_is_sorted = helpers::is_sorted(&vec[..]);
    assert!(vec_is_sorted);
    println!(
        "serial sort results using built-in sort, is sorted: {:?}, time elapsed: {:?} milliseconds",
        vec_is_sorted,
        elapsed.as_millis()
    );

    let start = Instant::now();
    let mut vec = test_vec.clone();
    sorting::parallel_sort(&mut vec, 4);
    let elapsed = start.elapsed();
    let vec_is_sorted = helpers::is_sorted(&vec[..]);
    assert!(vec_is_sorted);
    println!(
        "parallel sort results using built-in sort, is sorted: {:?}, time elapsed: {:?} milliseconds",
        vec_is_sorted,
        elapsed.as_millis()
    );

    let start = Instant::now();
    let mut vec_two = test_vec.clone();
    let vec_two_sz = vec_two.len();
    sorting::merge_sort(&mut vec_two, 0, vec_two_sz - 1);
    let elapsed = start.elapsed();
    let vec_two_is_sorted = helpers::is_sorted(&vec_two[..]);
    assert!(vec_two_is_sorted);
    println!(
        "serial mergesort results, is sorted: {:?}, time elapsed: {:?} milliseconds",
        vec_two_is_sorted,
        elapsed.as_millis()
    );

    let start = Instant::now();
    let mut vec = test_vec.clone();
    let chunks = 10;
    sorting::parallel_mergesort(&mut vec, chunks);
    let elapsed = start.elapsed();
    let vec_is_sorted = helpers::is_sorted(&vec[..]);
    assert!(vec_is_sorted);
    println!(
        "parallel mergesort results, is sorted: {:?}, time elapsed: {:?} milliseconds",
        vec_is_sorted,
        elapsed.as_millis()
    );
}
