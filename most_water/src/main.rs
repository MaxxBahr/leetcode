fn max_area(height: Vec<i32>) -> i32{
    let mut highest_index = 0;
    let mut highest = 0;
    let mut distance_to_highest = 0;
    let mut result = 0;
    // iterate over vec for first highest value
    for (i,value) in height.iter().enumerate(){
    // store current highest value
        if value > highest{
            highest = value;
            highest_index = i;
        }
    }
    for index in (highest_index..height.len()-1){
        if ((index-highest_index) * height[index]) > distance_to_highest{
            distance_to_highest = index;
        }
    }
    // search for value of height - <distanceToHighest>
    // return highest value
}

#[test]
fn test(){
    assert_eq!()
}