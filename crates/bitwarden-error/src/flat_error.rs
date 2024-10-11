pub trait FlatError {
    fn get_variant(&self) -> &str;
    fn get_message(&self) -> &str;
}
