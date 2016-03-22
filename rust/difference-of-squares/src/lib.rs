pub fn square_of_sum(end_number: u32) -> u32 {
    (1..(end_number + 1)).fold(0, |sum, x| sum + x).pow(2)
}

pub fn sum_of_squares(end_number: u32) -> u32 {
    (1..(end_number + 1)).fold(0, |sum, x| sum + x.pow(2))
}

pub fn difference(end_number: u32) -> u32 {
    square_of_sum(end_number) - sum_of_squares(end_number)
}
