#[cfg(test)]
mod tests {
    #[cfg(test)]
    fn test_change_env() {
        std::env::set_var("MY_VARIABLE", "new_value");
    }
}
