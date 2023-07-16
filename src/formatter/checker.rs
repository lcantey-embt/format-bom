pub fn is_buf_utf8(buf: &Vec<u8>) -> bool {
    let utf8_check = String::from_utf8(buf.clone());
    if utf8_check.is_err() {
        return false;
    }

    true
}
