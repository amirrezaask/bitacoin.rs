use sha2::Digest;

pub fn simple_hash(data: String) -> String {
    format!("{:x}", sha2::Sha256::digest(data.as_bytes()))
}

pub fn good_enough(hash: &String, dificulty: usize) -> bool {
    let matching_part = &hash[0..dificulty];
    let mut dificulty_string = String::new();
    for i in 0..dificulty {
        dificulty_string.push('0');
    }
    if matching_part != dificulty_string {
        return false;
    }
    return true;
}

// 00ajksdkajshdkajshd
// 0alskdhalksdhalskdhasd
// aklsdhlkashdlkashdllk

pub fn difficult_hash(data: String, dificulty: usize) -> String {
    let mut i = 0;
    loop {
        let data = format!("{}{}", data, i);
        let h = simple_hash(data);
        if good_enough(&h, dificulty) {
            return h
        }
        i = i + 1;
    }
}