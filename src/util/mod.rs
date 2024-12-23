use alloy::hex::decode;
use core::str;

use reqwest::Url;
use sha2::{Digest, Sha256};

pub fn sha256_double(input: impl AsRef<[u8]>) -> Vec<u8> {
    sha256(sha256(input))
}

pub fn sha256(input: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_ref());
    hasher.finalize().to_vec()
}

pub fn decode_url(input: &str) -> anyhow::Result<Url> {
    let bytes = decode(input)?;
    let url = Url::parse(str::from_utf8(&bytes)?)?;
    Ok(url)
}

#[cfg(test)]
mod tests {
    use alloy::hex::encode;
    use reqwest::Url;

    use crate::util::sha256_double;

    use super::{decode_url, sha256};

    #[test]
    fn sha256_test() {
        let data = b"data";

        let first_result = sha256(&data);
        let second_result = sha256(&data);

        assert_eq!(first_result, second_result);
    }

    #[test]
    fn sha256_double_test() {
        let data = b"data";

        let first_result = sha256_double(&data);
        let second_result = sha256_double(&data);

        assert_eq!(first_result, second_result);
    }

    #[test]
    fn sha256_double_is_double_test() {
        let data = b"data";

        let expected = sha256(sha256(&data));
        let actual = sha256_double(&data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_url_test() {
        let expected = Url::parse("http://localhost:3000").unwrap();
        let encoded = encode(expected.to_string());

        let actual = decode_url(&encoded).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_url_test_invalid_url() {
        let encoded = encode("invalid_url");

        let actual = decode_url(&encoded);

        assert!(actual.is_err());
    }
}
