use std::fs;

fn main() {
    process_file("playlist.m3u8").unwrap();
}

fn process_file(file_name: &str) -> std::io::Result<Vec<String>> {
    let content = fs::read_to_string(file_name)?;

    let match_lines: Vec<&str> =
        content.split("\n")
        // .enumerate()
        .filter(|x| x.contains(".ts"))
        .collect();

    for line in &match_lines {
        println!("{}", line);
    }

    Ok(vec![String::from("test")])
}
