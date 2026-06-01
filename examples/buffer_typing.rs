use sejong::Buffer;
use std::io::{self, Read, Write};
use std::process::Command;

struct RawMode;

impl RawMode {
    fn enter() -> io::Result<Self> {
        let status = Command::new("stty").args(["raw", "-echo"]).status()?;
        if status.success() {
            Ok(Self)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to enable terminal raw mode",
            ))
        }
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = Command::new("stty").args(["sane"]).status();
    }
}

fn render(committed: &str, composing: &str, last: &str) -> io::Result<()> {
    print!(
        "\r\x1b[2Kcommitted: {} | composing: {} | last: {}",
        committed, composing, last
    );
    io::stdout().flush()
}

fn main() -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut buffer = Buffer::default();
    let mut committed = String::new();

    println!("Sejong buffer typing demo");
    println!("Type QWERTY Korean input. Enter commits, Backspace pops, Space commits a space.");
    println!("Press Ctrl-C, Ctrl-D, or Esc to quit.\n");

    {
        let _raw = RawMode::enter()?;
        render(&committed, &buffer.to_string(), "ready")?;

        for byte in stdin.by_ref().bytes() {
            let byte = byte?;
            let last = match byte {
                3 | 4 | 27 => break,
                b'\r' | b'\n' => {
                    committed.push_str(&buffer.out());
                    "commit".to_string()
                }
                b' ' => {
                    committed.push_str(&buffer.out());
                    committed.push(' ');
                    "space".to_string()
                }
                8 | 127 => match buffer.pop() {
                    Some(()) => "pop".to_string(),
                    None => {
                        committed.pop();
                        "delete committed".to_string()
                    }
                },
                _ => {
                    let c = byte as char;
                    match buffer.put(c) {
                        None => format!("put {}", c),
                        Some(rejected) => format!("ignored {}", rejected),
                    }
                }
            };

            render(&committed, &buffer.to_string(), &last)?;
        }
    }

    committed.push_str(&buffer.out());
    println!("\n\nFinal: {}", committed);

    Ok(())
}
