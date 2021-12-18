use regex::Regex;

pub struct Instruction {}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<u8> {
    // 0 = 0000
    // 1 = 0001
    // 2 = 0010
    // 3 = 0011
    // 4 = 0100
    // 5 = 0101
    // 6 = 0110
    // 7 = 0111
    // 8 = 1000
    // 9 = 1001
    // A = 1010
    // B = 1011
    // C = 1100
    // D = 1101
    // E = 1110
    // F = 1111

    input
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Invalid character: {}", c),
        })
        .collect::<Vec<&str>>()
        .join("")
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Vec<u8>) -> i32 {
    parse_packet(input.to_vec()).version_num_total as i32
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Vec<u8>) -> i128 {
    parse_packet(input.to_vec()).data as i128
}

pub fn convert_to_dec(input: Vec<u8>) -> u32 {
    let mut result = 0;
    for (i, c) in input.iter().rev().enumerate() {
        result += *c as u32 * 2u32.pow(i as u32);
    }
    result
}

#[derive(Debug)]
pub struct Packet {
    pub version: usize,
    pub type_id: usize,
    pub index: usize,
    pub data: usize,
    pub version_num_total: usize,
}

pub fn parse_packet(input: Vec<u8>) -> Packet {
    // First three bytes are version
    // Second three bytes are type ID

    let mut packet = Packet {
        index: 0,
        data: 0,
        version: 0,
        type_id: 0,
        version_num_total: 0,
    };

    let version = convert_to_dec(input[0..3].to_vec());
    let type_id = convert_to_dec(input[3..6].to_vec());

    packet.version = version as usize;
    packet.type_id = type_id as usize;

    packet.version_num_total = packet.version;

    packet.index = 6;

    // dbg!(&packet);

    match type_id {
        // Packets with type ID 0 are sum packets - their value is the sum of
        // the values of their sub-packets. If they only have a single
        // sub-packet, their value is the value of the sub-packet.
        //
        // Packets with type ID 1 are product packets - their value is the
        // result of multiplying together the values of their sub-packets. If
        // they only have a single sub-packet, their value is the value of the
        // sub-packet.
        //
        // Packets with type ID 2 are minimum packets - their value is the
        // minimum of the values of their sub-packets.
        //
        // Packets with type ID 3 are maximum packets - their value is the
        // maximum of the values of their sub-packets.
        //
        // Packets with type ID 5 are greater than packets - their value is 1 if
        // the value of the first sub-packet is greater than the value of the
        // second sub-packet; otherwise, their value is 0. These packets always
        // have exactly two sub-packets.
        //
        // Packets with type ID 6 are less than packets - their value is 1 if
        // the value of the first sub-packet is less than the value of the
        // second sub-packet; otherwise, their value is 0. These packets always
        // have exactly two sub-packets.
        //
        // Packets with type ID 7 are equal to packets - their value is 1 if the
        // value of the first sub-packet is equal to the value of the second
        // sub-packet; otherwise, their value is 0. These packets always have
        // exactly two sub-packets.
        //
        4 => {
            let mut value = convert_to_dec(input[packet.index..packet.index + 5].to_vec());
            packet.index += 5;
            while value >= 16 {
                packet.data += value as usize;
                value = convert_to_dec(input[packet.index..packet.index + 5].to_vec());
                packet.index += 5;
            }
            packet.data += value as usize;
        }

        _ => {
            let length_type_id = convert_to_dec(input[packet.index..packet.index + 1].to_vec());
            packet.index += 1;
            let mut results: Vec<usize> = Vec::new();

            match length_type_id {
                // If the length type ID is 0, then the next 15 bits are a number that
                // represents the total length in bits of the sub-packets contained by this
                // packet.
                0 => {
                    let length =
                        convert_to_dec(input[packet.index..packet.index + 15].to_vec()) as usize;
                    packet.index += 15;

                    let goal_len = packet.index + length;

                    while packet.index < goal_len {
                        let parsed_packet = parse_packet(input[packet.index..goal_len].to_vec());

                        packet.index += parsed_packet.index;
                        packet.version_num_total += parsed_packet.version_num_total;
                        results.push(parsed_packet.data);
                    }
                }
                // If the length type ID is 1, then the next 11 bits are a number
                // that represents the number of sub-packets immediately contained by this
                // packet.
                1 => {
                    let num_subpackets =
                        convert_to_dec(input[packet.index..packet.index + 11].to_vec());
                    packet.index += 11;

                    for _ in 0..num_subpackets {
                        let parsed_packet = parse_packet(input[packet.index..].to_vec());

                        packet.index += parsed_packet.index;
                        packet.version_num_total += parsed_packet.version_num_total;

                        results.push(parsed_packet.data);
                    }
                }
                _ => panic!("Invalid length type ID"),
            }

            match type_id {
                0 => {
                    packet.data = results.iter().sum();
                }
                1 => {
                    packet.data = results.iter().product();
                }
                2 => {
                    packet.data = *results.iter().min().unwrap();
                }
                3 => {
                    packet.data = *results.iter().max().unwrap();
                }
                5 => {
                    packet.data = if results[0] > results[1] { 1 } else { 0 };
                }
                6 => {
                    packet.data = if results[0] < results[1] { 1 } else { 0 };
                }
                7 => {
                    packet.data = if results[0] == results[1] { 1 } else { 0 };
                }
                _ => panic!("Invalid type ID"),
            }
        }

    };

    packet
}
