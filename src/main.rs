use std::env;
use std::io::{self, Write};

fn main() {
	let args: Vec<String> = env::args().collect();
    let yes = if args.len() <= 1 {
		String::from("y\n")
	} else {
		let mut yes = String::new();
		yes.push_str(&args[1]);
		for i in 2..args.len() {
			yes.push(' ');
			yes.push_str(&args[i]);
			yes.trim();
		}
		yes.push('\n');
		yes
	};
	if arg_compare(&yes) {
		let yes = yes.as_bytes();
	    loop {
	    	match io::stdout().write_all(yes) {
	    		Ok(_) => continue,
	    		Err(_) => break,
	    	};
		}
	}
}

fn arg_compare(yes: &str) -> bool {
	if yes.len() > 2 {
		let mut test = yes.to_string();
		test.truncate(3);
		match test.as_bytes() {
			b"--h" => {
				println!("Help");
				return false
			},
			b"--v" => {
				println!("Version");
				return false
			},
			_ => return true,
		};
	} else {
		return true;
	}
}