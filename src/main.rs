extern crate ansi_term;
extern crate clap;
extern crate conv;
extern crate rand;

use std::f32::consts::FRAC_PI_3;
use std::fs::File;
use std::io::BufReader;
use std::io::StdoutLock;
use std::io::prelude::*;

use clap::Arg;
use conv::*;

const DESC: &'static str =
r"Concatenate FILE(s), or standard input, to standard output with rainbows.
With no FILE, or when FILE is -, read standard input.";

const EXAMPLES: &'static str = 
r"EXAMPLES:
    roflcat f - g      Output f's contents, then stdin, then g's contents.
    roflcat            Copy standard input to standard output.
    fortune | roflcat  Display a rainbow cookie.";

fn main() {
    let options = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        // .author(env!("CARGO_PKG_AUTHORS"))
        .about(DESC)
        .args(&[
            Arg::from_usage("[FILE]...")
                .help("files to concatenate to standard output"),
            Arg::from_usage("[spread] -p, --spread=<f>")
                .help("Rainbow spread")
                .default_value("3.0"),
            Arg::from_usage("[freq] -f, --freq=<f>")
                .help("Rainbow frequency")
                .default_value("0.1"),
            Arg::from_usage("[duration] -d, --duration=<i>")
                .help("Animation duration")
                .default_value("12"),
            Arg::from_usage("[speed] -s, --speed=<f>")
                .help("Animation speed")
                .default_value("20.0"),
            Arg::from_usage("[seed] -S, --seed=<i>")
                .help("Rainbow seed (0 = random)")
                .default_value("1"),
            Arg::from_usage("-a, --animate   'Enable psychedelics'"),
            Arg::from_usage("-i, --invert    'Invert fg and bg'"),
            Arg::from_usage("-t, --truecolor '24-bit (truecolor)'"),
            // Arg::from_usage("-f, --force        'Force color even when stdout is not a tty'"),
        ])
        .after_help(EXAMPLES)
        .get_matches();
    
    let spread = options.value_of("spread").unwrap().parse().unwrap();
    let freq = options.value_of("freq").unwrap().parse().unwrap();
    // let duration = options.value_of("duration").unwrap().parse().unwrap();
    // let speed = options.value_of("speed").unwrap().parse().unwrap();
    let seed = options.value_of("seed").unwrap().parse().unwrap();
    
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    if let Some(file_names) = options.values_of("FILE") {
        for file_name in file_names {
            if file_name == "-" {
                // Read from stdin
                cat(&mut handle, freq, spread, seed);
            } else {
                // Read file
                let file = match File::open(file_name) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("roflcat: {}: {}", file_name, e);
                        return;
                    }
                };
                let mut buf_reader = BufReader::new(file);
                cat(&mut buf_reader, freq, spread, seed);
            }
        }
    } else {
        cat(&mut handle, freq, spread, seed);
    }
}


fn rainbow(freq: f32, i: f32) -> (u8, u8, u8) {
    let red = ((freq*i).sin() * 127.0 + 128.0).approx().unwrap();
    let green = ((freq*i + 2.0*FRAC_PI_3).sin() * 127.0 + 127.0).approx().unwrap();
    let blue = ((freq*i + 4.0*FRAC_PI_3).sin() * 127.0 + 127.0).approx().unwrap();

    (red, green, blue)
}

fn cat<B: BufRead>(feed: &mut B, freq: f32, spread: f32, seed: usize) {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for (i, line) in feed.lines().enumerate() {
        cat_line(&mut handle, &line.unwrap(), freq, spread, seed + i);
    }
}

fn cat_line(stdout: &mut StdoutLock, s: &str, freq: f32, spread: f32, seed: usize) {
    for (i, ch) in s.chars().enumerate() {
        let rgb = rainbow(freq, f32::value_from(seed).unwrap() + f32::value_from(i).unwrap() / spread);
        write!(stdout, "{}", ansi_term::Colour::RGB(rgb.0, rgb.1, rgb.2).paint(ch.to_string()));
    }
    writeln!(stdout);
}
