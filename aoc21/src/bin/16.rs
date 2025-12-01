#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    data: PacketData,
}

#[derive(Debug)]
enum PacketData {
    Literal(u128),
    Operator(Vec<Packet>),
}

struct Progress {
    total: usize,
    done: usize,
}

fn main() {
    // const INPUT: &str = "A0016C880162017C3686B18A3D4780";
    const INPUT: &str = include_str!("../inputs/16.txt");
    let binary = &hex_to_binary(INPUT);
    let mut progress_tracker = Progress {
        total: binary.len(),
        done: 0,
    };
    let (packet, _) = parse_packet(binary, &mut progress_tracker).unwrap();
    println!(
        "{} out of {} bits were processed",
        progress_tracker.done, progress_tracker.total
    );
    println!("Sum of versions: {}", sum_versions(&packet));

    println!("Evaluation of packet gives: {}", evaluate(&packet));
}

fn hex_to_binary(pattern: &str) -> String {
    let mut binary = String::with_capacity(pattern.len() * 4);
    for hex in pattern.chars() {
        if let Ok(dec) = u8::from_str_radix(&hex.to_string(), 16) {
            binary.push_str(&format!("{:0>4b}", dec));
        }
    }
    binary
}

fn parse_packet(binary: &str, progress_tracker: &mut Progress) -> Option<(Packet, usize)> {
    let v = u8::from_str_radix(&binary[0..3], 2).ok()?;
    let t = u8::from_str_radix(&binary[3..6], 2).ok()?;

    progress_tracker.done += 6;

    if t == 4 {
        let mut i = 6;
        let mut num = String::new();
        loop {
            num.push_str(&binary[(i + 1)..(i + 5)]);
            i += 5;
            if let Some('0') = binary.chars().nth(i - 5) {
                break;
            }
        }
        progress_tracker.done += i - 6;
        Some((
            Packet {
                version: v,
                type_id: t,
                data: PacketData::Literal(
                    u128::from_str_radix(&num[..], 2).expect("Invalid literal!"),
                ),
            },
            i,
        ))
    } else {
        let mut subpackets;
        let mut i;
        if let Some('0') = binary.chars().nth(6) {
            progress_tracker.done += 16;
            let end = 22 + usize::from_str_radix(&binary[7..22], 2).ok()?;
            i = 22;
            subpackets = Vec::new();
            // The smallest packet possible is a literal value with 1 number (10 bits)
            while 9 < end - i {
                if let Some((pack, j)) = parse_packet(&binary[i..end], progress_tracker) {
                    subpackets.push(pack);
                    i += j;
                }
            }
        } else {
            progress_tracker.done += 12;
            let packet_count = usize::from_str_radix(&binary[7..18], 2).ok()?;
            subpackets = Vec::with_capacity(packet_count);
            i = 18;
            for _ in 0..packet_count {
                if let Some((pack, j)) = parse_packet(&binary[i..], progress_tracker) {
                    subpackets.push(pack);
                    i += j;
                }
            }
        }
        Some((
            Packet {
                version: v,
                type_id: t,
                data: PacketData::Operator(subpackets),
            },
            i,
        ))
    }
}

fn sum_versions(packet: &Packet) -> u64 {
    u64::from(packet.version)
        + match &packet.data {
            PacketData::Operator(subpackets) => subpackets.iter().map(sum_versions).sum(),
            _ => 0,
        }
}

fn combine_subpackets(mut it: impl Iterator<Item = u128>, type_id: u8) -> u128 {
    match type_id {
        0 => it.sum(),
        1 => it.product(),
        2 => it.min().unwrap(),
        3 => it.max().unwrap(),
        5 | 6 | 7 => {
            if let (Some(a), Some(b)) = (it.next(), it.next()) {
                match type_id {
                    5 => if a > b { 1 } else { 0 },
                    6 => if a < b { 1 } else { 0 },
                    7 => if a == b { 1 } else { 0 },
                    _ => unreachable!()
                }
            } else { panic!("Failed to read 2 values from iterator.") }
        },
        _ => unreachable!(),
    }
}

fn evaluate(packet: &Packet) -> u128 {
    match &packet.data {
        PacketData::Literal(v) => *v,
        PacketData::Operator(subpackets) => {
            combine_subpackets(subpackets.iter().map(evaluate), packet.type_id)
        }
    }
}

#[test]
fn test_hex_reader1() {
    assert_eq!(&hex_to_binary("D2FE28"), "110100101111111000101000");
}

#[test]
fn test_hex_reader2() {
    assert_eq!(
        &hex_to_binary("38006F45291200"),
        "00111000000000000110111101000101001010010001001000000000"
    );
}

#[test]
fn test_hex_reader3() {
    assert_eq!(
        &hex_to_binary("EE00D40C823060"),
        "11101110000000001101010000001100100000100011000001100000"
    );
}

#[test]
fn test_parse1() {
    let binary = hex_to_binary("D2FE28");
    let mut progress_tracker = Progress {
        total: binary.len(),
        done: 0,
    };
    assert!(
        match parse_packet(&binary, &mut progress_tracker).unwrap().0 {
            Packet {
                version: 6,
                type_id: 4,
                data: PacketData::Literal(2021),
            } => true,
            _ => false,
        }
    );
}

#[test]
fn test_parse2() {
    let binary = hex_to_binary("38006F45291200");
    let mut progress_tracker = Progress {
        total: binary.len(),
        done: 0,
    };
    assert!(
        match parse_packet(&binary, &mut progress_tracker).unwrap().0 {
            Packet {
                version: 1,
                type_id: 6,
                data: PacketData::Operator(v),
            } if v.len() == 2 => true,
            _ => false,
        }
    );
}

#[test]
fn test_parse3() {
    let binary = hex_to_binary("EE00D40C823060");
    let mut progress_tracker = Progress {
        total: binary.len(),
        done: 0,
    };
    assert!(
        match parse_packet(&binary, &mut progress_tracker).unwrap().0 {
            Packet {
                version: 7,
                type_id: 3,
                data: PacketData::Operator(v),
            } if v.len() == 3 => true,
            _ => false,
        }
    );
}

fn _test_version_sum(input: &str) -> u64 {
    let binary = hex_to_binary(input);
    let mut progress_tracker = Progress {
        total: binary.len(),
        done: 0,
    };
    let (packet, _) = parse_packet(&binary, &mut progress_tracker).unwrap();
    sum_versions(&packet)
}

#[test]
fn test_version_sum1() {
    assert_eq!(_test_version_sum("8A004A801A8002F478"), 16);
}

#[test]
fn test_version_sum2() {
    assert_eq!(_test_version_sum("620080001611562C8802118E34"), 12);
}

#[test]
fn test_version_sum3() {
    assert_eq!(_test_version_sum("C0015000016115A2E0802F182340"), 23);
}

#[test]
fn test_version_sum4() {
    assert_eq!(_test_version_sum("A0016C880162017C3686B18A3D4780"), 31);
}

fn _test_evaluate(input: &str) -> u128 {
    let binary = hex_to_binary(input);
    let mut progress_tracker = Progress {
        total: binary.len(),
        done: 0,
    };
    let (packet, _) = parse_packet(&binary, &mut progress_tracker).unwrap();
    evaluate(&packet)
}

#[test]
fn test_evaluate1() {
    assert_eq!(_test_evaluate("C200B40A82"), 3);
}

#[test]
fn test_evaluate2() {
    assert_eq!(_test_evaluate("04005AC33890"), 54);
}

#[test]
fn test_evaluate3() {
    assert_eq!(_test_evaluate("880086C3E88112"), 7);
}

#[test]
fn test_evaluate4() {
    assert_eq!(_test_evaluate("CE00C43D881120"), 9);
}

#[test]
fn test_evaluate5() {
    assert_eq!(_test_evaluate("D8005AC2A8F0"), 1);
}

#[test]
fn test_evaluate6() {
    assert_eq!(_test_evaluate("F600BC2D8F"), 0);
}

#[test]
fn test_evaluate7() {
    assert_eq!(_test_evaluate("9C005AC2F8F0"), 0);
}

#[test]
fn test_evaluate8() {
    assert_eq!(_test_evaluate("9C0141080250320F1802104A08"), 1);
}