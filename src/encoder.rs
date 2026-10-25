use std::collections::HashMap;

use crate::{assembler::{Item, JumpKind}, cpu::{CODE_END, CODE_START}, instruction::Instruction};

pub fn encode(src: String, items: Vec<Item>) -> Result<Vec<u8>, String> {
    let mut labels: HashMap<&str, u16> = HashMap::new();
    let mut adress: u16 = 0;

    for item in &items {
        match item {
            Item::Label(s, e) => {
                labels.insert(&src[*s..*e], adress);
            },
            Item::Instruction(i) => {
                adress += i.encode().len() as u16;
            },
            Item::UnresolvedJump{..} | Item::UnresolvedCall(..) => {
                adress += 3;
            }
        }
    }

    let mut bytes = vec![];

    for item in items {
        match item {
            Item::Label(..) => {},
            Item::Instruction(i) => {
                bytes.extend(i.encode());
            },
            Item::UnresolvedJump { kind, label } => {
                let label = &src[label.0..label.1];
                let target = labels.get(label)
                    .ok_or_else(|| format!("undefined label: {label}"))?;
                bytes.extend(match kind {
                    JumpKind::JMP => Instruction::JMP(*target),
                    JumpKind::JEQ => Instruction::JEQ(*target),
                    JumpKind::JNE => Instruction::JNE(*target),
                    JumpKind::JLT => Instruction::JLT(*target),
                    JumpKind::JLE => Instruction::JLE(*target),
                    JumpKind::JGT => Instruction::JGT(*target),
                    JumpKind::JGE => Instruction::JGE(*target),
                    JumpKind::JC => Instruction::JC(*target),
                    JumpKind::JNC => Instruction::JNC(*target),
                }.encode());
            },
            Item::UnresolvedCall(s, e) => {
                let label = &src[s..e];
                let target = labels.get(&label)
                    .ok_or_else(|| format!("undefined label: {label}"))?;
                bytes.extend(Instruction::CALL(*target).encode());
            }
        }
    }

    if bytes.len() > CODE_END as usize {
        return Err(format!(
            "Program size out of bounds; Reserved size for code: {:#X}-{:#X}, program size: {} bytes",
            CODE_START, CODE_END, bytes.len()
        ));
    }

    Ok(bytes)
}
