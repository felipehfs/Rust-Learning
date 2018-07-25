use std::env;
use std::path::PathBuf;

fn main() {
	let home = env::home_dir().expect("no home found");
	let mut path = PathBuf::new();
	path.push(home);
	path.push(".cargo");
	if path.is_dir() {
		println!("{}", path.display());
	}
}