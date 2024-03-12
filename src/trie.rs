use crate::utils;
use std::default;

#[derive(Debug, Clone, default::Default)]
pub struct Node {
    inner: String,
    count: usize,
    child: Vec<Self>,
}

impl Node {
    /// default initalization
    pub fn new() -> Self {
        Self::default()
    }
    /// named initialization (with count = 1)
    fn new_named(name: &str) -> Self {
        Self {
            inner: name.to_string(),
            count: 1,
            ..Default::default()
        }
    }
    fn _most_common_prefix(&self, prefix: &str, best_score: usize, best_string: &str) -> (usize, String) {
        let mut best_string = best_string.to_string();
        let mut best_score = best_score;
        let this_len = self.inner.len() + prefix.len();
        let this_str = prefix.to_string() + &self.inner;
        // L = length of string
        // N = #occurence
        // to calculate score, we use LN(N-2) and not just LN because LN is sensitive to long entries
        // with low frequency
        let this_score = this_len * self.count * (self.count.saturating_sub(2));
        if this_score > best_score {
            best_score = this_score;
            best_string = this_str.clone();
        }
        for c in &self.child {
            let (bscore, bstr) = c._most_common_prefix(&this_str, best_score, &best_string);
            if bscore > best_score {
                best_score = bscore;
                best_string = bstr.clone();
            }
        }
        (best_score, best_string.to_string())
    }
    /// find the prefix which maximizes:
    /// LN(N-2)
    /// where,
    /// L: length of prefix 
    /// N: number of occurence of prefix
    pub fn most_common_prefix(&self) -> (String, usize) {
        let (score, string) = self._most_common_prefix("", 0, "");
        (string, score)
    }
    fn find_placement(
        &mut self,
        child_name: &str,
    ) -> Option<(&mut Self, String, (String, String))> {
        self.child.iter_mut().find_map(|x| {
            let (pre, (a, b)) = utils::fork(&x.inner, child_name);
            if pre.is_empty() {
                return None;
            }
            Some((x, pre, (a, b)))
        })
    }
    pub fn insert(&mut self, entry: &str) {
        if entry.is_empty() {
            return;
        }
        self.count += 1;
        if let Some((node, pre, (a, b))) = self.find_placement(entry) {
            if !a.is_empty() {
                node.inner = a;
                unsafe {
                    map_in_place(node, |node| Self {
                        inner: pre,
                        count: node.count + 1,
                        child: vec![node, Self::new_named(&b)],
                    });
                }
            } else if b.is_empty() {
                node.count += 1
            } else {
                node.insert(&b)
            }
            return;
        }
        self.child.push(Self::new_named(entry));
    }
}

// https://stackoverflow.com/questions/67461269/replace-a-value-behind-a-mutable-reference-by-moving-and-mapping-the-original
unsafe fn map_in_place<T>(place: &mut T, f: impl FnOnce(T) -> T) {
    let place = place as *mut T;
    unsafe {
        let val = std::ptr::read(place);
        let new_val = f(val);
        std::ptr::write(place, new_val);
    }
}
