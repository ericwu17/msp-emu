pub mod parse_operand;

use crate::{
    asm_line::AsmLine,
    ccode::CC,
    get_verbs::parse_operand::parse_operand,
    operand::{Operand, Reg},
    source_cursor::SourceCodeCursor,
};

#[derive(Debug)]
pub struct Global {
    pub label: String,
    pub initial_bytes: Vec<u8>,
}

pub fn get_tokens(source_code_contents: String) -> (Vec<Global>, Vec<AsmLine>) {
    let mut cursor = SourceCodeCursor::new(source_code_contents);

    let mut globals = Vec::new();

    let mut lines: Vec<AsmLine> = Vec::new();

    while cursor.peek().is_some() {
        // this loop will consume one line per iteration:

        // consume leading whitespace
        consume_whitespace(&mut cursor);
        match cursor.peek() {
            None => break,
            Some('\n') | Some(';') | Some('.') => {
                if cursor.begins_with(".bits") {
                    // parse a global variable declaration
                    for _ in 0..5 {
                        cursor.next();
                    }
                    consume_whitespace(&mut cursor);
                    let label = lines.pop().unwrap().as_label_str();
                    let initial_bytes = parse_initial_bytes(&mut cursor);

                    globals.push(Global {
                        label,
                        initial_bytes,
                    });
                }
                // empty line or comment/directive. Consume the empty line.
                consume_rest_of_line(&mut cursor);
            }

            _ => {
                let mut component_1 = String::new();
                while cursor.peek().is_some() && !cursor.peek().unwrap().is_ascii_whitespace() {
                    let c = cursor.next().unwrap();
                    component_1.push(c);
                }

                if component_1.ends_with(":") {
                    // parse label
                    lines.push(AsmLine::Label(
                        component_1.strip_suffix(":").unwrap().to_owned(),
                    ));
                } else {
                    let component_1_base;
                    let mut is_byte_instr = false;
                    if component_1.ends_with(".W") || component_1.ends_with(".B") {
                        is_byte_instr = component_1.ends_with(".B");
                        component_1_base = &component_1[..component_1.len() - 2];
                    } else {
                        component_1_base = &component_1;
                    }

                    match component_1_base {
                        "RRC" | "SWPB" | "RRA" | "SXT" | "PUSH" | "CALL" => {
                            // SINGLE OPERAND FAMILY
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            match component_1_base {
                                "RRC" => {
                                    lines.push(AsmLine::RRC(operand, is_byte_instr));
                                }
                                "SWPB" => {
                                    lines.push(AsmLine::SWPB(operand, false));
                                }
                                "RRA" => {
                                    lines.push(AsmLine::RRA(operand, is_byte_instr));
                                }
                                "SXT" => {
                                    lines.push(AsmLine::SXT(operand, false));
                                }
                                "PUSH" => {
                                    lines.push(AsmLine::PUSH(operand, is_byte_instr));
                                }
                                "CALL" => {
                                    lines.push(AsmLine::CALL(operand, false));
                                }
                                _ => unreachable!(),
                            }
                        }
                        "RETI" => {
                            lines.push(AsmLine::RETI);
                        }

                        // JUMPS
                        "JNE" | "JNZ" => {
                            let label = parse_jmp_label(&mut cursor);
                            lines.push(AsmLine::Jump(CC::NotEq, label));
                        }
                        "JEQ" | "JZ" => {
                            let label = parse_jmp_label(&mut cursor);
                            lines.push(AsmLine::Jump(CC::Eq, label));
                        }
                        "JNC" | "JLO" => {
                            let label = parse_jmp_label(&mut cursor);
                            lines.push(AsmLine::Jump(CC::NoCarry, label));
                        }
                        "JC" | "JHS" => {
                            let label = parse_jmp_label(&mut cursor);
                            lines.push(AsmLine::Jump(CC::Carry, label));
                        }
                        "JN" => {
                            let label = parse_jmp_label(&mut cursor);
                            lines.push(AsmLine::Jump(CC::Neg, label));
                        }
                        "JGE" => {
                            let label = parse_jmp_label(&mut cursor);
                            lines.push(AsmLine::Jump(CC::GreaterEq, label));
                        }
                        "JL" => {
                            let label = parse_jmp_label(&mut cursor);
                            lines.push(AsmLine::Jump(CC::Less, label));
                        }
                        "JMP" => {
                            let label = parse_jmp_label(&mut cursor);
                            lines.push(AsmLine::Jump(CC::Unconditional, label));
                        }

                        "MOV" | "ADD" | "ADDC" | "SUB" | "SUBC" | "CMP" | "DADD" | "BIT"
                        | "BIC" | "BIS" | "XOR" | "AND" => {
                            // DOUBLE OPERAND FAMILY
                            consume_whitespace(&mut cursor);
                            let operand_1 = parse_operand(&mut cursor);
                            assert_eq!(cursor.next(), Some(','));
                            let operand_2 = parse_operand(&mut cursor);
                            match component_1_base {
                                "MOV" => {
                                    lines.push(AsmLine::MOV(operand_1, operand_2, is_byte_instr));
                                }
                                "ADD" => {
                                    lines.push(AsmLine::ADD(operand_1, operand_2, is_byte_instr));
                                }
                                "ADDC" => {
                                    lines.push(AsmLine::ADDC(operand_1, operand_2, is_byte_instr));
                                }
                                "SUB" => {
                                    lines.push(AsmLine::SUB(operand_1, operand_2, is_byte_instr));
                                }
                                "SUBC" => {
                                    lines.push(AsmLine::SUBC(operand_1, operand_2, is_byte_instr));
                                }
                                "CMP" => {
                                    lines.push(AsmLine::CMP(operand_1, operand_2, is_byte_instr));
                                }
                                "DADD" => {
                                    lines.push(AsmLine::DADD(operand_1, operand_2, is_byte_instr));
                                }
                                "BIT" => {
                                    lines.push(AsmLine::BIT(operand_1, operand_2, is_byte_instr));
                                }
                                "BIC" => {
                                    lines.push(AsmLine::BIC(operand_1, operand_2, is_byte_instr));
                                }
                                "BIS" => {
                                    lines.push(AsmLine::BIS(operand_1, operand_2, is_byte_instr));
                                }
                                "XOR" => {
                                    lines.push(AsmLine::XOR(operand_1, operand_2, is_byte_instr));
                                }
                                "AND" => {
                                    lines.push(AsmLine::AND(operand_1, operand_2, is_byte_instr));
                                }
                                _ => unreachable!(),
                            }
                        }

                        // ========================
                        // Pseudo-operations
                        // ========================
                        "ADC" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::ADDC(Operand::Imm(0), operand, is_byte_instr));
                        }
                        "BR" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::MOV(operand, Operand::Reg(Reg::PC), false));
                        }
                        "CLR" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::MOV(Operand::Imm(0), operand, is_byte_instr));
                        }

                        "DEC" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::SUB(Operand::Imm(1), operand, is_byte_instr));
                        }
                        "DECD" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::SUB(Operand::Imm(2), operand, is_byte_instr));
                        }
                        "INC" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::ADD(Operand::Imm(1), operand, is_byte_instr));
                        }
                        "INCD" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::ADD(Operand::Imm(2), operand, is_byte_instr));
                        }
                        "NOP" => {
                            lines.push(AsmLine::MOV(Operand::Imm(0), Operand::Reg(Reg::CG), false));
                        }
                        "POP" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::MOV(
                                Operand::IndirectAutoInc(Reg::SP),
                                operand,
                                is_byte_instr,
                            ));
                        }
                        "RET" => {
                            lines.push(AsmLine::MOV(
                                Operand::IndirectAutoInc(Reg::SP),
                                Operand::Reg(Reg::PC),
                                false,
                            ));
                        }
                        "RLA" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::ADD(operand.clone(), operand, is_byte_instr));
                        }
                        "RLC" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::ADDC(operand.clone(), operand, is_byte_instr));
                        }
                        "SBC" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::SUBC(Operand::Imm(0), operand, is_byte_instr));
                        }

                        "TST" => {
                            consume_whitespace(&mut cursor);
                            let operand = parse_operand(&mut cursor);
                            lines.push(AsmLine::CMP(Operand::Imm(0), operand, is_byte_instr));
                        }
                        // ========================
                        // end of Pseudo-operations
                        // ========================
                        _ => {
                            panic!("unrecognized instruction {}", component_1_base);
                        }
                    }
                }

                consume_rest_of_line(&mut cursor);
            }
        }
    }

    return (globals, lines);
}

fn parse_jmp_label(cursor: &mut SourceCodeCursor) -> String {
    consume_whitespace(cursor);
    let mut res = String::new();
    while cursor.peek().is_some() && !cursor.peek().unwrap().is_ascii_whitespace() {
        let c = cursor.next().unwrap();
        res.push(c);
    }
    consume_rest_of_line(cursor);

    res
}

pub fn consume_rest_of_line(cursor: &mut SourceCodeCursor) {
    while cursor.peek() != Some('\n') && cursor.peek() != None {
        cursor.next();
    }
    // consume newline if there is one
    cursor.next();
}

pub fn consume_whitespace(cursor: &mut SourceCodeCursor) {
    while cursor.peek() == Some(' ') || cursor.peek() == Some('\t') {
        cursor.next();
    }
}

pub fn parse_initial_bytes(cursor: &mut SourceCodeCursor) -> Vec<u8> {
    consume_whitespace(cursor);
    assert!(cursor.begins_with("0x"));
    cursor.next();
    cursor.next();

    let mut base_16_lit = String::new();
    while cursor.peek().is_some() && cursor.peek().unwrap().is_digit(16) {
        base_16_lit.push(cursor.next().unwrap());
    }
    assert_eq!(Some(','), cursor.next());
    let mut num_bits_base_10_lit = String::new();
    while cursor.peek().is_some() && cursor.peek().unwrap().is_numeric() {
        num_bits_base_10_lit.push(cursor.next().unwrap());
    }

    let base_16_int = u64::from_str_radix(&base_16_lit, 16).unwrap();
    let num_bytes = usize::from_str_radix(&num_bits_base_10_lit, 10).unwrap() / 8;

    let bytes = base_16_int.to_le_bytes();

    bytes.into_iter().take(num_bytes).collect()
}
