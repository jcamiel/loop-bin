use clap::{Arg, ArgMatches};
use colored::Colorize;
use std::io::Write;
use std::process::Command;
use std::{io, thread, time};

/// Loop a command.
fn main() {
    let matches = parse_args();
    let args: Vec<&str> = matches.values_of("CMD").unwrap().collect();
    let while_ok = matches.is_present("while_ok");
    let while_ko = matches.is_present("while_ko");
    let iter = matches.get_one::<usize>("iter");
    let delay = matches.get_one::<usize>("delay");
    let show_stat = matches.is_present("stats");

    let mut executed = 0;
    let mut executed_ok = 0;
    let cmd = args[0];

    // The main loop of loop...
    loop {
        let output = Command::new(cmd)
            .args(&args[1..])
            .output()
            .expect("failed to execute process");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        executed += 1;

        if output.status.success() {
            executed_ok += 1;
            if while_ko {
                break;
            }
        } else if while_ok {
            break;
        }
        if let Some(iter) = iter {
            if executed == *iter {
                break;
            }
        }
        if let Some(delay) = delay {
            let delay_millis = time::Duration::from_millis(*delay as u64);
            thread::sleep(delay_millis);
        }
    }

    if show_stat {
        let executed_str = executed.to_string().cyan().bold();
        let executed_ok_str = if executed_ok > 0 {
            executed_ok.to_string().green().bold()
        } else {
            "0".bold()
        };
        let executed_ko = executed - executed_ok;
        let executed_ko_str = if executed_ko > 0 {
            executed_ko.to_string().red().bold()
        } else {
            "0".bold()
        };

        eprintln!(
            "{} total: {} ok: {} ko: {}",
            cmd.bold(),
            executed_str,
            executed_ok_str,
            executed_ko_str
        )
    }
}

/// Returns the cli parsed arguments.
fn parse_args() -> ArgMatches {
    clap::Command::new("loop")
        .about("Executes a command in loop")
        .arg(
            Arg::new("iter")
                .short('i')
                .long("iter")
                .required(false)
                .takes_value(true)
                .value_parser(clap::value_parser!(usize))
                .help("Number of iteration"),
        )
        .arg(
            Arg::new("while_ok")
                .long("while-ok")
                .required(false)
                .conflicts_with("while_ko")
                .help("Loop while exit code is success"),
        )
        .arg(
            Arg::new("while_ko")
                .long("while-ko")
                .required(false)
                .conflicts_with("while_ok")
                .help("Loop while exit code is failure"),
        )
        .arg(
            Arg::new("delay")
                .short('d')
                .long("delay")
                .required(false)
                .takes_value(true)
                .value_parser(clap::value_parser!(usize))
                .help("Delay between iteration in milliseconds"),
        )
        .arg(
            Arg::new("stats")
                .long("stats")
                .required(false)
                .takes_value(false)
                .help("Display statistics at the end of execution"),
        )
        .arg(
            Arg::new("CMD")
                .required(true)
                .multiple_values(true)
                .help("Command to execute"),
        )
        .trailing_var_arg(true)
        .get_matches()
}
