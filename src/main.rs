use std::env;
use std::io::{self, Write};
use std::process;

macro_rules! HELP_DOC {
    () => ("Usage: {} [STRING]...\n\
    or:  {} OPTION\n\
    Repeatedly output a line with all specified STRING(s), or 'y'.\n\
    \n\
    --help     display this help and exit\n\
    --version  output version information and exit\n");
}

macro_rules! VERSION_INFO {
    () => ("{} (rusty tool) 0.1.0\n\
    A basic implementation of yes written in rust.\n\
    License MIT\n\
    This is free software that was written to learn the language\n\
    Source available at https://github.com/RSAkidinUSA/rusty\n\
    \n\
    Written by Ryan Burrow <rsardb11@vt.edu>\n");
}

macro_rules! INVALID_USAGE {
    () => ("{}: invalid option -- '{}'\n\
    Try '{} --help' for more information.\n");
}

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
    let retval = arg_compare(&args[0], &yes);
    if retval.0 {
        let yes = yes.as_bytes();
        loop {
            match io::stdout().write_all(yes) {
                Ok(_) => continue,
                Err(_) => break,
            };
        }
    } else {
        process::exit(retval.1);
    }
}

fn arg_compare(name: &str, yes: &str) -> (bool, i32) {
    if yes.len() > 2 {        
        if yes.as_bytes()[0] != '-' as u8 {
            return (true, 0);
        }

        match &yes[..=2] {
            "--h" => {
                print!(HELP_DOC!(), name, name);
                return (false, 0)
            },
            "--v" => {
                print!(VERSION_INFO!(), name);
                return (false, 0)
            },
            _ => {
                print!(INVALID_USAGE!(), name, yes.as_bytes()[1] as char, name);
                return (false, 1)
            },
        };
    } else {
        return (true, 0);
    }
}