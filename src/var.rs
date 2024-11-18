use std::env;

pub fn get_var(name: &str) -> Option<String> {
    env::var(name).ok()
}

pub fn set_var(name: &str, value: &str) {
    env::set_var(name, value);
}

pub fn unset_var(name: &str) {
    env::remove_var(name);
}
