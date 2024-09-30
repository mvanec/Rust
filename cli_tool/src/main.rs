use clap::{Arg, Command, ArgAction};

fn main() {
    let matches = Command::new("cli_tool")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Does awesome things")
        .arg(
            Arg::new("input")
                .help("Input file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .help("Increases verbosity")
                .action(ArgAction::Count),
        )
        .get_matches();

    // Access arguments
    let input = matches.get_one::<String>("input").unwrap();
    let verbosity: Vec<_> = matches.get_occurrences::<u8>("verbose").unwrap().map(Iterator::collect::<Vec<_>>).collect();
    let v_level: u8 = if verbosity.len() < 1 { 0 } else { *verbosity[0][0] };

    println!("Input file: {}", input);
    println!("Verbosity level: {}", v_level);
}