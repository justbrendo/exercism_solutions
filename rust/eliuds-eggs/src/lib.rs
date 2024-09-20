pub fn egg_count(mut display_value: u32) -> usize {
    let mut i = 0;
    while display_value > 0 {
        if display_value & 1 == 1 {
            i +=1;
        }
        display_value = display_value >> 1;
    }
    i
}
