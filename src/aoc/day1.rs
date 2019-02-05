// the device shows a sequence of changes in frequency (your puzzle input). A value like +6 means the current frequency increases by 6; a value like -3 means the current frequency decreases by 3.

// For example, if the device displays frequency changes of +1, -2, +3, +1, then starting from a frequency of zero, the following changes would occur:

// Current frequency  0, change of +1; resulting frequency  1.
// Current frequency  1, change of -2; resulting frequency -1.
// Current frequency -1, change of +3; resulting frequency  2.
// Current frequency  2, change of +1; resulting frequency  3.

// In this example, the resulting frequency is 3.
pub fn calculate_chronal_frequency(freq_changes: &[i32]) -> i32{
    return 1;
}

#[cfg(test)]
mod tests {
    use super::*; 
    //tests taken from the advent of code page
    #[test]
    fn day1_p1_test_1() {
        let input = [1, -2, 3, 1];
        let result = calculate_chronal_frequency(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn day1_p1_test_2() {
        let input = [1, 1, 1];
        let result = calculate_chronal_frequency(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn day1_p1_test_3() {
        let input = [1, 1, -2];
        let result = calculate_chronal_frequency(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn day1_p1_test_4() {
        let input = [-1, -2, -3];
        let result = calculate_chronal_frequency(&input);
        assert_eq!(result, -6);
    }
    
}

