use rand::Rng;

pub fn is_sorted(data: &[i64]) -> bool {
    for idx in 1..data.len() {
        if data[idx] < data[idx - 1] {
            return false;
        }
    }
    return true;
}

pub fn random_vec(capacity: usize) -> Vec<i64> {
    let mut vec = vec![0; capacity];
    rand::thread_rng().fill(&mut vec[..]);
    vec
}
