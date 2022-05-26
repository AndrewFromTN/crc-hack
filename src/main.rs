use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut crc_table = Vec::<i32>::new();
    if let Ok(lines) = read_lines("./input2.txt") {
        for line in lines {
            if let Ok(num_str) = line {
                crc_table.push(num_str.parse::<i32>().expect("That's not a valid i32."));
            }
        }
    }

    // Code for problem 6
    /*let _p_6 = (0x00..0xFFFFFFFF as u32)
        .into_par_iter()
        .for_each(|val: u32| {
            let mut buffer = "a".repeat(252).as_bytes().to_vec();
            buffer.append(&mut val.to_be_bytes().to_vec());
            let data_blk_size = buffer.len();
            let mut crc_accum: i32 = -1;

            for i in 0..data_blk_size {
                let j = ((crc_accum >> 24) as u32 ^ buffer[i as usize] as u32) & 0xFF;
                crc_accum = (crc_accum << 8) ^ crc_table[j as usize];
            }

            crc_accum = !crc_accum;
            if crc_accum == 0x4920cbc3 {
                println!(
                    "Buffer Bytes: {}, {}, {}, {}",
                    buffer[252], buffer[253], buffer[254], buffer[255]
                );
                println!("Decimal: {}", val);
                println!("Hex: {:X}", val);
                std::process::exit(0);
            }
        });*/

    // Code for problem 7
    let _p_7 = (0x00..0xFFFFFFFF as u32)
        .into_par_iter()
        .for_each(|val: u32| {
            let mut buffer = b"\xF7\xFF".to_vec();
            buffer.append(&mut "a".repeat(250).as_bytes().to_vec());
            buffer.append(&mut val.to_be_bytes().to_vec());
            let data_blk_size = buffer.len();
            let mut return_val: i32 = -1;

            assert!(data_blk_size == 256);
            for i in 0..data_blk_size {
                let j = ((return_val >> 24) as u32 ^ buffer[i as usize] as u32) & 0xFF;
                return_val = (return_val << 8) ^ crc_table[j as usize];
            }

            return_val = !return_val;
            
            if return_val == 0x66dc1b0e {
                println!("Return Value: {:X}", return_val);
                println!(
                    "Buffer Bytes: {}, {}, {}, {}",
                    buffer[252], buffer[253], buffer[254], buffer[255]
                );
                println!("Decimal: {}", val);
                println!("Hex: {:X}", val);
                std::process::exit(0);
            }
        });

    // Code for problem 8
    /* let _p_8 = (0x00..0xFFFFFFFF as u32)
        .into_par_iter()
        .for_each(|val: u32| {
            let mut buffer = b"a".repeat(246).to_vec();
            buffer.append(&mut val.to_be_bytes().to_vec());
            buffer.append(&mut b"aaaaaa".to_vec());
            let data_blk_size = buffer.len();
            let mut return_val: i32 = -1;

            assert!(data_blk_size == 256);
            for i in 0..(data_blk_size-6) {
                let j = ((return_val >> 24) as u32 ^ buffer[i + 3] as u32) & 0xFF;
                return_val = (return_val << 8) ^ crc_table[j as usize];
            }

            return_val = !return_val;
            
            if return_val == 0xa9d240a {
                println!("Return Value: {:X}", return_val);
                println!(
                    "Buffer Bytes: {}, {}, {}, {}",
                    buffer[246], buffer[247], buffer[248], buffer[249]
                );
                println!("Decimal: {}", val);
                println!("Hex: {:X}", val);
                std::process::exit(0);
            }
        }); */
}
