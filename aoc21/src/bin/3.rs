fn main() {
    const INPUT: &str = include_str!("../inputs/3.txt");
    // const INPUT: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    let bin_len: u32 = INPUT.lines().next().unwrap().chars().count().try_into().unwrap();
    let vals: Vec<u128> = INPUT.lines().map(|s|isize::from_str_radix(s, 2).unwrap().try_into().unwrap()).collect();
    let msb: u128 = (0..bin_len).map(|i|most_common_nth::<u128>(&vals, i) << i).sum();
    let lsb = (msb ^ (2u128.pow(bin_len)-1)) & (2u128.pow(bin_len)-1);
    println!("bin_len is {}, msb is {}, and lsb is {}. Product is: {}", bin_len, msb, lsb, msb * lsb);

    let mut vals2 = vals.clone();
    for i in (0..bin_len).rev() {
        let common = most_common_nth::<u128>(&vals2, i) << i;
        vals2 = vals2.into_iter().filter(|num|(num & (1 << i)) == common).collect();
        if vals2.is_empty() {
            panic!("This wasn't supposed to happen.");
        } else if vals2.len() == 1 {
            break;
        }
    }

    let oxygen_rating = vals2[0];

    let mut vals2 = vals.clone();
    for i in (0..bin_len).rev() {
        let common = most_common_nth::<u128>(&vals2, i) << i;
        vals2 = vals2.into_iter().filter(|num|(num & (1 << i)) != common).collect();
        if vals2.is_empty() {
            panic!("This wasn't supposed to happen.");
        } else if vals2.len() == 1 {
            break;
        }
    }

    let scrubber_rating = vals2[0];

    println!("Oxygen generating rating: {}, C02 scrubber rating: {}, life support rating: {}", oxygen_rating, scrubber_rating, oxygen_rating * scrubber_rating);
}

fn most_common_nth<I: std::convert::From<u8>>(nums: &[u128], n: u32) -> I {
    let count: u128 = nums.iter().count().try_into().unwrap();
    if 2u128.pow(n) * count / 2 <= nums.iter().map(|i|i & 2u128.pow(n)).sum() {
        1u8.try_into().unwrap()
    } else {
        0u8.try_into().unwrap()
    }
}

/*
fn significant_bit_pos(num: u64) -> u32 {
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
*/