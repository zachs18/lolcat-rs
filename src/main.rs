use std::io::{self, prelude::*, BufReader, BufWriter};

#[derive(Clone)]
struct Color{
    fg_hue: Option<u16>,
    bg_hue: Option<u16>,
}

impl Color {
    fn add(&mut self, value: i32) {
        if let Some(hue) = &mut self.fg_hue {
            *hue = ((*hue as i32 + value) % 1530) as u16;
        }
        if let Some(hue) = &mut self.bg_hue {
            *hue = ((*hue as i32 + value) % 1530) as u16;
        }
    }
}

fn hue_to_color(mut hue: u16) -> [u8; 3] {
    hue %= 1530;
    if hue <= 255 { return [255, hue as u8, 0]; }
    else if hue <= 510 { return [(510 - hue) as u8, 255, 0]; }
    else if hue <= 765 { return [0, 255, (hue - 510) as u8]; }
    else if hue <= 1020 { return [0, (1020 - hue) as u8, 255]; }
    else if hue <= 1275 { return [(hue - 1020) as u8, 0, 255]; }
    else /*if hue <= 1530*/ { return [255, 0, (1530 - hue) as u8]; }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(hue) = self.fg_hue {
            let channels = hue_to_color(hue);
            f.write_fmt(format_args!("\x1b[38;2;{};{};{}m", channels[0], channels[1], channels[2]))?;

        }
        if let Some(hue) = self.bg_hue {
            let channels = hue_to_color(hue);
            f.write_fmt(format_args!("\x1b[48;2;{};{};{}m", channels[0], channels[1], channels[2]))?;
        }
        Ok(())
    }
}

struct Clear;

impl std::fmt::Display for Clear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\x1b[0m")
    }
}

fn main() {
    let infile: Box<dyn BufRead> = if let Some(filename) = std::env::args().nth(1) {
        Box::new(BufReader::new(std::fs::File::open(filename).expect("Failed to open file")))
    } else {
        Box::new(BufReader::new(io::stdin().lock()))
    };

    let mut outfile: Box<dyn Write> = Box::new(BufWriter::new(io::stdout().lock()));

    let mut line_start_color = Color { fg_hue: Some(0), bg_hue: None };

    for line in infile.lines() {
        let mut char_color = line_start_color.clone();
        let line = line.unwrap();
        for ch in line.chars() {
            write!(outfile, "{}{}{}", char_color, ch, Clear).unwrap();
            char_color.add(32);
        }
        writeln!(outfile).unwrap();
        line_start_color.add(-96);
    }
    outfile.flush().unwrap();
}
