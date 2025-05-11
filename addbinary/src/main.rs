pub fn add_binary(a: String, b: String) -> String{
    let mut a_vec: Vec<char> = a.chars().collect();
    let mut b_vec: Vec<char> = b.chars().collect();
    let lena = a_vec.len();
    let lenb = b_vec.len();
    let n = if lena < lenb {lenb} else {lena};
    if lena < lenb{
        for i in (0..lenb-lena){
            a_vec.insert(0,'0');
        }
    }else{
        for i in (0..lena-lenb){
            b_vec.insert(0, '0');
        }
    }
    let mut result: Vec<char> = Vec::new();
    let mut temp: u32 = 0;
    for i in (0..n).rev(){
        let bit_a = a_vec[i].to_digit(2).unwrap();
        let bit_b = b_vec[i].to_digit(2).unwrap();
        let sum = bit_a + bit_b + temp;
        result.insert(0, std::char::from_digit(sum % 2, 10).unwrap());
        temp = sum/2;
    }
    if temp == 1{
        result.insert(0, '1');
    }
    result.iter().cloned().collect::<String>()
}
#[test]
pub fn smalladd(){
    assert_eq!(add_binary("1".to_string(), "10".to_string()), "11".to_string() );
}

#[test]
pub fn bigadd(){
    assert_eq!(add_binary("111".to_string(), "101".to_string()), "1100".to_string() );
}
