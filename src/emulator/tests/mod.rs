#[cfg(test)]
pub mod call;
#[cfg(test)]
pub mod test_double_operand_instrs;

#[cfg(test)]
fn convert_words_to_bytes(words: Vec<u16>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for w in words {
        let [low, high] = w.to_le_bytes();
        bytes.push(low);
        bytes.push(high);
    }
    bytes
}
