use std::{ffi::OsStr, path::Path};

use rand::{distributions::Alphanumeric, Rng};

pub fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn get_extension(file_name: &str) -> &str {
    Path::new(file_name)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
}
