use advent_2021::*;
use bitvec::prelude::*;
use itertools::Itertools;

const SUM: u8 = 0;
const PRODUCT: u8 = 1;
const MINIMUM: u8 = 2;
const MAXIMUM: u8 = 3;
const LITERAL: u8 = 4;
const GT: u8 = 5;
const LT: u8 = 6;
const EQ: u8 = 7;

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    // 3 bits
    version: u8,
    // 3 bits
    type_id: u8,
    // n bits
    payload: Payload,
}

#[derive(Debug, Eq, PartialEq)]
enum Payload {
    Sum(Args),
    Product(Args),
    Minimum(Args),
    Maximum(Args),
    Literal(u64),
    GreaterThan(Args),
    LessThan(Args),
    EqualTo(Args),
}

impl Payload {
    fn new(type_id: u8, data: &mut BitVec) -> Option<Payload> {
        if type_id == LITERAL {
            let literal = Self::parse_literal(data)?;
            Some(Payload::Literal(literal))
        } else {
            let args = Self::parse_args(data)?;
            match type_id {
                SUM => Some(Payload::Sum(args)),
                PRODUCT => Some(Payload::Product(args)),
                MINIMUM => Some(Payload::Minimum(args)),
                MAXIMUM => Some(Payload::Maximum(args)),
                GT => Some(Payload::GreaterThan(args)),
                LT => Some(Payload::LessThan(args)),
                EQ => Some(Payload::EqualTo(args)),
                _ => None,
            }
        }
    }

    fn parse_literal(data: &mut BitVec) -> Option<u64> {
        let mut is_more = decode_front(data, 1)?;
        let mut result = decode_front(data, 4)?;
        while is_more == 0b1 {
            is_more = decode_front(data, 1)?;
            result <<= 4;
            result |= decode_front(data, 4)?;
        }
        Some(result)
    }

    fn parse_args(data: &mut BitVec) -> Option<Args> {
        let length = if decode_front(data, 1)? == 0b0 {
            Length::BitCount(decode_front(data, 15)? as usize)
        } else {
            Length::SubPacketCount(decode_front(data, 11)? as usize)
        };

        let result = match length {
            Length::BitCount(bit_count) => {
                let mut subpacket_data = data.drain(0..bit_count).collect();
                let subpacket_decoder = Decoder::new(&mut subpacket_data);
                Args {
                    length,
                    subpackets: subpacket_decoder.collect_vec(),
                }
            }
            Length::SubPacketCount(subpacket_count) => {
                let subpacket_decoder = Decoder::new(data);
                Args {
                    length,
                    subpackets: subpacket_decoder.take(subpacket_count).collect_vec(),
                }
            }
        };
        Some(result)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Args {
    length: Length,
    subpackets: Vec<Packet>,
}

#[derive(Debug, Eq, PartialEq)]
enum Length {
    BitCount(usize),
    SubPacketCount(usize),
}

struct Decoder<'a> {
    data: &'a mut BitVec,
}

fn parse_string(string: &str) -> BitVec {
    let mut data = BitVec::new();
    for digit in string.chars().map(|c| c.to_digit(16).unwrap()) {
        data.push((digit >> 3) & 0b1 == 0b1);
        data.push((digit >> 2) & 0b1 == 0b1);
        data.push((digit >> 1) & 0b1 == 0b1);
        data.push((digit >> 0) & 0b1 == 0b1);
    }
    data
}
impl<'a> Decoder<'a> {
    fn new(data: &'a mut BitVec) -> Self {
        Self { data }
    }
}

impl Iterator for Decoder<'_> {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        let version = decode_front(&mut self.data, 3)? as u8;
        let type_id = decode_front(&mut self.data, 3)? as u8;
        let result = Some(Packet {
            version,
            type_id,
            payload: Payload::new(type_id, &mut self.data)?,
        });
        result
    }
}

fn decode_front(data: &mut BitVec, bits: usize) -> Option<u64> {
    if data.len() < bits {
        None
    } else {
        // Rust won't catch this itself!
        assert!(bits < 64);
        let mut result = 0;
        for bit in data.drain(0..bits) {
            result = result << 1;
            result = result | if bit { 0b1 } else { 0b0 }
        }
        Some(result)
    }
}

#[test]
fn example_1() {
    let mut data = parse_string("D2FE28");
    let mut decoder = Decoder::new(&mut data);
    let packet = decoder.next().unwrap();
    assert_eq!(6, packet.version);
    assert_eq!(4, packet.type_id);
    assert_eq!(Payload::Literal(2021), packet.payload);
    assert_eq!(None, decoder.next());
}

#[test]
fn example_2() {
    let mut data = parse_string("38006F45291200");
    let mut decoder = Decoder::new(&mut data);
    let packet = decoder.next().unwrap();
    assert_eq!(1, packet.version);
    assert_eq!(6, packet.type_id);
    assert_eq!(
        Payload::LessThan(Args {
            length: Length::BitCount(27),
            subpackets: vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    payload: Payload::Literal(10)
                },
                Packet {
                    version: 2,
                    type_id: 4,
                    payload: Payload::Literal(20)
                }
            ]
        }),
        packet.payload
    );
    assert_eq!(None, decoder.next());
}

#[test]
fn example_3() {
    let mut data = parse_string("EE00D40C823060");
    let mut decoder = Decoder::new(&mut data);
    let packet = decoder.next().unwrap();
    assert_eq!(7, packet.version);
    assert_eq!(3, packet.type_id);
    assert_eq!(
        Payload::Maximum(Args {
            length: Length::SubPacketCount(3),
            subpackets: vec![
                Packet {
                    version: 2,
                    type_id: 4,
                    payload: Payload::Literal(1)
                },
                Packet {
                    version: 4,
                    type_id: 4,
                    payload: Payload::Literal(2)
                },
                Packet {
                    version: 1,
                    type_id: 4,
                    payload: Payload::Literal(3)
                }
            ]
        }),
        packet.payload
    );
    assert_eq!(None, decoder.next());
}

fn version_sum(decoder: Decoder) -> u32 {
    let mut sum = 0;
    for packet in decoder {
        sum += version_sum_packet(packet);
    }
    sum
}

fn version_sum_packet(packet: Packet) -> u32 {
    let mut sum: u32 = 0;
    sum += packet.version as u32;
    sum += match packet.payload {
        Payload::Literal(_) => 0,
        Payload::Sum(args)
        | Payload::Product(args)
        | Payload::Minimum(args)
        | Payload::Maximum(args)
        | Payload::GreaterThan(args)
        | Payload::LessThan(args)
        | Payload::EqualTo(args) => args
            .subpackets
            .into_iter()
            .fold(0, |acc, packet| acc + version_sum_packet(packet)),
    };
    sum
}

#[test]
fn example_4() {
    let mut data = parse_string("8A004A801A8002F478");
    let decoder = Decoder::new(&mut data);
    let sum = version_sum(decoder);
    assert_eq!(16, sum);
}

#[test]
fn example_5() {
    let mut data = parse_string("620080001611562C8802118E34");
    let decoder = Decoder::new(&mut data);
    let sum = version_sum(decoder);
    assert_eq!(12, sum);
}

#[test]
fn example_6() {
    let mut data = parse_string("C0015000016115A2E0802F182340");
    let decoder = Decoder::new(&mut data);
    let sum = version_sum(decoder);
    assert_eq!(23, sum);
}

#[test]
fn example_7() {
    let mut data = parse_string("A0016C880162017C3686B18A3D4780");
    let decoder = Decoder::new(&mut data);
    let sum = version_sum(decoder);
    assert_eq!(31, sum);
}

fn process(packet: Packet) -> u64 {
    match packet.payload {
        Payload::Literal(n) => n as u64,
        Payload::Sum(args) => args
            .subpackets
            .into_iter()
            .fold(0, |acc, packet| acc + process(packet)),
        Payload::Product(args) => args
            .subpackets
            .into_iter()
            .fold(1, |acc, packet| acc * process(packet)),
        Payload::Minimum(args) => args
            .subpackets
            .into_iter()
            .map(|packet| process(packet))
            .min()
            .unwrap(),
        Payload::Maximum(args) => args
            .subpackets
            .into_iter()
            .map(|packet| process(packet))
            .max()
            .unwrap(),
        Payload::GreaterThan(args) => {
            assert_eq!(args.subpackets.len(), 2);
            let mut values = args.subpackets.into_iter();
            (process(values.next().unwrap()) > process(values.next().unwrap())) as u64
        }
        Payload::LessThan(args) => {
            assert_eq!(args.subpackets.len(), 2);
            let mut values = args.subpackets.into_iter();
            (process(values.next().unwrap()) < process(values.next().unwrap())) as u64
        }
        Payload::EqualTo(args) => {
            assert_eq!(args.subpackets.len(), 2);
            let mut values = args.subpackets.into_iter();
            (process(values.next().unwrap()) == process(values.next().unwrap())) as u64
        }
    }
}

#[test]
fn example_8() {
    let mut data = parse_string("C200B40A82");
    let mut decoder = Decoder::new(&mut data);
    let result = process(decoder.next().unwrap());
    assert_eq!(3, result);
}

#[test]
fn example_9() {
    let mut data = parse_string("04005AC33890");
    let mut decoder = Decoder::new(&mut data);
    let result = process(decoder.next().unwrap());
    assert_eq!(54, result);
}

#[test]
fn example_10() {
    let mut data = parse_string("880086C3E88112");
    let mut decoder = Decoder::new(&mut data);
    let result = process(decoder.next().unwrap());
    assert_eq!(7, result);
}

#[test]
fn example_11() {
    let mut data = parse_string("CE00C43D881120");
    let mut decoder = Decoder::new(&mut data);
    let result = process(decoder.next().unwrap());
    assert_eq!(9, result);
}

#[test]
fn example_12() {
    let mut data = parse_string("D8005AC2A8F0");
    let mut decoder = Decoder::new(&mut data);
    let result = process(decoder.next().unwrap());
    assert_eq!(1, result);
}

#[test]
fn example_13() {
    let mut data = parse_string("F600BC2D8F");
    let mut decoder = Decoder::new(&mut data);
    let result = process(decoder.next().unwrap());
    assert_eq!(0, result);
}

#[test]
fn example_14() {
    let mut data = parse_string("9C005AC2F8F0");
    let mut decoder = Decoder::new(&mut data);
    let result = process(decoder.next().unwrap());
    assert_eq!(0, result);
}

#[test]
fn example_15() {
    let mut data = parse_string("9C0141080250320F1802104A08");
    let mut decoder = Decoder::new(&mut data);
    let result = process(decoder.next().unwrap());
    assert_eq!(1, result);
}

#[test]
fn example_16() {
    let mut data =
        parse_string("3600888023024c01150044c0118330a440118330e44011833085c0118522008c29870");
    let mut decoder = Decoder::new(&mut data);
    let result = process(decoder.next().unwrap());
    assert_eq!(1, result);
}

fn main() {
    let input = input::lines("./input/day_16.txt").unwrap();
    assert_eq!(input.len(), 1);
    // part 1:
    {
        let mut data = parse_string(&input[0]);
        let decoder = Decoder::new(&mut data);

        let sum = version_sum(decoder);
        println!("#1:{}", sum);
    }

    // part 2:
    {
        let mut data = parse_string(&input[0]);
        let mut decoder = Decoder::new(&mut data);

        let result = process(decoder.next().unwrap());
        println!("#2:{}", result);
        assert_eq!(None, decoder.next());
    }
}
