extern crate clap;
extern crate rand;

use clap::{App, Arg, SubCommand, ArgMatches, AppSettings};
use rand::{thread_rng, Rng, sample};

/// Gets the value of an argument as a u64
macro_rules! get_number {
    ($argmatch:expr, $arg:expr) => {{
        $argmatch.value_of($arg)
                 .unwrap()
                 .parse::<u64>()
                 .unwrap()
    }};
}

/// Gets all values from a multi-value argument as a vector
macro_rules! get_list {
    ($argmatch:expr, $arg:expr) => {{
        $argmatch.values_of($arg)
                 .unwrap()
                 .collect::<Vec<&str>>()
    }};
}

fn main() {
    let matches = App::new("randomtools")
                    .version("1.0")
                    .author(concat!("Franziska Becker <buecher.apps@gmail.com>,",
                        " René Wanrking <rwarnking@gmail.com>"))
                    .about("Flips a coin, rolls a dice or selects a random objects from a list")
                    .usage("task2.exe [OPTIONS] <SUBCOMMAND>")
                    .settings(&[AppSettings::SubcommandRequiredElseHelp, AppSettings::ColoredHelp,
                        AppSettings::VersionlessSubcommands])
                    .arg(Arg::with_name("times")
                        .short("t")
                        .long("times")
                        .help("Determines the number of times the command is executed")
                        .takes_value(true)
                        .global(true)
                        .validator(is_a_number))
                    .subcommand(SubCommand::with_name("coin")
                        .about("Flips a coin"))
                    .subcommand(SubCommand::with_name("dice")
                        .about("Rolls a dice")
                        .arg(Arg::with_name("sides")
                            .short("s")
                            .long("sides")
                            .help("The number of sides the dice should have")
                            .validator(is_a_number)
                            .takes_value(true)))
                    .subcommand(SubCommand::with_name("choose")
                        .about("Chooses a random number of elements from a list")
                        .arg(Arg::with_name("count")
                            .short("c")
                            .long("count")
                            .help("The number of elements 'choose' should return")
                            .validator(is_a_number)
                            .takes_value(true))
                        .arg(Arg::with_name("input")
                            .help("The list of elements to choose from")
                            .multiple(true)
                            .required(true)
                            .use_delimiter(true)
                            .value_delimiter(" ")))
                    .get_matches();

    // perform specified 'repeat' times
    let repeat: u64 = repeat_action(&matches);
    for index in 0..repeat {
        // this seperates calls of 'choose' visually, if there is more than 1
        if index > 0 && matches.subcommand_name() == Some("choose") {
            println!("--------------------");
        }
        match matches.subcommand() {
            ("coin", Some(sub)) => {
                // flip a coin 'cycle' times
                let cycle = repeat_action(&sub);
                for _ in 0..cycle {
                    println!("{}" , flip_coin());
                }
            },
            ("dice", Some(sub)) => {
                // if 'sides' was given, get value
                let mut num: u64 = 6;
                if sub.is_present("sides") {
                    num = get_number!(sub, "sides");
                }
                // roll a dice with 'num' sides 'cycle' times
                let cycle = repeat_action(&sub);
                for _ in 0..cycle {
                    println!("{}", roll_dice(num));
                }
            },
            ("choose", Some(sub)) => {
                // if 'count' was given, get value
                let mut num: u64 = 0;
                if sub.is_present("count") {
                    num = get_number!(sub, "count");
                }
                // choose 'num' random elements from 'list' 'cycle' times
                let cycle = repeat_action(&sub);
                let list = get_list!(sub, "input");
                for i in 0..cycle {
                    // seperates calls visually, if there is more than 1
                    if i > 0 {
                            println!("--------------------");
                    }
                    for item in choose_elements(num, &list) {
                        println!("{}", item);
                    }
                }
            },
            _ => {}
        }
    }
}

/// Test whether global argument 'times' is present and if so
/// returns its value, otherwise '1' is returned
fn repeat_action(m: &ArgMatches) -> u64 {
    if m.is_present("times") {
        get_number!(m, "times")
    } else {
        1
    }
}

/// Validator function, tests if the string is a positive number
fn is_a_number(s: String) -> Result<(), String> {
    s.parse::<u64>()
            .map(|_| ())
            .map_err(|_| "argument must hold a positive number".into())
}

/// Flips a coin
fn flip_coin() -> String {
    let mut rng = thread_rng();

    if rng.gen::<bool>() {
        String::from("heads")
    } else {
        String::from("tails")
    }
}

/// Rolls a 'sides'-sided dice
fn roll_dice(sides: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(1, sides + 1)
}

/// Chooses 'num', if it is greater than 0, or a random number of elements from a slice
fn choose_elements<T>(num: u64, elems: &[T]) -> Vec<&T>
    where T: Copy + std::fmt::Display + std::fmt::Debug
{
    // 'elems' cannot be empty because its argument is required,
    // therefore we don't need to check the length or anything like that
    let mut rng = thread_rng();
    let count = rng.gen_range(1, elems.len() + 1);
    sample(&mut rng, elems.iter(), if num == 0 {count} else {num as usize})
}
