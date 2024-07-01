use std::env;

fn main() {
    let args: Vec<_> = env::args_os()
        .into_iter()
        .map(|x| x.to_string_lossy().into_owned().as_str().to_owned())
        .collect();

    if args.len() > 1 {
        println!("{}", args[1..].join(" "));
    } else {
        println!("");
    }
}
