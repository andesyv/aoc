

fn main() {
    const INPUT: &str = include_str!("../inputs/3.txt");
    // const INPUT: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    let val_count = u32::try_from(INPUT.lines().count()).unwrap();
    let mut sums = Vec::new();
    for line in INPUT.lines() {
        for (i, c) in line.chars().enumerate() {
            if let Some(j) = c.to_digit(10) {
                if i < sums.len() {
                    sums[i] += j;
                } else {
                    sums.push(j)
                }
            }
        }
    }

    let bin_len: u32 = sums.len().try_into().unwrap();
    let binary = sums.iter().map(|n|char::from_digit(u32::from(&(val_count / 2) < n), 10).unwrap()).collect::<String>();
    println!("msb binary: {}", binary);
    let msb = u32::try_from(isize::from_str_radix(&binary, 2).unwrap()).unwrap();
    let lsb = (msb ^ (2u32.pow(bin_len)-1)) & (2u32.pow(bin_len)-1);
    println!("msb is {}, and lsb is {}. Product is: {}", msb, lsb, msb * lsb);

    // let binary_list: Vec<Vec<u8>> = INPUT.lines().map(|s|s.chars().map(|c|u8::try_from(c.to_digit(10).unwrap()).unwrap()).collect::<Vec<u8>>()).collect();
    // let msb: String = binary_list.iter().enumerate().map(|(i, _)|most_common_nth(&binary_list.iter().map(|v|&v[..]).collect::<Vec<&[u8]>>()[..], i)).map(|n|char::from_digit(n.into(), 10).unwrap()).collect();
    // println!("MSB: {}", msb);

    
}

fn get_bit_len(num: u64) -> u32 {
    for i in (1u64..64u64).rev() {
        if 0 < (num >> i) {
            return u32::try_from(i).unwrap()
        }
    }
    0
}

fn trim_bits(num: u64, bit_len: u32) -> u64 {
    num & (2u64.pow(bit_len)-1)
}

fn most_common_nth<'a>(binaries: &[&[u8]], index: usize) -> u8 {
    let half_len: u32 = &u32::try_from(binaries.iter().count()).unwrap() / 2;
    if half_len < binaries.iter().map(|bits|u32::from(bits.iter().cloned().nth(index).unwrap())).sum() {
        1
    } else {
        0
    }
}