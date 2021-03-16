#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => ({
	println!($($arg)*);
	std::process::exit(1);
    })
}
