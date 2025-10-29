pub mod requests;
pub mod responses;

#[cfg(test)]
mod test {
    use crate::protocol::requests::Request;
    use std::str::FromStr;

    #[test]
    fn test_request_commands() {
        assert_eq!(Request::from_str("commands").unwrap(), Request::Commands);
    }
}
