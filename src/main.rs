use std::env;

fn main() {
    let args = args_os();
    write_args(args)
}

fn args_os() -> Vec<String> {
    env::args_os()
        .into_iter()
        .map(|x| x.to_string_lossy().into_owned().as_str().to_owned())
        .collect()
}

fn write_args(args: Vec<String>) {
    if args.len() > 1 {
        println!("{}", args[1..].join(" "));
    } else {
        println!("");
    }
}
