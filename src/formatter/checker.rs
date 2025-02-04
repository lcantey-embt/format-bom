use std::str;

pub fn is_buf_utf8(buf: &[u8]) -> bool {
    let utf8_check = str::from_utf8(buf);
    if utf8_check.is_err() {
        return false;
    }

    true
}
