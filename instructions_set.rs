// 8 bit
use std::collections::HashMap;

#[derive(Debug)]
struct CPU {
    registers: HashMap<String, i32>,
    bits: u32,
}

impl CPU {
    fn new(bits: u32) -> Self {
        let mut registers = HashMap::new();
        for i in 1..=8 {
            registers.insert(format!("R{}", i), 0);
        }
        CPU { registers, bits }
    }

    fn mask(&self) -> i32 {
        (1 << self.bits) - 1
    }

    fn to_twos_complement(&self, value: i32) -> i32 {
        value & self.mask()
    }

    fn get_value(&self, operand: &str) -> Result<i32, String> {
        if operand.starts_with('R') {
            self.registers.get(operand)
                .copied()
                .ok_or(format!("Register {} not found", operand))
        } else {
            operand.parse::<i32>()
                .map_err(|_| format!("Invalid immediate value: {}", operand))
        }
    }

    fn execute(&mut self, instruction: &str) -> Result<(), String> {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        if parts.is_empty() { return Err("Empty instruction".into()); }

        match parts[0].to_uppercase().as_str() {
            "MOV" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                self.registers.insert(reg.to_string(), self.to_twos_complement(val));
            }
            "ADD" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let result = self.to_twos_complement(self.registers[reg] + val);
                self.registers.insert(reg.to_string(), result);
            }
            "SUB" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let result = self.to_twos_complement(self.registers[reg] - val);
                self.registers.insert(reg.to_string(), result);
            }
            "MUL" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let result = self.to_twos_complement(self.registers[reg] * val);
                self.registers.insert(reg.to_string(), result);
            }
            "DIV" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                if val == 0 { return Err("Division by zero".into()); }
                let result = self.to_twos_complement(self.registers[reg] / val);
                self.registers.insert(reg.to_string(), result);
            }
            "AND" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let result = self.to_twos_complement(self.registers[reg] & val);
                self.registers.insert(reg.to_string(), result);
            }
            "OR" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let result = self.to_twos_complement(self.registers[reg] | val);
                self.registers.insert(reg.to_string(), result);
            }
            "XOR" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let result = self.to_twos_complement(self.registers[reg] ^ val);
                self.registers.insert(reg.to_string(), result);
            }
            "NOT" => {
                let reg = parts[1];
                let result = self.to_twos_complement(!self.registers[reg]);
                self.registers.insert(reg.to_string(), result);
            }
            "SHL" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let result = self.to_twos_complement(self.registers[reg] << val);
                self.registers.insert(reg.to_string(), result);
            }
            "SHR" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let result = self.to_twos_complement((self.registers[reg] as u32 >> val) as i32);
                self.registers.insert(reg.to_string(), result);
            }
            _ => return Err(format!("Unknown instruction: {}", parts[0])),
        }
        Ok(())
    }

    fn dump(&self) {
        println!("--- CPU Registers ({}-bit) ---", self.bits);
        for i in 1..=8 {
            let r = format!("R{}", i);
            let val = self.registers[&r];
            println!("{} = {} (0b{:0width$b}, 0x{:X})", r, val, val, val, width=self.bits as usize);
        }
        println!("------------------------------");
    }
}
fn main() {
    let mut cpu = CPU::new(8);  // 8-bit CPU

    cpu.execute("MOV R1, 10").unwrap();
    cpu.execute("MOV R2, 250").unwrap();
    cpu.execute("ADD R1, R2").unwrap();  // overflow
    cpu.execute("SHL R1, 2").unwrap();
    cpu.execute("NOT R2").unwrap();
    cpu.execute("AND R1, R2").unwrap();

    cpu.dump();
}
