use std::thread;

pub mod helpers;

pub fn merge_sort(nums: &mut [i64], left: usize, right: usize) {
    if left < right {
        let middle = left + (right - left) / 2;
        merge_sort(nums, left, middle);
        merge_sort(nums, middle + 1, right);
        merge(nums, left, middle, right);
    }
}

pub fn parallel_mergesort(data: &mut [i64], threads: usize) {
    let chunks = std::cmp::min(data.len(), threads);
    let mut chunk_lens = Vec::new();
    let data_len = data.len();
    thread::scope(|s| {
        for slice in data.chunks_mut(data_len / chunks) {
            let slice_len = slice.len();
            chunk_lens.push(slice_len);
            s.spawn(move || merge_sort(slice, 0, slice_len - 1));
        }
    });

    let mut middle: usize = 0;
    let mut end: usize = chunk_lens[0] - 1;
    for cl in chunk_lens {
        merge(data, 0, middle, end);
        middle = end;
        end = end + cl;
    }
}

fn merge(nums: &mut [i64], left: usize, middle: usize, right: usize) {
    let n1 = middle - left + 1;
    let n2 = right - middle;
    let mut left_nums: Vec<i64> = vec![0; n1];
    let mut right_nums: Vec<i64> = vec![0; n2];

    // copy data into left and right subarrays
    for i in 0..n1 {
        left_nums[i] = nums[left + i];
    }
    for j in 0..n2 {
        right_nums[j] = nums[middle + 1 + j];
    }

    let mut i = 0usize;
    let mut j = 0usize;
    let mut k = left;
    while i < n1 && j < n2 {
        if left_nums[i] <= right_nums[j] {
            nums[k] = left_nums[i];
            i += 1;
        } else {
            nums[k] = right_nums[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        nums[k] = left_nums[i];
        i += 1;
        k += 1;
    }

    while j < n2 {
        nums[k] = right_nums[j];
        j += 1;
        k += 1;
    }
}

pub fn insertion_sort(nums: &mut Vec<i64>) {
    // insertion sort is to iterate through nums and keep
    // placing less than elements before the current marker
    // e.g.
    //
    //  i
    // [5, 3, 20, 1, 4] ->   j
    //                      [3, 5, 20, 1, 4]
    //                      [1, 3, 5, 20, 4]
    for i in 1..nums.len() {
        let mut j = i;
        while j > 0 && nums[j - 1] > nums[j] {
            let tmp = nums[j];
            nums[j] = nums[j - 1];
            nums[j - 1] = tmp;
            j -= 1;
        }
    }
}

pub fn parallel_sort(data: &mut [i64], threads: usize) {
    let chunks = std::cmp::min(data.len(), threads);
    let _ = thread::scope(|scope| {
        for slice in data.chunks_mut(data.len() / chunks) {
            scope.spawn(move || serial_sort(slice));
        }
    });
    merge_placeholder(data, chunks);
}

pub fn serial_sort(data: &mut [i64]) {
    // actual implementation omitted for conciseness
    data.sort()
}

fn merge_placeholder(data: &mut [i64], _sorted_chunks: usize) {
    // actual implementation omitted for conciseness
    data.sort()
}
