use clap::{Arg, ArgMatches};
use colored::Colorize;
use std::borrow::BorrowMut;
use std::io::Write;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::{io, thread, time};

/// Loop a command.
fn main() {
    let matches = parse_args();
    let args: Vec<&str> = matches.values_of("CMD").unwrap().collect();
    let while_ok = matches.is_present("while_ok");
    let while_ko = matches.is_present("while_ko");
    let iter = matches.get_one::<usize>("iter");
    let delay = matches.get_one::<usize>("delay");
    let stats = !matches.is_present("no_stat");
    let cmd = args[0];

    let progress = Progress::new(cmd);
    let mut progress = Arc::new(Mutex::new(progress));

    // Handler to manage ctr+c interruptions.
    {
        let progress = progress.clone();
        ctrlc::set_handler(move || {
            if stats {
                progress.lock().unwrap().print();
            }
            exit(0);
        })
        .expect("Error setting Ctrl-C handler");
    }

    // The main loop of loop...
    let progress = &mut *progress.borrow_mut();
    loop {
        let output = Command::new(cmd).args(&args[1..]).output();

        match output {
            Ok(output) => {
                // This command iteration has been executed, we test the exit code.
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
                if output.status.success() {
                    progress.lock().unwrap().inc_ok();
                    if while_ko {
                        break;
                    }
                } else {
                    progress.lock().unwrap().inc_ko();
                    if while_ok {
                        break;
                    }
                }
            }
            Err(_) => {
                // This command iteration can't been executed.
                eprintln!("{}: unable to execute {}", "warning".yellow().bold(), cmd);
                progress.lock().unwrap().inc_ko();
                if while_ok {
                    break;
                }
            }
        }
        if let Some(iter) = iter {
            if progress.lock().unwrap().total() == *iter {
                break;
            }
        }
        if let Some(delay) = delay {
            let delay_millis = time::Duration::from_millis(*delay as u64);
            thread::sleep(delay_millis);
        }
    }

    if stats {
        progress.lock().unwrap().print();
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
            Arg::new("no_stat")
                .long("no-stat")
                .required(false)
                .takes_value(false)
                .help("Do not display statistics at the end of execution"),
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

/// Represents a state of the loop.
struct Progress {
    ok: usize,
    ko: usize,
    name: String,
}

impl Progress {
    /// Creates a new progress for the command named `name`.
    fn new(name: &str) -> Progress {
        Progress {
            ok: 0,
            ko: 0,
            name: name.to_string(),
        }
    }

    /// Increments the number of commands that have succeeded.
    fn inc_ok(&mut self) {
        self.ok += 1
    }

    /// Increments the number of commands that have failed.
    fn inc_ko(&mut self) {
        self.ko += 1
    }

    /// Returns the total of commands executed.
    fn total(&self) -> usize {
        self.ok + self.ko
    }

    /// Prints a summary of this progress.
    fn print(&self) {
        let total = self.total().to_string().cyan().bold();
        let ok = if self.ok > 0 {
            self.ok.to_string().green().bold()
        } else {
            "0".bold()
        };
        let ko = if self.ko > 0 {
            self.ko.to_string().red().bold()
        } else {
            "0".bold()
        };

        eprintln!(
            "\n{} total: {} ok: {} ko: {}",
            self.name.bold(),
            total,
            ok,
            ko
        )
    }
}
