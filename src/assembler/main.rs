
fn is_ident(word: String) -> bool {
    let mut char_iter = word.chars();

    match char_iter.next() {
        None => return false,
        Some(ch) => {
            if !ch.is_alphabetic() {
                return false;
            }
        }
    }

    loop {
        match char_iter.next() {
            None => break,
            Some(ch) => {
                if !(ch.is_alphanumeric() || ch == '_') {
                    return false;
                }
            }
        }
    }

    return true
}


fn is_literal(word: String) -> bool {
    word.chars().map(|ch| if (!ch.is_numeric) { return false });

    let val: u16 = match word.parse() {
        Ok(n) => n,
        Err(_) => return false
    }

    if (val & 0x8000) != 0 {
        return false
    }

    return true
}


fn parse_dest(word: String) -> Result<u16, bool> {
    match word {
        ""    =>     Ok(    0b000    ),
        "M"   =>     Ok(    0b001    ),
        "D"   =>     Ok(    0b010    ),
        "MD"  =>     Ok(    0b011    ),
        "A"   =>     Ok(    0b100    ),
        "AM"  =>     Ok(    0b101    ),
        "AD"  =>     Ok(    0b110    ),
        "AMD" =>     Ok(    0b111    ),
        _     =>    Err(    false    )
    }
}

fn parse_jump(word: String) -> Result<u16, bool> {
    match word {
        "JGT" =>    Ok(    0b001    ),
        "JEQ" =>    Ok(    0b010    ),
        "JGE" =>    Ok(    0b011    ),
        "JLT" =>    Ok(    0b100    ),
        "JNE" =>    Ok(    0b101    ),
        "JLE" =>    Ok(    0b110    ),
        "JMP" =>    Ok(    0b111    ),
        _     =>   Err(    false    )
    }
}

fn parse_comp(word: String) -> Result<u16, bool> {
    match word {
        "0"   =>      Ok(    0b0101010    ),
        "1"   =>      Ok(    0b0111111    ),
        "-1"  =>      Ok(    0b0111010    ),
        "D"   =>      Ok(    0b0001100    ),
        "A"   =>      Ok(    0b0110000    ),
        "!D"  =>      Ok(    0b0001101    ),
        "!A"  =>      Ok(    0b0110001    ),
        "-D"  =>      Ok(    0b0001111    ),
        "-A"  =>      Ok(    0b0110001    ),
        "D+1" =>      Ok(    0b0011111    ),
        "A+1" =>      Ok(    0b0110111    ),
        "D-1" =>      Ok(    0b0001110    ),
        "A-1" =>      Ok(    0b0110010    ),
        "D+A" =>      Ok(    0b0000010    ),
        "D-A" =>      Ok(    0b0010010    ),
        "A-D" =>      Ok(    0b0000111    ),
        "D&A" =>      Ok(    0b0000000    ),
        "D|A" =>      Ok(    0b0010101    ),
        "M"   =>      Ok(    0b1110000    ),
        "!M"  =>      Ok(    0b1110001    ),
        "-M"  =>      Ok(    0b1110011    ),
        "M+1" =>      Ok(    0b1110111    ),
        "M-1" =>      Ok(    0b1110010    ),
        "D+M" =>      Ok(    0b1000010    ),
        "D-M" =>      Ok(    0b1010011    ),
        "M-D" =>      Ok(    0b1000111    ),
        "D&M" =>      Ok(    0b1000000    ),
        "D|M" =>      Ok(    0b1010101    ),
        _     =>     Err(    false        )
    }
}


fn init_symbol_table() -> HashMap<String, u16> {
    let symbol_table = HashMap::new();

    symbol_table.insert("SP",      0x0000);
    symbol_table.insert("LCL",     0x0001);
    symbol_table.insert("ARG",     0x0002);
    symbol_table.insert("THIS",    0x0003);
    symbol_table.insert("THAT",    0x0004);
    symbol_table.insert("SCREEN",  0x4000);
    symbol_table.insert("KBD",     0x6000);
    
    for i in 0..16 {
        let key = "R".push_str(i.to_string());
        symbol_table.insert(key, i);
    }

    synbol_table
}

