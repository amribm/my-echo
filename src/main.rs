use std::{env, ffi::OsString};

fn main() {
    let args: Vec<_> = env::args_os().collect();

    if args.len() > 1 {
        println!("{:?}", args[1..].join(&OsString::from(" ")));
    } else {
        println!("");
    }
}
