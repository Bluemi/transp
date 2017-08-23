use std::env;
mod receiver;
mod sender;

fn print_usage() {
	println!("transp:");
	println!("");
	println!("transp -r");
	println!("transp -s IP FILE");
}

fn main() {
	let mut args = env::args().skip(1);
	match args.next() {
		Some(ref x) if x == "-r" => receiver::call(args),
		Some(ref x) if x == "-s" => sender::call(args),
		_ => print_usage(),
	}
}
