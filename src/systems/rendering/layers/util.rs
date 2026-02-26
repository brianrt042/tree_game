pub fn read_ascii_content() -> Vec<String>{
    let content = fs::read_to_string("file.txt").unwrap();
    let lines: Vec<String> = content.lines()
        .map(|l| l.to_string())
        .collect();
    return lines;
}