fn main() {
    let test1 = "1003347634";
    let test2 = "1093791627";

    let try1 = get_vector_from_string(test1.to_string());
    let result1 = validate_npi(try1);
    println!("This {} should be valid: {}", test1, result1);
    assert_eq!(true, result1);

    let try2 = get_vector_from_string(test2.to_string());
    let result2 = validate_npi(try2);
    println!("This {} should be invalid: {}", test2, result2);
    assert_eq!(false, result2);
}

fn get_vector_from_string(input: String)  -> Vec<u32> {

    let mut result = vec![]; 
    let vecs : Vec<char> = input.chars().collect();

    //TODO: Refactor this to validate that the elements are digits
    for it in vecs {
        result.push(it.to_digit(10).unwrap());
    }

    result
}
 

fn validate_npi(arr: Vec<u32>) -> bool
{
    if arr.len() != 10 {
        return false
    }

    let mut accumulator = 0;
    let mut index = 1;
    let check_digit = arr.last().unwrap();

    while index <  arr.len() {

        let item = arr[index-1];

        //TODO: Convert to pattern matching
        if index  % 2 == 1 {
            if (item * 2) >= 10 {
                accumulator = accumulator + item * 2 - 10 + 1;
            }
            else {
                accumulator = accumulator + item * 2;
            }
        }
        else {
            accumulator = accumulator + item;
        }
        
        index = index + 1;
    }

    if ceil(accumulator + 24) - (accumulator + 24) == *check_digit {
        true
    }
    else {
        false
    }
}

/// Ceil to the closest next tenth 
fn ceil(i: u32) -> u32 {
    return ((i as f32/10.0)
            .ceil() * 10.0) as u32
}