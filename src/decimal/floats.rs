const MANTISSA_BITS: f32 = 23.0;
const RADIX: f32 = 2.0;
const BIAS: i32 = 127;


pub fn break_to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xFF;
    let fraction = bits & 0x7FFFFF;

    (sign, exponent, fraction)
}

fn exponent_calc(expo: u8) -> f32 {
    RADIX.powf((expo as i32 - BIAS) as f32)
}

fn mantissa_calc_good(n_bits: u32) -> f32 {
    let mut mantissa: f32 = 1.0; // implicit leading 1

    for i in 0..23 {
        let mask = 1 << (22 - i);
        if n_bits & mask != 0 {
            mantissa += 2f32.powf(-(i as f32 + 1.0));
        }
    }
    mantissa
}

fn mantissa_calc(n_bits: u32) -> f32 {
    let mut mantissa: f32 = 1.0; // implicit leading 1

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
           let i_ = i as f32;
           let weight = 2_f32.powf(i_ - 23.0);
           mantissa += weight;
        } 
    }
    return mantissa;
}


pub fn decode(sign: u32, exponent: u32, fraction: u32 ) -> (f32, f32, f32) {
    let signed_1 = (-1.0f32).powf(sign as f32);

    let exponent_value = exponent_calc(exponent as u8) as f32;
    let mantissa_value = mantissa_calc_good(fraction);

    (signed_1, exponent_value, mantissa_value)
}

pub fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_to_parts() {
        let n: f32 = 42.42;

        let (sign, exponent, mantissa) = break_to_parts(n);
        let (sign_, exp_, mant) = decode(sign, exponent, mantissa);
        let n_ = from_parts(sign_, exp_, mant);

        println!("{} -> {}", n, n_);
        println!("field        | as bits  | as real number");
        println!("sign         | {:08b} | {}", sign, sign_);
        println!("exponent     | {:08b} | {}", exponent, exp_);
        println!("mantissa     | {:023b} | {}", mantissa, mant);
        assert_eq!(n, n_);
    }
}