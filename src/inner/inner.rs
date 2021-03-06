use crate::networking::buffer::Buffer;
use std::str::Chars;

#[path = "./protocol/protocol.rs"]
pub mod protocol;
#[path = "./data/server.rs"]
pub mod server;

const CHAR_MAP: [i32; 26] = [
    25, 21, 19, 10, 8, 11, 12, 13, 22, 15, 16, 6, 24, 23, 18, 7, 0, 3, 9, 4, 14, 20, 1, 2, 5, 17,
];

#[allow(overflowing_literals)]
pub fn code_to_int(mut game_code: String) -> i32 {
    // let char_set: Vec<char> = "QWXRTYLPESDFGHUJKZOCVBINMA".chars().collect();
    game_code = game_code.to_uppercase();

    if game_code.chars().any(|c| !c.is_alphabetic()) {
        println!("Returning");
        return 0;
    };

    if game_code.len() == 4 {
        let mut buffer = Buffer::new(game_code.into_bytes());
        println!("Four, bytes {:?}", buffer.array());
        return buffer.read_int_32();
    };

    if game_code.len() != 6 {
        return 0;
    }

    let arr: Vec<char> = game_code.chars().collect();

    println!("First Char: {:?}", arr[0] as u8);
    let first = CHAR_MAP[(arr[0] as u8 - 65) as usize];
    let second = CHAR_MAP[(arr[1] as u8 - 65) as usize];
    let third = CHAR_MAP[(arr[2] as u8 - 65) as usize];
    let fourth = CHAR_MAP[(arr[3] as u8 - 65) as usize];
    let fifth = CHAR_MAP[(arr[4] as u8 - 65) as usize];
    let sixth = CHAR_MAP[(arr[5] as u8 - 65) as usize];

    println!(
        "{} to int: {},{},{},{},{},{}",
        game_code, first, second, third, fourth, fifth, sixth
    );

    let first_two = (first + 26 * second) & 0x3FF;
    let last_four = third + 26 * (fourth + 26 * (fifth + 26 * sixth));

    let value = first_two | ((last_four << 10) & 0x3FFFFC00) | 0x80000000;

    return value;
}

pub fn int_to_code(int: i32) -> String {
    let char_set: Vec<char> = "QWXRTYLPESDFGHUJKZOCVBINMA".chars().collect();
    if int < 0 {
        let first_two = int & 0x3ff;
        let mut last_four = ((int >> 10) & 0xfffff);

        let one = char_set[(first_two % 26) as usize].to_string();
        let two = char_set[(first_two / 26) as usize].to_string();
        let three = char_set[(last_four % 26) as usize].to_string();
        last_four /= 26;
        let four = char_set[(last_four % 26) as usize].to_string();
        last_four /= 26;
        let five = char_set[(last_four % 26) as usize].to_string();
        let six = char_set[(last_four / 26 % 26) as usize].to_string();

        return format!("{}{}{}{}{}{}", one, two, three, four, five, six);
    }

    return "".to_string();
}
