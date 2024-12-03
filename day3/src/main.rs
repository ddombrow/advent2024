use std::fs::File;
use std::io::{self, Read};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::char,
    combinator::{map, map_res, recognize},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

// Parser for numbers, converting them to i32
fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(take_while(is_digit), |s: &str| s.parse::<i32>())(input)
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(
        tag("mul("),
        separated_pair(parse_number, char(','), parse_number),
        char(')'),
    )(input)
}

fn parse_do(input: &str) -> IResult<&str, &str> {
    recognize(tag("do()"))(input)
}
fn parse_dont(input: &str) -> IResult<&str, &str> {
    recognize(tag("don't()"))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(parse_mul, |(a, b)| Instruction::Mul(a, b)),
        map(parse_do, |_| Instruction::Do),
        map(parse_dont, |_| Instruction::Dont),
    ))(input)
}

fn main() -> io::Result<()> {
    // Open the file in read-only mode (ignoring errors).
    let mut file = File::open("input.txt")?;

    let mut text = String::new();
    let _ = file.read_to_string(&mut text);

    //let text = String::from("do()asdasdon't()sdafsdfdo()dsfmul(5,6)sdfjsdkljf");

    let mut processing_enabled = true;
    let mut total_muls: i64 = 0;
    for i in 0..text.len() {
        let sub = &text[i..text.len()];
        match parse_instruction(sub) {
            Ok((_, instruction)) => match instruction {
                Instruction::Mul(first, second) => {
                    if processing_enabled {
                        //println!("Parsed `mul`: ({}, {})", first, second);
                        total_muls += (first * second) as i64;
                    }
                }
                Instruction::Do => {
                    //println!("Parsed `do`");
                    processing_enabled = true;
                }
                Instruction::Dont => {
                    //println!("Parsed `don't`");
                    processing_enabled = false;
                }
            },
            Err(_) => (),
        }
    }

    println!("total of do muls: {}", total_muls);
    Ok(())
}
