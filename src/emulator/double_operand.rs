pub fn process_double_operand_w(
    operand_1: u16,
    operand_2: u16,
    carry_bit: bool,
    opcode: u16,
) -> (u16, Option<bool>, Option<bool>, Option<bool>, Option<bool>) {
    let mut new_cf = None;
    let mut new_zf = None;
    let mut new_nf = None;
    let mut new_vf = None;

    let result;
    match opcode {
        4 => {
            // MOV
            result = operand_1;
        }
        5 => {
            // ADD
            let has_carry;
            let has_overflow;
            (result, has_carry) = operand_1.overflowing_add(operand_2);
            (_, has_overflow) = (operand_1 as i16).overflowing_add(operand_2 as i16);
            new_cf = Some(has_carry);
            new_zf = Some(result == 0);
            new_nf = Some(result & 0x8000 != 0);
            new_vf = Some(has_overflow);
        }
        6 => {
            // ADDC
            let has_carry;
            let has_overflow;
            (result, has_carry) = operand_1.carrying_add(operand_2, carry_bit);
            (_, has_overflow) = (operand_1 as i16).carrying_add(operand_2 as i16, carry_bit);
            new_cf = Some(has_carry);
            new_zf = Some(result == 0);
            new_nf = Some(result & 0x8000 != 0);
            new_vf = Some(has_overflow);
        }
        7 => {
            // SUBC
            let has_carry;
            let has_overflow;
            (result, has_carry) = operand_2.borrowing_sub(operand_1, !carry_bit);
            (_, has_overflow) = (operand_2 as i16).borrowing_sub(operand_1 as i16, !carry_bit);
            new_cf = Some(!has_carry); // set to 1 if no borrow, reset if borrow
            new_zf = Some(result == 0);
            new_nf = Some(result & 0x8000 != 0);
            new_vf = Some(has_overflow);
        }
        8 | 9 => {
            // SUB or CMP
            let has_carry;
            let has_overflow;
            (result, has_carry) = operand_2.overflowing_sub(operand_1);
            println!("{}", operand_1);
            println!("{}", operand_2);
            (_, has_overflow) = (operand_2 as i16).overflowing_sub(operand_1 as i16);
            new_cf = Some(!has_carry); // set to 1 if no borrow, reset if borrow
            new_zf = Some(result == 0);
            new_nf = Some(result & 0x8000 != 0);
            new_vf = Some(has_overflow);
        }
        10 => {
            // DADD
            unreachable!()
        }
        11 | 15 => {
            // BIT or AND
            result = operand_1 & operand_2;
            new_cf = Some(result != 0);
            new_zf = Some(result == 0);
            new_nf = Some(result & 0x8000 != 0);
            new_vf = Some(false);
        }
        12 => {
            // BIC
            result = !operand_1 & operand_2;
        }
        13 => {
            // BIS
            result = operand_1 | operand_2;
        }
        14 => {
            // XOR
            result = operand_1 ^ operand_2;
            new_cf = Some(result != 0);
            new_zf = Some(result == 0);
            new_nf = Some(result & 0x8000 != 0);
            new_vf = Some((operand_1 & 0x8000 != 0) && (operand_2 & 0x8000 != 0));
        }

        _ => unreachable!(),
    }

    return (result, new_cf, new_zf, new_nf, new_vf);
}
