pub(crate) fn capitalize_first_letter(s: &str) -> String {
    // Unicode case conversion can change the length of the string, so we can't capitalize in place.
    // Instead we extract the first character and convert it to uppercase. This returns
    // an iterator which we collect into a string, and then append the rest of the input.
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
