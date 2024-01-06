pub fn concat_u32_to_u64((num1, num2): (u32, u32)) -> u64 {
    let result: u64 = (num1 as u64) << 32 | (num2 as u64);
    result
}

// pub fn split_u64_to_u32s(combined: u64) -> (u32, u32) {
//     let num1: u32 = (combined >> 32) as u32;
//     let num2: u32 = combined as u32;
//     (num1, num2)
// }
