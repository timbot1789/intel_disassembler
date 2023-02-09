use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("searching for file {}", file_path);

    let contents = fs::read(file_path)
        .expect("Should have been able to read the file");
    
    // println!("{}", format_hex(&contents));

    let mut pc: usize = 0;
    let mut result = String::from("");
    while pc < contents.len(){
        result.push_str(&disassemble_8080(&contents, &mut pc));
    }
    println!("{}", result);
}

// fn format_hex(contents: &Vec<u8>) -> String {
//     let mut result = String::from("");
    
//     for (i, byte) in contents.iter().enumerate(){
//         if i % 16 == 0 && i != 0 {
//             result.push('\n')
//         }
//         result.push_str(&format!("{:x}", byte));
//     }

//     return result;
// }

fn disassemble_8080(buffer: &Vec<u8>, pc: &mut usize) -> String{
    let reg_list = HashMap::from([
        (0, "B"),
        (1, "C"),
        (2, "D"),
        (3, "E"),
        (4, "H"),
        (5, "L"),
        (6, "M"),
        (7, "A")
    ]);

    return match buffer[*pc] {
        0x00 => (|| {
                return simple_render("NOP\n", pc);
            } )(),
        0x01 | 0x11 | 0x21 | 0x31 => (|| {
            let registers = HashMap::from([
                (0x01, "B"),
                (0x11, "D"),
                (0x21, "H"),
                (0x31, "SP"),
            ]);
            let reg = registers[&buffer[*pc]];
            let res = format!("LXI {},{:x}{:x}\n", reg, buffer[*pc + 1], buffer[*pc + 2]);
            *pc += 3;
            return res;
        })(),
        0x02 | 0x12 => (|| {
            let reg = if buffer[*pc] == 0x02 { "B" } else { "D" };
            *pc += 1;
            return format!("STAX {}\n", reg);
        } )(),
        0x03 | 0x13 | 0x23 | 0x33 => (|| {
            let registers = HashMap::from([
                (0x03, "B"),
                (0x13, "D"),
                (0x23, "H"),
                (0x33, "SP"),
            ]);
            let reg = registers[&buffer[*pc]];
            let res = format!("INX {}\n", reg);
            *pc += 1;
            return res;
        })(),
        0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x34 | 0x3c => (|| {
            let registers = HashMap::from([
                (0x04, "B"),
                (0x0c, "C"),
                (0x14, "D"),
                (0x1c, "E"),
                (0x24, "H"),
                (0x2c, "L"),
                (0x34, "M"),
                (0x3c, "A")
            ]);
            let reg = registers[&buffer[*pc]];
            let res = format!("INR {}\n", reg);
            *pc += 1;
            return res;
        })(),
        0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 | 0x3d => (|| {
            let registers = HashMap::from([
                (0x05, "B"),
                (0x0d, "C"),
                (0x15, "D"),
                (0x1d, "E"),
                (0x25, "H"),
                (0x2d, "L"),
                (0x35, "M"),
                (0x3d, "A")
            ]);
            let reg = registers[&buffer[*pc]];
            let res = format!("DCR {}\n", reg);
            *pc += 1;
            return res;
        })(),
        0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x36 | 0x3e => (|| {
            let registers = HashMap::from([
                (0x06, "B"),
                (0x0e, "C"),
                (0x16, "D"),
                (0x1e, "E"),
                (0x26, "H"),
                (0x2e, "L"),
                (0x36, "M"),
                (0x3e, "A")
            ]);
            let reg = registers[&buffer[*pc]];
            let res = format!("MVI {},{}\n", reg, buffer[*pc + 1]);
            *pc += 2;
            return res;
        })(),
        0x07 => (|| {
            *pc += 1;
            return String::from("RLC\n");
        } )(),
        0x09 | 0x19 | 0x29 | 0x39 => (|| {
            let registers = HashMap::from([
                (0x09, "B"),
                (0x19, "D"),
                (0x29, "H"),
                (0x39, "SP"),
            ]);
            let reg = registers[&buffer[*pc]];
            let res = format!("DAD {}\n", reg);
            *pc += 1;
            return res;
        })(),
        0x0a | 0x1a => (|| {
            let registers = HashMap::from([
                (0x0a, "B"),
                (0x1a, "D"),
            ]);
            let reg = registers[&buffer[*pc]];
            let res = format!("LDAX {}\n", reg);
            *pc += 1;
            return res;
        })(),
        0x0b | 0x1b | 0x2b | 0x3b => (|| {
            let registers = HashMap::from([
                (0x0b, "B"),
                (0x1b, "D"),
                (0x2b, "H"),
                (0x3b, "SP"),
            ]);
            let reg = registers[&buffer[*pc]];
            let res = format!("DCX {}\n", reg);
            *pc += 1;
            return res;
        })(),
        0x1f => (|| {
            *pc += 1;
            return String::from("RAR\n");
        } )(),
        0x22 => (|| {
            let res = format!("SHLD {:x}{:x}\n", &buffer[*pc + 1], &buffer[*pc + 2]);
            *pc += 1;
            return res;
        } )(),
        0x27 => (|| {
            *pc += 1;
            return String::from("DAA\n");
        } )(),
        0x2f => (|| {
            *pc += 1;
            return String::from("CMA\n");
        } )(),
        0x32 => (|| {
            let ret = format!("STA {:x}{:x}\n", &buffer[*pc + 1], &buffer[*pc + 2]);
            *pc += 3;
            return ret;
        } )(),
        0x37 => (|| {
            *pc += 1;
            return format!("STC");
        } )(),
        0x3a => (|| {
            let ret =  format!("LDA {:x}{:x}\n", &buffer[*pc + 1], &buffer[*pc + 2]);
            *pc += 3;
            return ret;
        } )(),
        0x3f => (|| {
            *pc += 1;
            return String::from("CMC\n");
        } )(),
        0x40..=0x75 => (|| {
            let reg_1 = (buffer[*pc] << 2) >> 5;
            let reg_2 = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("MOV {}{}\n", reg_list[&reg_1], reg_list[&reg_2]);
        })(),
        0x76 => (|| {
            *pc += 1;
            return String::from("HLT\n");
        } )(),
        0x77 => (|| {
            *pc += 1;
            return String::from("MOV M,A\n");
        } )(),
        0x78..=0x7f => (|| {
            let reg_2 = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("MOV A,{}\n", reg_list[&reg_2]);
        } )(),
        0x80..=0x87 => (|| {
            let reg = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("ADD {}\n", reg_list[&reg]);
        })(),
        0x88..=0x8f => (|| {
            let reg = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("ADC {}\n", reg_list[&reg]);
        })(),
        0x90..=0x97 => (|| {
            let reg = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("SUB {}\n", reg_list[&reg]);
        })(),
        0x98..=0x9f => (|| {
            let reg = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("SBB {}\n", reg_list[&reg]);
        })(),
        0xa0..=0xa7 => (|| {
            let reg = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("ANA {}\n", reg_list[&reg]);
        })(),
        0xa8..=0xaf => (|| {
            let reg = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("XRA {}\n", reg_list[&reg]);
        })(),
        0xb0..=0xb7 => (|| {
            let reg = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("ORA {}\n", reg_list[&reg]);
        })(),
        0xb8..=0xbf => (|| {
            let reg = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("CMP {}\n", reg_list[&reg]);
        })(),
        0xc0 => (|| {
            return simple_render("RNZ\n", pc)
        } )(),
        0xc1 | 0xd1 | 0xe1 | 0xf1 => (|| {
            let registers = HashMap::from([
                (0xc1, "B"),
                (0xd1, "D"),
                (0xe1, "H"),
                (0xf1, "SP"),
            ]);
            let reg = registers[&buffer[*pc]];
            *pc += 1;
            return format!("POP {}\n", reg);
        })(),
        0xc2 => (|| {
            let ret = format!("JNZ {:x}{:x}\n", buffer[*pc + 1], buffer[*pc + 2]);
            *pc += 3;
            return ret;
        })(),
        0xc3 => (|| {
            let ret = format!("JMP {:x}{:x}\n", buffer[*pc + 1], buffer[*pc + 2]);
            *pc += 3;
            return ret;
        })(),
        0xc4 => (|| {
            let ret = format!("CNZ {:x}{:x}\n", buffer[*pc + 1], buffer[*pc + 2]);
            *pc += 3;
            return ret;
        })(),
        0xc5 | 0xd5 | 0xe5 | 0xf5 => (|| {
            let registers = HashMap::from([
                (0xc5, "B"),
                (0xd5, "D"),
                (0xe5, "H"),
                (0xf5, "PSW"),
            ]);
            let reg = registers[&buffer[*pc]];
            *pc += 1;
            return format!("PUSH {}\n", reg);
        })(),
        0xc6 => (|| {
            let ret = format!("ADI {:x}\n", buffer[*pc + 1]);
            *pc += 2;
            return ret;
        })(),
        0xc7 => (|| {
            return simple_render("RST 0\n", pc)
        } )(),
        0xc8 => (|| {
            return simple_render("RZ\n", pc)
        } )(),
        0xc9 => (|| {
            return simple_render("RET\n", pc)
        } )(),
        0xca => (|| {
            return triple_render("JZ", pc, buffer);
        })(),
        0xcc => (|| {
            return triple_render("CZ", pc, buffer);
        })(),
        0xcd => (|| {
            return triple_render("CALL", pc, buffer);
        })(),
        0xce => (|| {
            return double_render("ACI", pc, buffer);
        })(),
        0xcf => (|| {
            return simple_render("RST 1\n", pc)
        } )(),
        0xd0 => (|| {
            return simple_render("RNC\n", pc)
        } )(),
        0xd2 => (|| {
            return triple_render("JNC", pc, buffer);
        })(),
        0xd3 => (|| {
            return double_render("OUT", pc, buffer);
        })(),
        0xd4 => (|| {
            return triple_render("CNC", pc, buffer);
        })(),
        0xd6 => (|| {
            return double_render("SUI", pc, buffer);
        })(),
        0xd7 => (|| {
            return simple_render("RST 2\n", pc);
        })(),
        0xd8 => (|| {
            return simple_render("RC\n", pc);
        })(),
        _ => (|| {
            return simple_render("Nimpl\n", pc)
        } )(),
    }
}

fn simple_render(string: &str, pc: &mut usize) -> String{
    *pc += 1;
    return String::from(string);
}

fn double_render(string: &str, pc: &mut usize, buffer: &Vec<u8>) -> String {
    let ret = format!("{} {:x}\n",string, buffer[*pc + 1]);
    *pc += 2;
    return ret;
}

fn triple_render(string: &str, pc: &mut usize, buffer: &Vec<u8>) -> String {
    let ret = format!("{} {:x}{:x}\n",string, buffer[*pc + 1], buffer[*pc + 2]);
    *pc += 3;
    return ret;
}
