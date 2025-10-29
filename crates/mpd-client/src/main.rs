use mpd::{Query, Term};

fn main() {
    let mut client = mpd::Client::connect("127.0.0.1:6600").unwrap();
    let commands = client.commands().unwrap();
    println!("{:?}", commands);

    let res = client
        .list(&Term::Tag("Artist".into()), &Query::new())
        .unwrap();
    println!("{:?}", res);
}
