use std::str::FromStr;

use honggfuzz::fuzz;

fn do_test(data: &[u8]) {
    let data_str = String::from_utf8_lossy(data);

    // signed
    let samt = match litecoin::amount::SignedAmount::from_str(&data_str) {
        Ok(amt) => amt,
        Err(_) => return,
    };
    let samt_roundtrip = match litecoin::amount::SignedAmount::from_str(&samt.to_string()) {
        Ok(amt) => amt,
        Err(_) => return,
    };
    assert_eq!(samt, samt_roundtrip);

    // unsigned
    let amt = match litecoin::amount::Amount::from_str(&data_str) {
        Ok(amt) => amt,
        Err(_) => return,
    };
    let amt_roundtrip = match litecoin::amount::Amount::from_str(&amt.to_string()) {
        Ok(amt) => amt,
        Err(_) => return,
    };
    assert_eq!(amt, amt_roundtrip);
}

fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}

#[cfg(all(test, fuzzing))]
mod tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'..=b'F' => b |= c - b'A' + 10,
                b'a'..=b'f' => b |= c - b'a' + 10,
                b'0'..=b'9' => b |= c - b'0',
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        extend_vec_from_hex("00000000", &mut a);
        super::do_test(&a);
    }
}
