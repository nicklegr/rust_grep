use std::{fs, io, path, sync::mpsc, result};
use threadpool::ThreadPool;
use tokio::task::JoinHandle;

struct FileResult {
    file_name: String,
    match_lines: Vec<MatchLine>,
}

struct MatchLine {
    line_no: usize,
    content: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // iterator of Result<T, E> items can be collected into Result<Collection<T>, E>
    let paths =
        fs::read_dir("texts")?
        .map(|e| e.map(|dir| dir.path())) // e.map()はResultに対するmap
        .collect::<Result<Vec<path::PathBuf>, _>>()?;

    // read_dirのイテレート中のエラーを無視するならシンプルに書ける
    // let entries: Vec<PathBuf> = fs::read_dir("texts")?
    //     .filter_map(|e| e.ok())
    //     .map(|e| e.path())
    //     .collect();

    // process_files(paths)
    process_files_async(paths).await
}

fn process_files(paths: Vec<path::PathBuf>) -> io::Result<()> {
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    let (sender, receiver) = mpsc::channel::<FileResult>();

    // pathbufをborrowではなくmoveすればpathsの寿命が尽きても大丈夫
    for pathbuf in paths {
        let sender = sender.clone();
        pool.execute(move || {
            let path = pathbuf;
            let file_name = path.file_name().unwrap().to_str().unwrap();
            // println!("{}", file_name);

            let match_lines = process_file(&path).unwrap();
            let result = FileResult { file_name: file_name.to_string(), match_lines };
            sender.send(result).unwrap();
        });
    }
    drop(sender);

    pool.join();

    while let Ok(file_result) = receiver.recv() {
        for line in file_result.match_lines {
            println!("{}:{}: {}", file_result.file_name, line.line_no, line.content);
        }
    }

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

async fn process_files_async(paths: Vec<path::PathBuf>) -> io::Result<()> {
    let handles: Vec<JoinHandle<FileResult>> =
        paths.into_iter().map(|path|
            tokio::spawn(async move {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let match_lines = process_file_async(&path).await.unwrap();
                FileResult { file_name: file_name.to_string(), match_lines }
            })
        ).collect();

    for handle in handles {
        let result = handle.await.unwrap();
        for line in result.match_lines {
            println!("{}:{}: {}", result.file_name, line.line_no, line.content);
        }
    }

    Ok(())
}

async fn process_file_async(path: &path::Path) -> io::Result<Vec<MatchLine>> {
    let content = tokio::fs::read_to_string(path).await?;

    let match_lines: Vec<MatchLine> =
        content.lines()
        .enumerate()
        .filter(|x| x.1.contains("def "))
        .map(|x| MatchLine { line_no: x.0 + 1, content: String::from(x.1) })
        .collect();

    Ok(match_lines)
}
