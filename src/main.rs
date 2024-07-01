use clap::{Arg, ArgAction, Command};
use std::{
    env,
    io::{self, Write},
};

mod options {
    pub const NO_NEWLINE: &str = "no_newline";
    pub const STRING: &str = "STRING";
}

fn main() -> io::Result<()> {
    let matches = app_commmad().get_matches_from(env::args_os());
    let no_newline = matches.get_flag(options::NO_NEWLINE);
    let all = match matches.get_many::<String>(options::STRING) {
        Some(s) => s.map(|s| s.to_string()).collect(),
        None => vec![String::new()],
    };
    write_args(no_newline, all)
}

fn write_args(no_newline: bool, args: Vec<String>) -> io::Result<()> {
    let mut stdout = io::stdout();

    if !no_newline {
        writeln!(stdout, "{}", args.join(" "))?;
    } else {
        write!(stdout, "{}", args.join(" "))?;
    }

    Ok(())
}

fn app_commmad() -> Command {
    Command::new("my-echo")
        .bin_name("/target/debug/my-echo")
        .dont_delimit_trailing_values(true)
        .about("my-echo - write arguments to the standard output")
        .arg(
            Arg::new(options::NO_NEWLINE)
                .short('n')
                .help("do not output trailing `\n`")
                .action(ArgAction::SetTrue)
                .overrides_with(options::NO_NEWLINE),
        )
        .arg(Arg::new(options::STRING).action(ArgAction::Append))
}
