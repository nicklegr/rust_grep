use std::{fs, io};

struct MatchLine {
    line_no: usize,
    content: String,
}

fn main() {
    process_file("playlist.m3u8").unwrap();
}

fn process_file(file_name: &str) -> io::Result<Vec<MatchLine>> {
    let content = fs::read_to_string(file_name)?;

    let match_lines: Vec<MatchLine> =
        content.lines()
        .enumerate()
        .filter(|x| x.1.contains(".ts"))
        .map(|x| MatchLine { line_no: x.0 + 1, content: String::from(x.1) })
        .collect();

    for line in &match_lines {
        println!("{}: {}", line.line_no, line.content);
    }

    Ok(match_lines)
}
