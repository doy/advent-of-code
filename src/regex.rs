macro_rules! regex_captures {
    ($rx:expr, $s:expr $(,)?) => {{
        static RX: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($rx).unwrap());
        RX.captures($s)
    }};
}
