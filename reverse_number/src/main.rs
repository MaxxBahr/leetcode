fn main(){
    println!("{}", reverse(-123));
}


fn reverse(x: i32) -> i32{
    if x > std::i32::MAX || x < std::i32::MIN {return 0;}
let val = x;
let mut result: Vec<char> = vec![];
let mut sign = false;
for i in val.to_string().chars(){
    if i == '-'{
        sign = true;
        continue;
    }
    result.push(i);
}
let mut result_value: i32 = 0;
for j in 0..result.len(){
    let digit = result[j].to_digit(10).unwrap() as i32;
    result_value += digit * i32::pow(10, j as u32);
}
if sign{
    result_value = result_value * -1;
}
return result_value;
}
