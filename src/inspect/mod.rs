use std::str::from_utf8;

pub fn inspect(bytes: &[u8]) {
    let text = from_utf8(bytes).unwrap();
    let mut lines = text.lines();
    let request_line = lines.next().unwrap();
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    for line in lines {
        if line.starts_with("Host" ) {
            let host = line["Host:".len()..].trim();
            println!("{} {} {} ", parts[0], parts[1], host);
        }
    }
}