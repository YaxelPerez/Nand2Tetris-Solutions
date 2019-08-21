use std::collections::HashMap;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
extern crate regex;
use regex::Regex;

pub struct Program {
    mnemonics: Vec<Mnemonic>,
}

impl Program {
    pub fn from_string<S: Into<String>>(s: S) -> Program {
        Program {
            mnemonics: s.into().lines().enumerate().filter_map(|(index, line)| {
                match Mnemonic::from_string(line) {
                    Ok(m) => m,
                    Err(e) => panic!("Error parsing program at line {}: {}\n    {}", index, line, e)
                }
            }).collect()
        }
    }

    pub fn resolve_symbols(&mut self) {
        let mut symbol_table = hashmap!{
            // builtin symbols
            "R0" => 0u16,
            "R1" => 1,
            "R2" => 2,
            "R3" => 3,
            "R4" => 4,
            "R5" => 5,
            "R6" => 6,
            "R7" => 7,
            "R8" => 8,
            "R9" => 9,
            "R10" => 10,
            "R11" => 11,
            "R12" => 12,
            "R13" => 13,
            "R14" => 14,
            "R15" => 15,
            "SP" => 0,
            "LCL" => 1,
            "ARG" => 2,
            "THIS" => 3,
            "THAT" => 4,
            "SCREEN" => 16384,
            "KBD" => 24576,
        };

        let mut variable_table: HashMap<&str, u16> = HashMap::new();
        let mut variable_count: u16 = 0;

        // collect all symbols, delete from mnemonics list
        let mut symbol_count: u16 = 0;
        let old_mnemonics = self.mnemonics.clone();
        let mut new_mnemonics: Vec<Mnemonic> = Vec::new();
        for (i, m) in old_mnemonics.iter().enumerate() {
            match m {
                Mnemonic::Symbol(s) => {
                    symbol_table.insert(
                        s.name.as_str(),
                        (i as u16) - symbol_count // index of next instruction, ignoring space symbols take up
                    );
                    symbol_count += 1;
                },
                _ => new_mnemonics.push(m.clone())
            }
        }
        for m in new_mnemonics.iter_mut() {
            if let Mnemonic::A(a) = m {
                if let Value::Symbol(s) = a.x {
                    let val = match symbol_table.get(s.as_str()) {
                        Some(i) => *i,
                        None => variable_count + 16
                    };
                    *m = Mnemonic::A(ACommand {
                        x: Value::Literal(val)
                    });
                }
            }
        }

        self.mnemonics = new_mnemonics;
    }

    pub fn to_bytes(&self) -> Vec<u16> {
        self.mnemonics.iter().filter_map(|m| m.to_bytes()).collect()
    }
}

#[derive(Clone)]
pub enum Mnemonic {
    A(ACommand),
    C(CCommand),
    Symbol(Symbol),
}

impl Mnemonic {
    fn from_string<S: Into<String>>(s: S) -> Result<Option<Mnemonic>, &'static str> {
        lazy_static! { static ref COMMENT_REGEXP: Regex = Regex::new(r" *//.*").unwrap(); }
        let s = s.into();
        let mnemonic_str = COMMENT_REGEXP.replace_all(s.trim(), "");

        if !mnemonic_str.is_empty() {
            if let Some(first_char) = mnemonic_str.chars().next() {
                return match first_char {
                    '@'                   => Ok(Some(Mnemonic::A(ACommand::from_string(mnemonic_str)?))),
                    'A' | 'D' | 'M' | '0' => Ok(Some(Mnemonic::C(CCommand::from_string(mnemonic_str)?))),
                    '('                   => Ok(Some(Mnemonic::Symbol(Symbol::from_string(mnemonic_str)?))),
                    _ => Err("Invalid mnemonic")
                }
            }
        }
        Ok(None)
    }

    fn to_bytes(&self) -> Option<u16> {
        match self {
            Mnemonic::A(a) => Some(a.to_bytes()),
            Mnemonic::C(c) => Some(c.to_bytes()),
            Mnemonic::Symbol(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct Symbol {
    name: String,
}

impl Symbol {
    fn from_string<S: Into<String>>(s: S) -> Result<Symbol, &'static str> {
        lazy_static! { static ref SYMBOL_REGEXP: Regex = Regex::new(r"^\(([A-Za-z_.$:][A-Za-z0-9_.$:]*)\)$").unwrap(); }
        if let Some(caps) = SYMBOL_REGEXP.captures(&s.into()) {
            Ok(Symbol {
                name: caps.get(1).unwrap().as_str().to_string(),
            })
        } else {
            Err("Could not parse symbol.")
        }
    }
}

#[derive(Clone)]
enum Value {
    Literal(u16),
    Symbol(String),
}

#[derive(Clone)]
pub struct ACommand {
    x: Value,
}

impl ACommand {
    fn from_string<S: Into<String>>(s: S) -> Result<ACommand, &'static str> {
        let s = s.into();
        lazy_static! { static ref A_SYMBOL_REGEXP: Regex = Regex::new(r"^@([A-Za-z_.$:][A-Za-z0-9_.$:]*)$").unwrap(); }
        lazy_static! { static ref A_LITERAL_REGEXP: Regex = Regex::new(r"^@(\d*)$").unwrap(); }

        if let Some(symbol_caps) = A_SYMBOL_REGEXP.captures(&s) {
            return Ok(ACommand {
                x: Value::Symbol(symbol_caps.get(1).unwrap().as_str().to_string())
            });
        }
        if let Some(literal_caps) = A_LITERAL_REGEXP.captures(&s) {
            return Ok(ACommand {
                x: match literal_caps.get(1).unwrap().as_str().to_string().parse::<u16>() {
                    Ok(parsed) => Value::Literal(parsed & 0b0111_1111_1111_1111),
                    Err(_) => return Err("Could not parse literal.")
                }
            });
        }
        Err("Could not parse literal.")
    }

    fn to_bytes(&self) -> u16 {
        match self.x {
            Value::Literal(x) => x,
            Value::Symbol(_) => 0,
        }
    }
}

impl Default for ACommand {
    fn default() -> Self { ACommand { x: Value::Literal(0) } }
}

#[derive(Default, Clone)]
pub struct CCommand {
    a: bool,

    c1: bool,
    c2: bool,
    c3: bool,
    c4: bool,
    c5: bool,
    c6: bool,

    d1: bool,
    d2: bool,
    d3: bool,

    j1: bool,
    j2: bool,
    j3: bool,
}

impl CCommand {
    fn from_string<S: Into<String>>(s: S) -> Result<CCommand, &'static str> {
        // buckle up lol
        let s = s.into();
        lazy_static! { static ref C_COMMAND_REGEXP: Regex = Regex::new(r"^(?:([AMD]{1,3})=)?([01AMD+\-\&\|!]{1,3})(?:;([JMPEQNLTG]{3}))?$").unwrap(); }
        if let Some(caps) = C_COMMAND_REGEXP.captures(&s) {
            let dest = caps.get(1).map_or("", |m| m.as_str());
            let comp = caps.get(2).map_or("", |m| m.as_str());
            let jump = caps.get(3).map_or("", |m| m.as_str());

            let mut x: u16 = 0b1110_0000_0000_0000;
            if dest.contains("A") { x |= 1 << 5; }
            if dest.contains("D") { x |= 1 << 4; }
            if dest.contains("M") { x |= 1 << 3; }

            x |= match comp {
                "0" => 0b0_1010_1000_0000,
                "1" => 0b0_1111_1100_0000,
                "-1" => 0b0_1110_1000_0000,
                "D" => 0b0_0011_0000_0000,
                "A" => 0b0_1100_0000_0000,
                "M" => 0b1_1100_0000_0000,
                "!D" => 0b0_0011_0100_0000,
                "!A" => 0b0_1100_0100_0000,
                "!M" => 0b1_1100_0100_0000,
                "-D" => 0b0_0011_1100_0000,
                "-A" => 0b0_1100_1100_0000,
                "-M" => 0b1_1100_1100_0000,
                "D+1" => 0b0_0111_1100_0000,
                "A+1" => 0b0_1101_1100_0000,
                "M+1" => 0b1_1101_1100_0000,
                "D-1" => 0b0_0011_1000_0000,
                "A-1" => 0b0_1100_1000_0000,
                "M-1" => 0b1_1100_1000_0000,
                "D+A" => 0b0_0000_0100_0000,
                "D+M" => 0b1_0000_0100_0000,
                "D-A" => 0b0_0100_1100_0000,
                "D-M" => 0b1_0100_1100_0000,
                "A-D" => 0b0_0001_1100_0000,
                "M-D" => 0b1_0001_1100_0000,
                "D&A" => 0b0_0000_0000_0000,
                "D&M" => 0b1_0000_0000_0000,
                "D|A" => 0b0_0101_0100_0000,
                "D|M" => 0b1_0101_0100_0000,
                _ => return Err("Error parsing instruction. (comp failed).")
            };

            x |= match jump {
                "" => 0b000,
                "JGT" => 0b001,
                "JEQ" => 0b010,
                "JGE" => 0b011,
                "JLT" => 0b100,
                "JNE" => 0b101,
                "JLE" => 0b110,
                "JMP" => 0b111,
                _ => return Err("Could not parse instruction. (jump failed).")
            };

            return CCommand::from_bytes(x);
        }
        Err("Could not parse command. (Regex Failed)")
    }

    fn from_bytes(x: u16) -> Result<CCommand, &'static str> {
        Ok(CCommand {
            a: (1 << 12) & x != 0,

            c1: (1 << 11) & x != 0,
            c2: (1 << 10) & x != 0,
            c3: (1 << 9) & x != 0,
            c4: (1 << 8) & x != 0,
            c5: (1 << 7) & x != 0,
            c6: (1 << 6) & x != 0,

            d1: (1 << 5) & x != 0,
            d2: (1 << 4) & x != 0,
            d3: (1 << 3) & x != 0,

            j1: (1 << 2) & x != 0,
            j2: (1 << 1) & x != 0,
            j3: 1 & x != 0
        })
    }

    fn to_bytes(&self) -> u16 {
        let mut bytes = 0b1110_0000_0000_0000;
        if self.a { bytes |= 1 << 12 }
        if self.c1 { bytes |= 1 << 11 }
        if self.c2 { bytes |= 1 << 10 }
        if self.c3 { bytes |= 1 << 9 }
        if self.c4 { bytes |= 1 << 8 }
        if self.c5 { bytes |= 1 << 7 }
        if self.c6 { bytes |= 1 << 6 }
        if self.d1 { bytes |= 1 << 5 }
        if self.d2 { bytes |= 1 << 4 }
        if self.d3 { bytes |= 1 << 3 }
        if self.j1 { bytes |= 1 << 2 }
        if self.j2 { bytes |= 1 << 1 }
        if self.j3 { bytes |= 1 << 0 }
        bytes
    }
}

#[cfg(test)]
mod tests {
}
