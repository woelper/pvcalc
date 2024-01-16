use std::fmt::format;

#[macro_export]
macro_rules! tr {
    ($($arg:tt)*) => {{
        // let res = std::fmt::format(std::fmt::__export::format_args!($($arg)*));
        let res = format!($($arg)*);
        res
    }}
}

