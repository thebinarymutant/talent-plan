use failure::Fail;
// use std::io;

/// Error type for kvs.
#[derive(Fail, Debug)]
pub enum KvsError {
    /// Removing non-existent key error.
    #[fail(display = "Key not found")]
    KeyNotFound,
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;w


use std::collections::HashMap;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut set: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        set.insert(target, vec![]);

        set.get(&target).unwrap()
    }

    fn dp(candidates: &Vec<i32>, target: i32, set: &mut HashMap<i32, Vec<Vec<i32>>) {
        if target == 0 {
            set.insert(0, vec![]);
        }
    }
}
