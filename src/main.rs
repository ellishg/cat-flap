use anyhow::Result;
use clap::Parser;
use humantime::Duration;
use std::io;
use std::io::Write;
use std::string::String;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 's', long, default_value = "100ms")]
    flip_speed: Duration,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for target in io::stdin().lines() {
        let target = target?;
        let mut line = std::iter::repeat('A')
            .take(target.len())
            .collect::<String>();
        let blank_line = std::iter::repeat(' ')
            .take(target.len())
            .collect::<String>();

        loop {
            print!("\r{}\r", blank_line);
            print!("{}", line);
            io::stdout().flush()?;
            std::thread::sleep(args.flip_speed.into());

            if let Some(next) = flip_flaps(&line, &target) {
                line = next;
            } else {
                break;
            }
        }
        print!("\r{}\r", blank_line);
        println!("{}", target);
    }
    Ok(())
}

fn next_char(c: char, target: char) -> char {
    match c {
        c if c == target => target,
        'Z' => 'a',
        'z' => ' ',
        ' ' => '!',
        '@' => '[',
        '`' => '{',
        '~' => target,
        c if c.is_ascii_graphic() => unsafe { std::char::from_u32_unchecked(c as u32 + 1) },
        _ => 'A',
    }
}

fn flip_flaps(line: &str, target: &str) -> Option<String> {
    if line == target {
        None
    } else {
        Some(
            line.chars()
                .zip(target.chars())
                .map(|(c, target)| next_char(c, target))
                .collect::<String>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars() {
        assert_eq!(next_char('A', 'A'), 'A');
        assert_eq!(next_char('A', 'X'), 'B');
        assert_eq!(next_char('~', 'X'), 'X');
        assert_eq!(next_char('~', '\t'), '\t');

        let cycle = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./0123456789:;<=>?@[\\]^_`{|}~";
        for (current, next) in cycle.chars().zip(cycle.chars().skip(1)) {
            assert_eq!(next_char(current, '\t'), next);
        }
    }

    #[test]
    fn test_flaps() {
        assert_eq!(flip_flaps("AAAA", "ABCD"), Some("ABBB".into()));
        assert_eq!(flip_flaps("ABCD", "ABCD"), None);
        assert_eq!(flip_flaps("~~", "❤️"), Some("❤️".into()));
    }
}
