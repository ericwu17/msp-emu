use std::collections::HashMap;

use crate::{
    asm_line::AsmLine,
    ccode::CC,
    get_verbs::Global,
    operand::{Operand, Reg},
};

const STACK_INIT_POSITION: u16 = 0x8000;

#[derive(Debug)]
pub enum UnresolvedLabel {
    FullWord { offset: usize, label: String },
    Low10Bits { offset: usize, label: String },
}

pub fn generate_bytes(globals: Vec<Global>, instrs: Vec<AsmLine>) -> Vec<u8> {
    let initial_instrs = vec![
        AsmLine::MOV(
            Operand::Imm(STACK_INIT_POSITION),
            Operand::Reg(Reg::SP),
            false,
        ),
        AsmLine::Jump(CC::Unconditional, "main".to_owned()),
    ];

    let mut result_bytes = Vec::new();
    let mut unresolved_labels: Vec<UnresolvedLabel> = Vec::new();
    unresolved_labels.push(UnresolvedLabel::Low10Bits {
        offset: 4,
        label: "main".to_owned(),
    });
    let mut label_map: HashMap<String, usize> = HashMap::new();

    for instr in initial_instrs {
        convert_instr_to_bytes(
            instr,
            &mut result_bytes,
            &mut unresolved_labels,
            &mut label_map,
        );
    }

    for global in globals {
        label_map.insert(global.label, result_bytes.len());
        result_bytes.extend(global.initial_bytes);
    }
    if result_bytes.len() % 2 != 0 {
        result_bytes.push(0x00); // instructions must be aligned on an even byte boundary
    }

    for instr in instrs {
        let mut instr = instr.clone();
        optimize_zero_index_instr(&mut instr);
        convert_instr_to_bytes(
            instr,
            &mut result_bytes,
            &mut unresolved_labels,
            &mut label_map,
        )
    }

    resolve_labels(&mut result_bytes, &unresolved_labels, &label_map);

    result_bytes
}

fn convert_instr_to_bytes(
    instr: AsmLine,
    result: &mut Vec<u8>,
    unresolved_labels: &mut Vec<UnresolvedLabel>,
    label_map: &mut HashMap<String, usize>,
) {
    match &instr {
        AsmLine::Label(s) => {
            let offset = result.len() - 2; // we subtract two because all jumps to
                                           //instruction labels take an immediate which is two less
                                           // than the actual position to start executing
            label_map.insert(s.clone(), offset);
        }

        AsmLine::Jump(cc, label) => {
            unresolved_labels.push(UnresolvedLabel::Low10Bits {
                offset: result.len(),
                label: label.clone(),
            });
            let c_code = cc.to_bits_repr();
            let instr: u16 = 0x2000 | c_code;
            let [low_byte, high_byte] = instr.to_le_bytes();
            result.push(low_byte);
            result.push(high_byte);
        }

        AsmLine::RRC(op, is_byte_instr)
        | AsmLine::SWPB(op, is_byte_instr)
        | AsmLine::RRA(op, is_byte_instr)
        | AsmLine::SXT(op, is_byte_instr)
        | AsmLine::PUSH(op, is_byte_instr)
        | AsmLine::CALL(op, is_byte_instr) => {
            let mut instr_word: u16 = 0x1000;
            let opcode: u16 = match &instr {
                AsmLine::RRC(..) => 0 << 7,
                AsmLine::SWPB(..) => 1 << 7,
                AsmLine::RRA(..) => 2 << 7,
                AsmLine::SXT(..) => 3 << 7,
                AsmLine::PUSH(..) => 4 << 7,
                AsmLine::CALL(..) => 5 << 7,
                _ => unreachable!(),
            };
            instr_word = instr_word | opcode;
            if *is_byte_instr {
                instr_word = instr_word | (1 << 6);
            }
            instr_word |= op.to_as_bits();
            instr_word |= op.to_reg_bits();

            let [low_byte, high_byte] = instr_word.to_le_bytes();
            result.push(low_byte);
            result.push(high_byte);

            match op.get_imm_word() {
                (None, None) => {}
                (None, Some(_)) => unreachable!(),
                (Some(imm), optional_unres_label) => {
                    if let Some(label) = optional_unres_label {
                        unresolved_labels.push(UnresolvedLabel::FullWord {
                            offset: result.len(),
                            label: label.clone(),
                        });
                    }
                    let [low_byte, high_byte] = imm.to_le_bytes();
                    result.push(low_byte);
                    result.push(high_byte);
                }
            }
        }

        AsmLine::MOV(src_op, dst_op, is_byte_instr)
        | AsmLine::ADD(src_op, dst_op, is_byte_instr)
        | AsmLine::ADDC(src_op, dst_op, is_byte_instr)
        | AsmLine::SUB(src_op, dst_op, is_byte_instr)
        | AsmLine::SUBC(src_op, dst_op, is_byte_instr)
        | AsmLine::CMP(src_op, dst_op, is_byte_instr)
        | AsmLine::BIT(src_op, dst_op, is_byte_instr)
        | AsmLine::BIC(src_op, dst_op, is_byte_instr)
        | AsmLine::BIS(src_op, dst_op, is_byte_instr)
        | AsmLine::XOR(src_op, dst_op, is_byte_instr)
        | AsmLine::AND(src_op, dst_op, is_byte_instr) => {
            let mut instr_word: u16 = 0x0000;
            let opcode: u16 = match &instr {
                AsmLine::MOV(..) => 0x4 << 12,
                AsmLine::ADD(..) => 0x5 << 12,
                AsmLine::ADDC(..) => 0x6 << 12,
                AsmLine::SUBC(..) => 0x7 << 12,
                AsmLine::SUB(..) => 0x8 << 12,
                AsmLine::CMP(..) => 0x9 << 12,
                AsmLine::BIT(..) => 0xB << 12,
                AsmLine::BIC(..) => 0xC << 12,
                AsmLine::BIS(..) => 0xD << 12,
                AsmLine::XOR(..) => 0xE << 12,
                AsmLine::AND(..) => 0xF << 12,
                _ => unreachable!(),
            };
            instr_word |= opcode;
            if *is_byte_instr {
                instr_word |= 1 << 6;
            }
            instr_word |= src_op.to_as_bits();
            instr_word |= dst_op.to_ad_bit();
            instr_word |= src_op.to_reg_bits() << 8;
            instr_word |= dst_op.to_reg_bits();

            let [low_byte, high_byte] = instr_word.to_le_bytes();
            result.push(low_byte);
            result.push(high_byte);

            match src_op.get_imm_word() {
                (None, None) => {}
                (None, Some(_)) => unreachable!(),
                (Some(imm), optional_unres_label) => {
                    if let Some(label) = optional_unres_label {
                        unresolved_labels.push(UnresolvedLabel::FullWord {
                            offset: result.len(),
                            label: label.clone(),
                        });
                    }
                    let [low_byte, high_byte] = imm.to_le_bytes();
                    result.push(low_byte);
                    result.push(high_byte);
                }
            }

            match dst_op.get_imm_word() {
                (None, None) => {}
                (None, Some(_)) => unreachable!(),
                (Some(imm), optional_unres_label) => {
                    if let Some(label) = optional_unres_label {
                        unresolved_labels.push(UnresolvedLabel::FullWord {
                            offset: result.len(),
                            label: label.clone(),
                        });
                    }
                    let [low_byte, high_byte] = imm.to_le_bytes();
                    result.push(low_byte);
                    result.push(high_byte);
                }
            }
        }

        AsmLine::RETI => {
            panic!("RETI not implemented")
        }
        AsmLine::DADD(_, _, _) => {
            panic!("DADD not implemented")
        }
    }
}

fn optimize_zero_index_instr(instr: &mut AsmLine) {
    match instr {
        AsmLine::Label(_) => {}
        AsmLine::Jump(_, _) => {}
        AsmLine::DADD(_, _, _) => {}
        AsmLine::RETI => {}
        AsmLine::RRC(src, _)
        | AsmLine::SWPB(src, _)
        | AsmLine::RRA(src, _)
        | AsmLine::SXT(src, _)
        | AsmLine::PUSH(src, _)
        | AsmLine::CALL(src, _)
        | AsmLine::MOV(src, _, _)
        | AsmLine::ADD(src, _, _)
        | AsmLine::ADDC(src, _, _)
        | AsmLine::SUB(src, _, _)
        | AsmLine::SUBC(src, _, _)
        | AsmLine::CMP(src, _, _)
        | AsmLine::BIT(src, _, _)
        | AsmLine::BIC(src, _, _)
        | AsmLine::BIS(src, _, _)
        | AsmLine::XOR(src, _, _)
        | AsmLine::AND(src, _, _) => match src {
            Operand::IndexedReg(reg, 0) => {
                *src = Operand::Indirect(*reg);
            }
            _ => {}
        },
    }
}

fn resolve_labels(
    result_bytes: &mut Vec<u8>,
    unresolved_labels: &Vec<UnresolvedLabel>,
    label_map: &HashMap<String, usize>,
) {
    for unres_label in unresolved_labels {
        match unres_label {
            UnresolvedLabel::FullWord { offset, label } => {
                let label_location = label_map.get(label);
                let label_location = match label_location {
                    None => panic!("unresolved label {}", label),
                    Some(l) => *l,
                };
                let word = label_location as u16;

                let [low_byte, high_byte] = word.to_le_bytes();
                result_bytes[*offset] = low_byte;
                result_bytes[*offset + 1] = high_byte;
            }
            UnresolvedLabel::Low10Bits { offset, label } => {
                let label_location = label_map.get(label);
                let label_location = match label_location {
                    None => panic!("unresolved label {}", label),
                    Some(l) => *l,
                };
                let difference_in_addrs = label_location as i64 - *offset as i64;
                assert!(difference_in_addrs % 2 == 0);
                let signed_offset = difference_in_addrs / 2;

                // check that the signed offset will fit in 10 bits
                if signed_offset > 511 || signed_offset < -512 {
                    panic!("trying to jump too far with label {}!", label);
                }
                let signed_offset_bits = (signed_offset as i16) & 0x03FF;

                let [low_byte, high_byte] = signed_offset_bits.to_le_bytes();
                result_bytes[*offset] = low_byte;
                result_bytes[*offset + 1] |= high_byte;
            }
        }
    }
}
