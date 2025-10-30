use std::io::{BufRead, Write};

use mpd::{Query, Term, search::Window};

fn main() {
    let mut client = mpd::Client::connect("127.0.0.1:6600").unwrap();
    let commands = client.commands().unwrap();
    println!("{:?}", commands);

    let res = client
        .list(&Term::Tag("Artist".into()), &Query::new())
        .unwrap();

    let mut stream = std::net::TcpStream::connect("127.0.0.1:6600").unwrap();
    stream
        .write_all(b"find \"((Artist == 'Machine Girl'))\"\n")
        .unwrap();
    let mut response = String::new();
    let mut reader = std::io::BufReader::new(&stream);
    // "find \"((Artist == 'Machine Girl'))\""
    // "find \"((Artist == 'Macine Girl'))\""
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                print!("{}", line);
                std::io::stdout().flush().unwrap();
                response.push_str(&line);
            }
            Err(e) => {
                eprintln!("Error reading response: {}", e);
                break;
            }
        }
    }
    println!("{}", response);
}
