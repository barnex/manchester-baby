use crate::*;

pub fn display(mem: &[i32], pc: u8, acc: i32) {
    // clear screen and hide cursor
    print!("\x1B[?25l");
    print!("\x1B[H");
    for (i, &word) in mem.iter().enumerate() {
        let (opcode, s) = decode(word);
        print!("{}", if i == pc as usize { ">" } else { " " });
        println!(
            "{i:02}  {} {opcode} {s} \t({word})  \x1B[K",
            format!("{:032b}: ", word.reverse_bits())
                .replace('0', "·")
                .replace('1', "▇")
        )
    }
    println!();
    print!("pc: {pc:02} acc: {acc:+012} ");
    if let Some(instr) = mem.get(pc as usize) {
        let (opcode, s) = decode(*instr);
        print!("{opcode} {s}")
    }
    println!();
}
