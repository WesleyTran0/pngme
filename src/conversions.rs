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

// Converts bit representations to its byte where idx 0 is the MSB
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

// Converts 4 bytes to its u32 representation where index 0 is the MSB
pub fn bytes_to_u32(rep: [u8; 4]) -> u32 {
    let mut sum = 0u32;

    // EFFECT: Converts each u8 to its u32 representation and adds it to sum
    for (idx, &byte) in rep.iter().enumerate() {
        sum += (byte as u32) << (8 * (3 - idx));
    }

    sum
}
