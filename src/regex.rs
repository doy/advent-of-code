macro_rules! regex_captures {
    ($rx:expr, $s:expr $(,)?) => {{
        lazy_static::lazy_static! {
            static ref RX: Regex = regex::Regex::new($rx).unwrap();
        }
        RX.captures($s)
    }};
}
