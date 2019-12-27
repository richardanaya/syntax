#![no_std]

#[macro_export]
macro_rules! js {
    ($($x:tt)*) => {
        {
        register_function(anystring!($($x)*))
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
        register_function(anystring!($($x)*))
        }
    };
}

#[macro_export]
macro_rules! sql {
    ($($x:tt)*) => {
        {
        register_function(anystring!($($x)*))
        }
    };
}