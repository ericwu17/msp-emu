pub fn process_single_operand_w(
    operand_1: u16,
    carry_bit: bool,
    opcode: u16,
    regs: &[u16],
) -> (
    u16,
    Option<bool>,
    Option<bool>,
    Option<bool>,
    Option<bool>,
    bool,
    u16,
) {
    let mut dec_sp = false;

    let mut new_cf = None;
    let mut new_zf = None;
    let mut new_nf = None;
    let mut new_vf = None;

    let mut new_pc_val = 0;

    let result;

    match opcode {
        0 => {
            // RRC
            new_cf = Some(operand_1 % 2 != 0);
            new_vf = Some(false);
            new_nf = Some(carry_bit);
            let mut rotated_right = operand_1 >> 1;
            if carry_bit {
                rotated_right |= 0x8000;
            }
            result = rotated_right;
            new_zf = Some(result == 0);
        }
        1 => {
            // SWPB
            result = ((operand_1 & 0xFF) << 8) | ((operand_1 & 0xFF00) >> 8);
        }
        2 => {
            // RRA
            new_cf = Some(operand_1 % 2 != 0);
            new_vf = Some(false);

            let rotated_right: i16 = (operand_1 as i16) >> 1;
            result = rotated_right as u16;
            new_zf = Some(result == 0);
            new_nf = Some(result & 0x8000 != 0);
        }
        3 => {
            // SXT
            new_vf = Some(false);
            let is_negative = operand_1 & 0x0080 != 0;
            result = if is_negative {
                (operand_1 & 0x00FF) | 0xFF00
            } else {
                operand_1 & 0x00FF
            };
            new_zf = Some(result == 0);
            new_cf = Some(result != 0);
            new_nf = Some(is_negative);
        }
        4 => {
            // PUSH
            dec_sp = true;
            result = operand_1;
        }
        5 => {
            // CALL
            dec_sp = true;
            result = regs[0] + 2;

            new_pc_val = operand_1;
        }
        _ => unreachable!(),
    }

    return (result, new_cf, new_zf, new_nf, new_vf, dec_sp, new_pc_val);
}
