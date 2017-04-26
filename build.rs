extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "static ALPHABET_MAP: phf::Map<char, u8> = ").unwrap();
    phf_codegen::Map::new()
        .entry('1', "0u8").entry('2', "1u8").entry('3', "2u8")
        .entry('4', "3u8").entry('5', "4u8").entry('6', "5u8")
        .entry('7', "6u8").entry('8', "7u8").entry('9', "8u8")
        .entry('A', "9u8").entry('B', "10u8").entry('C', "11u8")
        .entry('D', "12u8").entry('E', "13u8").entry('F', "14u8")
        .entry('G', "15u8").entry('H', "16u8").entry('J', "17u8")
        .entry('K', "18u8").entry('L', "19u8").entry('M', "20u8")
        .entry('N', "21u8").entry('P', "22u8").entry('Q', "23u8")
        .entry('R', "24u8").entry('S', "25u8").entry('T', "26u8")
        .entry('U', "27u8").entry('V', "28u8").entry('W', "29u8")
        .entry('X', "30u8").entry('Y', "31u8").entry('Z', "32u8")
        .entry('a', "33u8").entry('b', "34u8").entry('c', "35u8")
        .entry('d', "36u8").entry('e', "37u8").entry('f', "38u8")
        .entry('g', "39u8").entry('h', "40u8").entry('i', "41u8")
        .entry('j', "42u8").entry('k', "43u8").entry('m', "44u8")
        .entry('n', "45u8").entry('o', "46u8").entry('p', "47u8")
        .entry('q', "48u8").entry('r', "49u8").entry('s', "50u8")
        .entry('t', "51u8").entry('u', "52u8").entry('v', "53u8")
        .entry('w', "54u8").entry('x', "55u8").entry('y', "56u8")
        .entry('z', "57u8")
        .build(&mut file)
        .unwrap();
    write!(&mut file, ";\n").unwrap();
}
