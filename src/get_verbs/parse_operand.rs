use crate::{
    asm_line::{Operand, Reg},
    source_cursor::SourceCodeCursor,
};

pub fn parse_operand(cursor: &mut SourceCodeCursor) -> Operand {
    if let Some(r) = parse_reg(cursor) {
        return Operand::Register(r);
    }
    if let Some(operand) = parse_imm(cursor) {
        return operand;
    }
    if cursor.peek() == Some('@') {
        cursor.next();
        let reg = parse_reg(cursor).unwrap();
        if cursor.peek() == Some('+') {
            cursor.next();
            return Operand::IndirectAutoInc(reg);
        } else {
            return Operand::Indirect(reg);
        }
    }

    if cursor.peek() == Some('&') {
        cursor.next();
        let s = parse_label(cursor);
        assert_eq!(cursor.next().unwrap(), '+');
        assert_eq!(cursor.next().unwrap(), '0');
        return Operand::Absolute(s);
    }

    let offset: i16 = parse_signed_int(cursor).unwrap();
    assert_eq!(cursor.next().unwrap(), '(');
    let r = parse_reg(cursor).unwrap();
    assert_eq!(cursor.next().unwrap(), ')');

    return Operand::IndexedReg(r, offset);
}

pub fn parse_reg(cursor: &mut SourceCodeCursor) -> Option<Reg> {
    // tries to parse a register. moves the cursor only if
    // successfully parsed.

    let mut res = None;
    let (a, b, c) = (cursor.peek_nth(1), cursor.peek_nth(2), cursor.peek_nth(3));
    let mut is_two_char_reg = true;
    match (a, b) {
        (Some(a), Some(b)) => match (a, b) {
            // ('P', 'C') => res = Some(Reg::PC),
            ('S', 'P') => res = Some(Reg::SP),
            ('S', 'R') => res = Some(Reg::SR),
            // ('C', 'G') => res = Some(Reg::CG),
            ('r', '4') => res = Some(Reg::R4),
            ('r', '5') => res = Some(Reg::R5),
            ('r', '6') => res = Some(Reg::R6),
            ('r', '7') => res = Some(Reg::R7),
            ('r', '8') => res = Some(Reg::R8),
            ('r', '9') => res = Some(Reg::R9),
            _ => {
                is_two_char_reg = false;
            }
        },
        _ => {
            is_two_char_reg = false;
        }
    }
    let mut is_three_char_reg = true;
    match (a, b, c) {
        (Some(a), Some(b), Some(c)) => match (a, b, c) {
            ('r', '1', '0') => res = Some(Reg::R10),
            ('r', '1', '1') => res = Some(Reg::R11),
            ('r', '1', '2') => res = Some(Reg::R12),
            ('r', '1', '3') => res = Some(Reg::R13),
            ('r', '1', '4') => res = Some(Reg::R14),
            ('r', '1', '5') => res = Some(Reg::R15),
            _ => {
                is_three_char_reg = false;
            }
        },
        _ => {
            is_three_char_reg = false;
        }
    }

    if is_three_char_reg {
        cursor.next();
        cursor.next();
        cursor.next();
    } else if is_two_char_reg {
        cursor.next();
        cursor.next();
    }

    return res;
}

fn parse_imm(cursor: &mut SourceCodeCursor) -> Option<Operand> {
    // tries to parse an imm such as "#13". moves the cursor only if
    // successfully parsed.
    let mut local_cursor = cursor.clone();

    if local_cursor.next() != Some('#') {
        return None;
    }

    let mut imm_str = String::new();
    while local_cursor.peek().is_some()
        && (local_cursor.peek().unwrap().is_ascii_alphanumeric()
            || local_cursor.peek().unwrap() == '_')
    {
        imm_str.push(local_cursor.next().unwrap());
    }
    let imm_u16 = u16::from_str_radix(&imm_str, 10);

    match imm_u16 {
        Ok(x) => {
            *cursor = local_cursor;
            return Some(Operand::Imm(x));
        }
        Err(_) => {
            return Some(Operand::ImmLabel(imm_str));
        }
    }
}

fn parse_signed_int(cursor: &mut SourceCodeCursor) -> Option<i16> {
    // tries to parse an imm such as "-4". moves the cursor only if
    // successfully parsed.
    let mut local_cursor = cursor.clone();

    let mut signed_int_str = String::new();
    while local_cursor.peek().is_some()
        && (local_cursor.peek().unwrap().is_ascii_digit() || local_cursor.peek().unwrap() == '-')
    {
        signed_int_str.push(local_cursor.next().unwrap());
    }
    let signed_int = i16::from_str_radix(&signed_int_str, 10);

    match signed_int {
        Ok(x) => {
            *cursor = local_cursor;
            return Some(x);
        }
        Err(_) => {
            return None;
        }
    }
}

fn parse_label(cursor: &mut SourceCodeCursor) -> String {
    let mut res = String::new();
    while cursor.peek().is_some()
        && (cursor.peek().unwrap().is_ascii_alphanumeric() || cursor.peek().unwrap() == '_')
    {
        res.push(cursor.next().unwrap());
    }

    res
}
