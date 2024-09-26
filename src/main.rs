use std::io::{Read, Write};

/// Increment the data pointer by one (to point to the next cell to the right).
const FWD: u8 = b'>';
/// Decrement the data pointer by one (to point to the next cell to the left).
const BAK: u8 = b'<';
/// Increment the byte at the data pointer by one.
const INC: u8 = b'+';
/// Decrement the byte at the data pointer by one.
const DEC: u8 = b'-';
/// Output the byte at the data pointer.
const PUT: u8 = b'.';
/// Accept one byte of input, storing its value in the byte at the data pointer.
const INP: u8 = b',';
/// If the byte at the data pointer is zero,
/// then instead of moving the instruction pointer forward to the next command,
/// jump it forward to the command after the matching ] command.
const LBR: u8 = b'[';
/// If the byte at the data pointer is nonzero,
/// then instead of moving the instruction pointer forward to the next command,
/// jump it back to the command after the matching [ command.
const RBR: u8 = b']';

fn generate_bracket_jumptables(src: &[u8]) -> Vec<u32> {
    let mut table = vec![0; src.len()];
    let mut stack = Vec::new();

    for (i, &byte) in src.iter().enumerate() {
        if byte == LBR {
            stack.push(i);
        } else if byte == RBR {
            if let Some(start) = stack.pop() {
                table[start] = i as u32;
                table[i] = start as u32;
            } else {
                panic!("Unmatched closing bracket at position {}", i);
            }
        }
    }

    if !stack.is_empty() {
        panic!("Unmatched opening bracket(s) at positions: {:?}", stack);
    }

    table
}

fn engine(src: &[u8]) -> std::io::Result<()> {
    let mut ip = 0;
    let mut dp = 0;
    let mut mem = vec![0u8; 1024 * 1024];

    let jmp = generate_bracket_jumptables(src);

    loop {
        if ip == src.len() {
            break Ok(());
        }
        match src[ip] {
            FWD => dp += 1,
            BAK => dp -= 1,
            INC => mem[dp] = mem[dp].wrapping_add(1),
            DEC => mem[dp] = mem[dp].wrapping_sub(1),
            PUT => print!("{}", mem[dp] as char),
            INP => std::io::stdin().read_exact(std::array::from_mut(&mut mem[dp]))?,
            LBR => if mem[dp] == 0 {
                ip = jmp[ip] as usize;
                continue;
            }
            RBR => if mem[dp] != 0 {
                ip = jmp[ip] as usize;
                continue;
            }
            _ => {}
        }
        ip += 1;
    }
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let input = &*args[1];

    let text = std::fs::read_to_string(input)?;

    engine(text.as_bytes())?;

    std::io::stdout().flush()?;

    Ok(())
}

