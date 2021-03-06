pub mod stack;

pub mod vm;

#[macro_export]
macro_rules! debug {
    ($($p:tt)*) => (if cfg!(debug_assertions) { println!($($p)*) } else { () })
}
