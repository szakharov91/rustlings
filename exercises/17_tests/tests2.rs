// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.

        let po2_1 = 1;  
        let po2_2 = 2;  
        let po2_3 = 8;  
        let po2_4 = 16; 

        assert_eq!(power_of_2(0), po2_1);
        assert_eq!(power_of_2(1), po2_2);
        assert_eq!(power_of_2(3), po2_3); 
        assert_eq!(power_of_2(4), po2_4); 
    }
}
