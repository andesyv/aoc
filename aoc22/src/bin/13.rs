#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

fn strict_order_cmp<T: std::cmp::Eq + std::cmp::Ord>(l: &T, r: &T) -> Option<std::cmp::Ordering> {
    if l != r {
        Some(l.cmp(r))
    } else {
        None
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Packet::*;
        match (self, other) {
            (Value(i), Value(j)) => strict_order_cmp(i, j),
            (List(is), List(js)) => is
                .iter()
                .zip(js.iter())
                .map(|(l, r)| l.partial_cmp(r))
                .reduce(|acc, el| acc.or(el))
                .flatten()
                .or(strict_order_cmp(&is.len(), &js.len())),
            (List(_), j) => self.partial_cmp(&List(vec![j.clone()])),
            (i, List(_)) => List(vec![i.clone()]).partial_cmp(&other),
        }
    }
}

impl Ord for Packet {
    // Only used for stable sorting, so doesn't really matter that None -> Equal
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn parse_packet<T>(line: &mut T) -> Option<Packet>
where
    T: Iterator<Item = char>,
{
    let mut sub_packets = Vec::new();
    let mut val = String::new();
    while let Some(c) = line.next() {
        match c {
            ',' => {
                if let Ok(num) = val.parse() {
                    sub_packets.push(Packet::Value(num));
                }
                val.clear();
            }
            '0'..='9' => val.push(c),
            '[' => sub_packets.push(parse_packet(line)?),
            ']' => {
                if let Ok(num) = val.parse() {
                    sub_packets.push(Packet::Value(num));
                }
                return Some(Packet::List(sub_packets));
            }
            _ => (),
        }
    }
    // If the code reaches this point, we've already parsed the whole subtree:
    // (this could've definetively be done in a better way)
    sub_packets.first().cloned()
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .filter_map(|chunk| {
            chunk.split_once('\n').and_then(|(l, r)| {
                Some((parse_packet(&mut l.chars())?, parse_packet(&mut r.chars())?))
            })
        })
        .collect()
}

fn get_index_sums(packet_pairs: &Vec<(Packet, Packet)>) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| if l < r { Some(i + 1) } else { None })
        .sum()
}

fn get_decoder_key(packet_pairs: &Vec<(Packet, Packet)>) -> usize {
    let mut flattened_packets: Vec<Packet> = packet_pairs
        .iter()
        .flat_map(|(l, r)| vec![l.clone(), r.clone()].into_iter())
        .collect();
    let divider_packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Value(6)])]),
    ];
    flattened_packets.append(&mut divider_packets.clone());
    flattened_packets.sort();
    divider_packets
        .iter()
        .filter_map(|target| flattened_packets.iter().position(|packet| packet == target))
        .reduce(|acc, el| (acc + 1) * (el + 1))
        .unwrap()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/13.txt");
    let packet_pairs = parse(INPUT);
    println!(
        "The sum of indices of pairs in correct order is {}",
        get_index_sums(&packet_pairs)
    );

    println!("The decoder key is {}", get_decoder_key(&packet_pairs));
}

const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[test]
fn parse_test() {
    use Packet::*;
    let packets = parse("[[1],[2,3,4]]\n[[1],4]");
    let (fst, snd) = packets.first().unwrap();
    assert_eq!(
        *fst,
        List(vec![
            List(vec![Value(1)]),
            List(vec![Value(2), Value(3), Value(4)])
        ])
    );
    assert_eq!(*snd, List(vec![List(vec![Value(1)]), Value(4)]));
}

#[test]
fn example_1_test() {
    let packet_pairs = parse(EXAMPLE_INPUT);
    assert_eq!(packet_pairs.len(), 8);
    assert_eq!(get_index_sums(&packet_pairs), 13);
}

#[test]
fn example_2_test() {
    let packet_pairs = parse(EXAMPLE_INPUT);
    assert_eq!(get_decoder_key(&packet_pairs), 140);
}
