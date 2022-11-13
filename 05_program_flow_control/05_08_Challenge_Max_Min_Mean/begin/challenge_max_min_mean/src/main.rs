fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut mean: f64 = 0.0;

    for num in numbers {
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
        mean += num as f64;
    }

    mean /= numbers.len() as f64;

    /* YOUR CODE GOES HERE */

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
