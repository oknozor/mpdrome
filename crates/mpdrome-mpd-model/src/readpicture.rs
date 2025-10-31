use mpdrome_macro::MpdResponse;

#[derive(MpdResponse)]
pub struct ReadPicture {
    r#type: String,
    #[mpd(binary)]
    picture: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpdrome_macro::ToMpdResponse;

    #[test]
    fn test_mpd_response() {
        let response = ReadPicture {
            r#type: "img/png".to_string(),
            picture: "1234567890".as_bytes().to_vec(),
        };

        let mut buffer = Vec::new();
        response.write_response(&mut buffer).unwrap();
        let actual = String::from_utf8_lossy(&buffer);
        assert_eq!(actual, "type: img/png\nbinary: 10\n1234567890\nOK\n");
    }
}
