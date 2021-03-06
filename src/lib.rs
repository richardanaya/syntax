#![no_std]

pub use anystring::anystring;

#[macro_export]
macro_rules! js {
    ($($x:tt)*) => {
        {
        anystring!($($x)*)
        }
    };
}

#[macro_export]
macro_rules! html {
    ($($x:tt)*) => {
        {
        register_function(anystring!($($x)*))
        }
    };
}

#[macro_export]
macro_rules! css {
    ($($x:tt)*) => {
        {
        anystring!($($x)*)
        }
    };
}

#[macro_export]
macro_rules! sql {
    ($($x:tt)*) => {
        {
        anystring!($($x)*)
        }
    };
}
