use std::{fs, io, path, sync::mpsc};
use threadpool::ThreadPool;

struct MatchLine {
    line_no: usize,
    content: String,
}

fn main() -> io::Result<()> {
    // iterator of Result<T, E> items can be collected into Result<Collection<T>, E>
    let entries =
        fs::read_dir("texts")?
        .map(|e| e.map(|dir| dir.path())) // e.map()はResultに対するmap
        .collect::<Result<Vec<path::PathBuf>, _>>()?;

    // read_dirのイテレート中のエラーを無視するならシンプルに書ける
    // let entries: Vec<PathBuf> = fs::read_dir("texts")?
    //     .filter_map(|e| e.ok())
    //     .map(|e| e.path())
    //     .collect();

    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    let (sender, receiver) = mpsc::channel::<path::PathBuf>();

    for pathbuf in entries {
        sender.send(pathbuf).unwrap();
    }
    drop(sender);

    pool.execute(move || {
        while let Ok(file) = receiver.recv() {
            let file_name = file.file_name().unwrap().to_str().unwrap();
            // println!("{}", file_name);

            let match_lines = process_file(&file).unwrap();
            for line in &match_lines {
                println!("{}:{}: {}", file_name, line.line_no, line.content);
            }
        }
    });

    pool.join();

    Ok(())
}

fn process_file(path: &path::Path) -> io::Result<Vec<MatchLine>> {
    let content = fs::read_to_string(path)?;

    let match_lines: Vec<MatchLine> =
        content.lines()
        .enumerate()
        .filter(|x| x.1.contains("def "))
        .map(|x| MatchLine { line_no: x.0 + 1, content: String::from(x.1) })
        .collect();

    Ok(match_lines)
}
