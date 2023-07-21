use std::{fs, io, path::PathBuf};

struct MatchLine {
    line_no: usize,
    content: String,
}

fn main() -> io::Result<()> {
    let entries = fs::read_dir("texts")?
        .map(|e| e.map(|dir| dir.path()))
        .collect::<Result<Vec<PathBuf>, _>>()?;

    for file in &entries {
        let file_name = file.file_name().unwrap().to_str().unwrap();
        println!("{}", file_name);
    }

    // process_file("playlist.m3u8").unwrap();

    Ok(())
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
