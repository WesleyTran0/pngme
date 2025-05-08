// Converts the given u8 number into its string representation
pub fn u8_to_string(num: u8) -> String {
    (num as char).to_string()
}

// converts a single char to its ASCII representation
pub fn char_to_u8(my_char: char) -> u8 {
    // TODO find a better way to implement this
    my_char as u8
}

// converts single u8 integer to its bit representation from MSB to LSB
//  in the form of booleans
pub fn u8_to_bits(num: u8) -> [bool; 8] {
    // represented from MSB to LSB
    let u8_bin_str = format!("{num:08b}");

    let u8_bin = u8_bin_str.chars().enumerate();
    let mut fin_arr = [false; 8];

    //EFFECT: Determine from the MSB to LSB, if the given bit is true or false
    for (idx, bit_char) in u8_bin {
        fin_arr[idx] = bit_char == '1';
    }

    fin_arr
}

// Converts bit representations to its byte
pub fn bits_to_byte(rep: &[bool; 8]) -> u8 {
    let mut sum = 0u8;

    // EFFECT: Converts each bool to its binary representation and adds it to sum
    for (idx, &bit) in rep.iter().enumerate() {
        if bit {
            sum += 1u8 << (7 - idx);
        }
    }

    sum
}
