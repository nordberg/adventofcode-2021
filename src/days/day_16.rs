use std::collections::HashMap;

pub fn solve_day_16(input: &str) {
    let mapping = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);

    let mut binary_string = String::new();

    for char in input.chars() {
        binary_string.push_str(mapping.get(&char).unwrap());
    }

    extract_sub_packets(&binary_string);
}

fn extract_sub_packets(packets: &str) -> Vec<String> {
    let mut result = vec![];

    let packet_version = i32::from_str_radix(&packets[0..3], 2).unwrap();
    let packet_type_id = i32::from_str_radix(&packets[3..6], 2).unwrap();
    match packet_type_id {
        4 => {
            find_literal_value(&packets[6..]);
        }
        _ => {
            todo!();
        }
    }
    println!("Packet version: {}", packet_version);
    println!("Packet type id: {}", packet_type_id);

    result
}

fn find_literal_value(packet: &str) -> i32 {
    let mut result = String::new();
    for i in (0..packet.len()).step_by(5) {
        result.push_str(&packet[i + 1..i + 5]);
        if packet[i..].starts_with('0') {
            break;
        }
    }

    println!("Literal value: {}", result);
    5
}
