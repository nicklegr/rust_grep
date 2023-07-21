use std::{fs, io, path::PathBuf};

struct MatchLine {
    line_no: usize,
    content: String,
}

fn main() -> io::Result<()> {
    // iterator of Result<T, E> items can be collected into Result<Collection<T>, E>
    let entries = fs::read_dir("texts")?
        .map(|e| e.map(|dir| dir.path())) // e.map()はResultに対するmap
        .collect::<Result<Vec<PathBuf>, _>>()?;

    // read_dirのイテレート中のエラーを無視するならシンプルに書ける
    // let entries: Vec<PathBuf> = fs::read_dir("texts")?
    //     .filter_map(|e| e.ok())
    //     .map(|e| e.path())
    //     .collect();

    for file in &entries {
        // let file_name = file.file_name().unwrap().to_str().unwrap();
        // println!("{}", file_name);
        process_file(file.as_path().to_str().unwrap())?;
    }

    Ok(())
}

fn process_file(file_name: &str) -> io::Result<Vec<MatchLine>> {
    let content = fs::read_to_string(file_name)?;

    let match_lines: Vec<MatchLine> =
        content.lines()
        .enumerate()
        .filter(|x| x.1.contains("def "))
        .map(|x| MatchLine { line_no: x.0 + 1, content: String::from(x.1) })
        .collect();

    for line in &match_lines {
        println!("{}:{}: {}", file_name, line.line_no, line.content);
    }

    Ok(match_lines)
}
