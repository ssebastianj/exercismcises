pub fn is_leap_year(year: u16) -> bool {
    let year_is_div_by_4 = year % 4 == 0;
    let year_is_div_by_100 = year % 100 == 0;
    let year_is_div_by_400 = year % 400 == 0;

    year_is_div_by_4 && (!year_is_div_by_100 || year_is_div_by_400)
}
