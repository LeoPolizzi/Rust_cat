use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

#[derive(Clone, Copy)]
struct OptionFlag {
    active: bool,
}

struct Options {
    number_lines: OptionFlag,
    number_lines_nonblank: OptionFlag,
    squeeze_blank: OptionFlag,
    show_ends: OptionFlag,
    show_tabs: OptionFlag,
    show_nonprint: OptionFlag,
}

fn main() -> io::Result<()> {
    let mut opts = Options {
        number_lines: OptionFlag { active: false },
        number_lines_nonblank: OptionFlag { active: false },
        squeeze_blank: OptionFlag { active: false },
        show_ends: OptionFlag { active: false },
        show_tabs: OptionFlag { active: false },
        show_nonprint: OptionFlag { active: false },
    };

    let mut files: Vec<String> = Vec::new();

    for arg in std::env::args().skip(1) {
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg.chars().skip(1) {
                match ch {
                    'A' => {
                        opts.show_ends.active = true;
                        opts.show_tabs.active = true;
                        opts.show_nonprint.active = true;
                    }
                    'b' => opts.number_lines_nonblank.active = true,
                    'e' => {
                        opts.show_ends.active = true;
                        opts.show_nonprint.active = true;
                    }
                    'E' => opts.show_ends.active = true,
                    'n' => opts.number_lines.active = true,
                    's' => opts.squeeze_blank.active = true,
                    't' => {
                        opts.show_tabs.active = true;
                        opts.show_nonprint.active = true;
                    }
                    'T' => opts.show_tabs.active = true,
                    'v' => opts.show_nonprint.active = true,
                    _ => {
                        eprintln!("Unknown option: -{}", ch);
                        std::process::exit(1);
                    }
                };
            }
        } else if arg == "-" {
            files.push(String::from("/dev/stdin"));
        } else {
            files.push(arg);
        }
    }

    if files.is_empty() {
        files.push(String::from("/dev/stdin"));
    }

    let no_opts = !opts.number_lines.active
        && !opts.number_lines_nonblank.active
        && !opts.squeeze_blank.active
        && !opts.show_ends.active
        && !opts.show_tabs.active
        && !opts.show_nonprint.active;

    let stdout = io::stdout();
    let mut stdout = BufWriter::with_capacity(64 * 1024, stdout.lock());
    let mut line_number = 1u64;
    let mut prev_blank = false;
    let mut buffer = [0u8; 8192];
    let mut out_buf = Vec::with_capacity(64 * 1024);

    for filename in files {
        let input: Box<dyn Read> = if filename == "/dev/stdin" {
            Box::new(io::stdin().lock())
        } else {
            Box::new(File::open(&filename)?)
        };

        if no_opts {
            std::io::copy(&mut BufReader::new(input), &mut stdout)?;
            continue;
        }

        let mut reader = BufReader::new(input);

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }

            let mut start = 0;
            for i in 0..n {
                if buffer[i] == b'\n' {
                    process_line(
                        &buffer[start..=i],
                        &opts,
                        &mut line_number,
                        &mut prev_blank,
                        &mut out_buf,
                    );
                    stdout.write_all(&out_buf)?;
                    out_buf.clear();
                    start = i + 1;
                }
            }

            if start < n {
                out_buf.extend_from_slice(&buffer[start..n]);
            }
        }
    }

    stdout.flush()?;
    Ok(())
}

#[inline(always)]
fn process_line(
    line: &[u8],
    opts: &Options,
    line_number: &mut u64,
    prev_blank: &mut bool,
    out_buf: &mut Vec<u8>,
) {
    let mut i = 0;
    let mut last_was_newline = false;

    let is_nonblank = line.len() > 1 || (line.len() == 1 && line[0] != b'\n');

    if opts.squeeze_blank.active && line == b"\n" {
        if *prev_blank {
            return;
        }
        *prev_blank = true;
    } else if line.last() == Some(&b'\n') {
        *prev_blank = false;
    }

    if (opts.number_lines_nonblank.active && is_nonblank) || opts.number_lines.active {
        write_line_number(out_buf, *line_number);
        *line_number += 1;
    }

    while i < line.len() {
        let b = line[i];
        last_was_newline = b == b'\n';
        let mut write_byte = true;

        if opts.show_nonprint.active && b != b'\n' && b != b'\t' && (b < 32 || b == 127) {
            out_buf.push(b'^');
            out_buf.push(b + 64);
            write_byte = false;
        }

        if opts.show_tabs.active && b == b'\t' {
            out_buf.extend_from_slice(b"^I");
            write_byte = false;
        }

        if write_byte {
            out_buf.push(b);
        }

        i += 1;
    }

    if opts.show_ends.active && last_was_newline && !out_buf.is_empty() {
        out_buf.pop();
        out_buf.extend_from_slice(b"$\n");
    }
}

#[inline(always)]
fn write_line_number(out_buf: &mut Vec<u8>, n: u64) {
    if n <= 999_999 {
        let mut i = 5;
        let mut tmp = n;
        let mut buf = [b' '; 6];

        if tmp == 0 {
            buf[i] = b'0';
        } else {
            while tmp > 0 {
                buf[i] = b'0' + (tmp % 10) as u8;
                tmp /= 10;
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }

        out_buf.extend_from_slice(&buf);
    } else {
        let mut tmp = n;
        let mut digits = 0;

        if tmp == 0 {
            digits = 1;
        } else {
            let mut t = tmp;
            while t > 0 {
                digits += 1;
                t /= 10;
            }
        }

        let start = out_buf.len();
        out_buf.resize(start + digits, 0);
        let mut pos = start + digits;

        while tmp > 0 {
            pos -= 1;
            out_buf[pos] = b'0' + (tmp % 10) as u8;
            tmp /= 10;
        }
    }

    out_buf.push(b'\t');
}
