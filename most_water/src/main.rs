fn max_area(height: Vec<i32>) -> i32{
    let mut highest_index = 0;
    let mut highest = 0;
    let mut distance_to_highest = 0;
    let mut result = 0;
    // iterate over vec for first highest value
    for (i,value) in height.iter().enumerate(){
    // store current highest value
        if *value > highest{
            highest = *value;
            highest_index = i;
        }
    }
    for index in (highest_index..height.len()-1){
    // search for value of height - <distanceToHighest>
        if ((index-highest_index) * height[index]as usize) > distance_to_highest{
            distance_to_highest = index;
        }
    }
    // return highest value
    return ((distance_to_highest - highest_index) * height[distance_to_highest] as usize) as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        // Expected: 49
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn test_example_2() {
        let height = vec![1,1];
        // Expected: 1
        assert_eq!(max_area(height), 1);
    }

    #[test]
    fn test_decreasing() {
        let height = vec![5,4,3,2,1];
        // Expected: 6
        assert_eq!(max_area(height), 6);
    }

    #[test]
    fn test_all_equal() {
        let height = vec![3,3,3,3,3];
        // Expected: 12
        assert_eq!(max_area(height), 12);
    }

    #[test]
    fn test_single_element() {
        let height = vec![7];
        // Expected: 0
        assert_eq!(max_area(height), 0);
    }
}