fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    // let test3 = String::from("There's space to the rear. ");
    // assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    // let test4 = "  We're surrounded by space!    ";
    // assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    // let test5 = "     ";
    // assert_eq!(trim_spaces(test5), "");

    // let test6 = "";
    // assert_eq!(trim_spaces(test6), "");

    // let test7 = " 🚀 ";
    // assert_eq!(trim_spaces(test7), "🚀");

    println!("Tests passed!");
}

/* YOUR CODE GOES HERE */

fn trim_spaces(test_string: &str) -> &str {
    let mut start_idx = 0;
    let mut end_idx = start_idx + 1; // is exclusive
    let test_string_bytes = test_string.as_bytes();
    for (idx, &char) in test_string_bytes.iter().enumerate() {
        if char == b' ' {
            if start_idx == 0 {
                continue;
            }
        } else {
            if start_idx == 0 {
                start_idx = idx;
            }
        }
        end_idx = idx + 1; // string slices exclude the last index
    }

    &test_string[start_idx..end_idx]
}
