
use super::error::UtilsError;

pub fn u64_to_binary(data: u64, bit_space_len: u32) -> Result<String, UtilsError> {
    let bit_string = format!("{:b}", data);
    let bit_string_len = bit_string.len() as u32;
    if bit_string_len > bit_space_len {
        Err(
            UtilsError::TotalBitSpaceIsLessThanDataSignificantBitLength {
                data_significant_bit_len: bit_string_len,
                bit_space_len,
            },
        )
    } else {
        let num_leading_zeros = bit_space_len - bit_string_len;
        let mut leading_zeros = "0".repeat(num_leading_zeros as usize);
        leading_zeros.push_str(&bit_string);
        let final_bit_string = leading_zeros;
        Ok(final_bit_string)
    }
}

pub fn vec_u8_to_bit_string(data: &[u8]) -> String {
    let mut buff = String::new();
    data.iter()
        .for_each(|byte| buff.push_str(&format!("{:08b}", byte)));
    buff
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u64_to_binary_works_01() {
        let data = 42;
        let bit_space_len = 8;
        let result = u64_to_binary(data, bit_space_len).unwrap();
        let expected = "00101010".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn u64_to_binary_emits_error_01() {
        let data = 42;
        let bit_space_len = 2;
        let result = u64_to_binary(data, bit_space_len);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            UtilsError::TotalBitSpaceIsLessThanDataSignificantBitLength {
                data_significant_bit_len: 6,
                bit_space_len: 2
            }
        )
    }

    #[test]
    fn vec_u8_to_bit_string_works_01() {
        let data = vec![255];
        let result = vec_u8_to_bit_string(&data);
        let expected = "11111111".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn vec_u8_to_bit_string_works_02() {
        let data = vec![1];
        let result = vec_u8_to_bit_string(&data);
        let expected = "00000001".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn vec_u8_to_bit_string_works_03() {
        let data = vec![1, 32, 64, 128];
        let result = vec_u8_to_bit_string(&data);
        let expected = "00000001001000000100000010000000".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn vec_u8_to_bit_string_works_04() {
        let data = vec![243, 198, 200, 29];
        let result = vec_u8_to_bit_string(&data);
        let expected = "11110011110001101100100000011101".to_string();
        assert_eq!(result, expected);
    }
}
