use std::iter;

/// returns mutual prefix and non-mutual trailing characters
///
/// ```
/// let s1 = "AAB";
/// let s2 = "ABC";
/// let (prefix, (left, right)) = fork(s1, s2);
/// assert_eq!(prefix, "A");
/// assert_eq!(left, "AB");
/// assert_eq!(right, "BC");
/// ```
pub fn fork(s1: &str, s2: &str) -> (String, (String, String)) {
    let mut prefix = String::new();
    let i = iter::zip(s1.chars(), s2.chars()).into_iter();
    let mut n = 0;
    let mut s1_trail = String::new();
    let mut s2_trail = String::new();
    let mut trailing = false;
    for (a, b) in i {
        if a != b {
            trailing = true
        }
        if trailing {
            s1_trail.push(a);
            s2_trail.push(b);
            continue;
        };
        n += 1;
        prefix.push(a);
    }
    (prefix, (s1[n..].to_string(), s2[n..].to_string()))
}

fn _simple_hash(n: usize, i: u32) -> usize {
    if i <= 0 {
        return n;
    }
    let hash: usize = 1315423911;
    _simple_hash(hash ^ ((hash << 5) + n + (hash >> 2)), i - 1)
}

pub fn simple_hash(n: usize) -> usize {
    let hash: usize = 1315424911;
    _simple_hash(
        hash ^ ((hash << 5) + n.wrapping_mul(3214124) + (hash >> 2)),
        12,
    ) % 1000
}
