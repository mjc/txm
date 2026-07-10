use std::env;

use unicode_width::UnicodeWidthStr;

fn boxed(rendered: &str) -> String {
    let lines: Vec<&str> = rendered.lines().collect();
    let width = lines
        .iter()
        .map(|line| UnicodeWidthStr::width(*line))
        .max()
        .unwrap_or(0);
    let height = lines.len();
    let w = width + 4;
    let h = height + 4;
    let mut box_data = vec![' '; w * h];

    box_data[0] = '┌';
    box_data[w - 1] = '┐';
    box_data[(h - 1) * w] = '└';
    box_data[(h - 1) * w + w - 1] = '┘';

    for x in 1..w - 1 {
        box_data[x] = '─';
        box_data[(h - 1) * w + x] = '─';
    }

    for y in 1..h - 1 {
        box_data[y * w] = '│';
        box_data[y * w + w - 1] = '│';
    }

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            box_data[(y + 2) * w + x + 2] = c;
        }
    }

    let mut out = String::with_capacity(box_data.len() + h);
    for y in 0..h {
        for x in 0..w {
            out.push(box_data[y * w + x]);
        }
        out.push('\n');
    }
    out
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: txm [LaTeX input]");
        return;
    }

    let rendered = txm::render(&args[1]).unwrap();
    print!("{}", boxed(&rendered));
}
