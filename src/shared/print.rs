#[macro_export]
macro_rules! smart_println {
    ($($arg:tt)*) => {
        #[cfg(feature = "cloudflare")]
        {
            worker::console_log!($($arg)*);
        }
        #[cfg(not(feature = "cloudflare"))]
        {
            println!($($arg)*);
        }
    }
}
