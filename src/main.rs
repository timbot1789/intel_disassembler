use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read(file_path)
        .expect("Should have been able to read the file");

    let mut pc: usize = 0;
    let mut result = String::from("");
    while pc < contents.len(){
        let cpc = pc;
        result.push_str(&format!("{:04x} | {}", cpc, &disassemble_8080(&contents, &mut pc)));
    }
    println!("{}", result);
}

// fn gen_testing_file() {
//     let arr: Vec<u8> = (0..=255).collect();
//     fs::write("testfile", arr);
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
        0x00 => simple_render("NOP\n", pc),
        0x01 => triple_render("LXI B,", pc, buffer),
        0x11 => triple_render("LXI D,", pc, buffer),
        0x21 => triple_render("LXI H,", pc, buffer),
        0x31 => triple_render("LXI SP,", pc, buffer),
        0x02 => simple_render("STAX B\n", pc),
        0x12 => simple_render("STAX D\n", pc),
        0x03 => simple_render("INX B\n", pc),
        0x13 => simple_render("INX D\n", pc),
        0x23 => simple_render("INX H\n", pc),
        0x33 => simple_render("INX SP\n", pc),
        0x04 => simple_render("INR B\n", pc),
        0x0c => simple_render("INR C\n", pc),
        0x14 => simple_render("INR D\n", pc),
        0x1c => simple_render("INR D\n", pc),
        0x24 => simple_render("INR E\n", pc),
        0x2c => simple_render("INR H\n", pc),
        0x34 => simple_render("INR L\n", pc),
        0x3c => simple_render("INR A\n", pc),
        0x05 => simple_render("DCR B\n", pc),
        0x0d => simple_render("DCR C\n", pc),
        0x15 => simple_render("DCR D\n", pc),
        0x17 => simple_render("RAL\n", pc),
        0x1d => simple_render("DCR E\n", pc),
        0x25 => simple_render("DCR H\n", pc),
        0x2d => simple_render("DCR L\n", pc),
        0x3d => simple_render("DCR A\n", pc),
        0x35 => simple_render("DCR M\n", pc),
        0x06 => double_render("MVI B,", pc, buffer),
        0x0e => double_render("MVI C,", pc, buffer),
        0x16 => double_render("MVI D,", pc, buffer),
        0x1e => double_render("MVI E,", pc, buffer),
        0x26 => double_render("MVI H,", pc, buffer),
        0x2e => double_render("MVI L,", pc, buffer),
        0x36 => double_render("MVI M,", pc, buffer),
        0x3e => double_render("MVI A,", pc, buffer),
        0x07 => simple_render("RLC\n", pc),
        0x09 => simple_render("DAD B\n", pc),
        0x19 => simple_render("DAD D\n", pc),
        0x29 => simple_render("DAD H\n", pc),
        0x39 => simple_render("DAD SP\n", pc),
        0x0a => simple_render("LDAX B\n", pc),
        0x1a => simple_render("LDAX D\n", pc),
        0x0b => simple_render("DCX B\n", pc),
        0x1b => simple_render("DCX D\n", pc),
        0x2b => simple_render("DCX H\n", pc),
        0x3b => simple_render("DCX SP\n", pc),
        0x0f => simple_render("RRC\n", pc),
        0x1f => simple_render("RAR\n", pc),
        0x22 => triple_render("SHLD", pc, buffer),
        0x27 => simple_render("DAA\n", pc),
        0x2a => triple_render("LHLD", pc, buffer),
        0x2f => simple_render("CMA\n", pc),
        0x32 => triple_render("STA", pc, buffer),
        0x37 => simple_render("STC\n", pc),
        0x3a => triple_render("LDA", pc, buffer),
        0x3f => simple_render("CMC\n", pc),
        0x40..=0x75 => (|| {
            let reg_1 = (buffer[*pc] << 2) >> 5;
            let reg_2 = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("MOV {},{}\n", reg_list[&reg_1], reg_list[&reg_2]);
        })(),
        0x76 => simple_render("HLT\n", pc),
        0x77 => simple_render("MOV M,A\n", pc),
        0x78..=0x7f => (|| {
            let reg = buffer[*pc] & 0b00000111;
            *pc += 1;
            return format!("MOV A,{}\n", reg_list[&reg]);
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
        0xc0 => simple_render("RNZ\n", pc),
        0xc1 => simple_render("POP B\n", pc),
        0xd1 => simple_render("POP D\n", pc),
        0xe1 => simple_render("POP H\n", pc),
        0xf1 => simple_render("POP PSW\n", pc),
        0xc2 => triple_render("JNZ", pc, buffer),
        0xc3 => triple_render("JMP", pc, buffer),
        0xc4 => triple_render("CNZ", pc, buffer),
        0xc5 => simple_render("PUSH B\n", pc),
        0xd5 => simple_render("PUSH D\n", pc),
        0xe5 => simple_render("PUSH H\n", pc),
        0xf5 => simple_render("PUSH PSW\n", pc),
        0xc6 => double_render("ADI", pc, buffer),
        0xc7 => simple_render("RST 0\n", pc),
        0xc8 => simple_render("RZ\n", pc),
        0xc9 => simple_render("RET\n", pc),
        0xca => triple_render("JZ", pc, buffer),
        0xcc => triple_render("CZ", pc, buffer),
        0xcd => triple_render("CALL", pc, buffer),
        0xce => double_render("ACI", pc, buffer),
        0xcf => simple_render("RST 1\n", pc),
        0xd0 => simple_render("RNC\n", pc),
        0xd2 => triple_render("JNC", pc, buffer),
        0xd3 => double_render("OUT", pc, buffer),
        0xd4 => triple_render("CNC", pc, buffer),
        0xd6 => double_render("SUI", pc, buffer),
        0xd7 => simple_render("RST 2\n", pc),
        0xd8 => simple_render("RC\n", pc),
        0xda => triple_render("JC", pc, buffer),
        0xdb => double_render("IN", pc, buffer),
        0xdc => triple_render("CC", pc, buffer),
        0xde => double_render("SBI", pc, buffer),
        0xdf => simple_render("RST 3\n", pc),
        0xe0 => simple_render("RPO\n", pc),
        0xe2 => triple_render("JPO", pc, buffer),
        0xe3 => simple_render("XHTL\n", pc),
        0xe4 => triple_render("CPO", pc, buffer),
        0xe6 => double_render("ANI", pc, buffer),
        0xe7 => simple_render("RST 4\n", pc),
        0xe8 => simple_render("RPE\n", pc),
        0xe9 => simple_render("PCHL\n", pc),
        0xea => triple_render("JPE", pc, buffer),
        0xeb => simple_render("XCHG\n", pc),
        0xec => triple_render("CPE", pc, buffer),
        0xee => double_render("XRI", pc, buffer),
        0xef => simple_render("RST 5\n", pc),
        0xf0 => simple_render("RP\n", pc),
        0xf2 => triple_render("JP", pc, buffer),
        0xf3 => simple_render("DI\n", pc),
        0xf4 => triple_render("CP", pc, buffer),
        0xf6 => double_render("ORI", pc, buffer),
        0xf7 => simple_render("RST 6\n", pc),
        0xf8 => simple_render("RM\n", pc),
        0xf9 => simple_render("SPHL\n", pc),
        0xfa => triple_render("JM", pc, buffer),
        0xfb => simple_render("EI\n", pc),
        0xfc => triple_render("CM", pc, buffer),
        0xfe => double_render("CPI", pc, buffer),
        0xff => simple_render("RST 7\n", pc),
        _ => simple_render(&format!("Nimpl: {:x}\n", buffer[*pc]), pc),
    }
}

fn simple_render(string: &str, pc: &mut usize) -> String{
    *pc += 1;
    return String::from(string);
}

fn double_render(string: &str, pc: &mut usize, buffer: &Vec<u8>) -> String {
    let ret = format!("{} {:02x}\n",string, buffer[*pc + 1]);
    *pc += 2;
    return ret;
}

fn triple_render(string: &str, pc: &mut usize, buffer: &Vec<u8>) -> String {
    let ret = format!("{} {:02x}{:02x}\n",string, buffer[*pc + 2], buffer[*pc + 1]);
    *pc += 3;
    return ret;
}
