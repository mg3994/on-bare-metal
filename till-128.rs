use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum CpuWidth {
    Bit32,
    Bit64,
    Bit128,
}

#[derive(Debug)]
struct CPU {
    registers: HashMap<String, u128>, // using u128 for max width
    bits: CpuWidth,
    flags: HashMap<String, bool>, // ZERO, CARRY, OVERFLOW, SIGN
    memory: Vec<u128>,
}

impl CPU {
    fn new(bits: CpuWidth, reg_count: usize, mem_size: usize) -> Self {
        let mut registers = HashMap::new();
        for i in 1..=reg_count {
            registers.insert(format!("R{}", i), 0);
        }
        let mut flags = HashMap::new();
        for f in &["ZERO", "CARRY", "OVERFLOW", "SIGN"] {
            flags.insert(f.to_string(), false);
        }
        CPU {
            registers,
            bits,
            flags,
            memory: vec![0; mem_size],
        }
    }

    fn mask(&self) -> u128 {
        match self.bits {
            CpuWidth::Bit32 => (1 << 32) - 1,
            CpuWidth::Bit64 => (1 << 64) - 1,
            CpuWidth::Bit128 => u128::MAX,
        }
    }

    fn to_masked(&self, value: u128) -> u128 {
        value & self.mask()
    }

    fn set_flag(&mut self, name: &str, value: bool) {
        if let Some(flag) = self.flags.get_mut(name) {
            *flag = value;
        }
    }

    fn get_value(&self, operand: &str) -> Result<u128, String> {
        if operand.starts_with('R') {
            self.registers.get(operand)
                .copied()
                .ok_or(format!("Register {} not found", operand))
        } else if operand.starts_with("0x") {
            u128::from_str_radix(&operand[2..], 16).map_err(|_| "Invalid hex".into())
        } else {
            operand.parse::<u128>().map_err(|_| "Invalid number".into())
        }
    }

    fn execute(&mut self, instruction: &str) -> Result<(), String> {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        if parts.is_empty() { return Err("Empty instruction".into()); }

        match parts[0].to_uppercase().as_str() {
            "MOV" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                self.registers.insert(reg.to_string(), self.to_masked(val));
            }
            "ADD" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let (res, carry) = self.registers[reg].overflowing_add(val);
                self.set_flag("CARRY", carry);
                self.set_flag("ZERO", res == 0);
                self.registers.insert(reg.to_string(), self.to_masked(res));
            }
            "SUB" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let (res, borrow) = self.registers[reg].overflowing_sub(val);
                self.set_flag("CARRY", borrow);
                self.set_flag("ZERO", res == 0);
                self.registers.insert(reg.to_string(), self.to_masked(res));
            }
            "MUL" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                let res = self.registers[reg].wrapping_mul(val);
                self.registers.insert(reg.to_string(), self.to_masked(res));
            }
            "DIV" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                if val == 0 { return Err("Division by zero".into()); }
                let res = self.registers[reg] / val;
                self.registers.insert(reg.to_string(), self.to_masked(res));
            }
            "AND" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                self.registers.insert(reg.to_string(), self.to_masked(self.registers[reg] & val));
            }
            "OR" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                self.registers.insert(reg.to_string(), self.to_masked(self.registers[reg] | val));
            }
            "XOR" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                self.registers.insert(reg.to_string(), self.to_masked(self.registers[reg] ^ val));
            }
            "NOT" => {
                let reg = parts[1];
                self.registers.insert(reg.to_string(), self.to_masked(!self.registers[reg]));
            }
            "SHL" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                self.registers.insert(reg.to_string(), self.to_masked(self.registers[reg] << val));
            }
            "SHR" => {
                let reg = parts[1].trim_end_matches(',');
                let val = self.get_value(parts[2])?;
                self.registers.insert(reg.to_string(), self.to_masked(self.registers[reg] >> val));
            }
            "LOAD" => {
                let reg = parts[1].trim_end_matches(',');
                let addr = self.get_value(parts[2])? as usize;
                self.registers.insert(reg.to_string(), self.memory.get(addr).copied().unwrap_or(0));
            }
            "STORE" => {
                let reg = parts[1].trim_end_matches(',');
                let addr = self.get_value(parts[2])? as usize;
                if addr >= self.memory.len() { return Err("Memory out of bounds".into()); }
                self.memory[addr] = self.registers[reg];
            }
            _ => return Err(format!("Unknown instruction: {}", parts[0])),
        }
        Ok(())
    }

    fn dump(&self) {
        println!("--- CPU Registers ({:?}) ---", self.bits);
        for (r, v) in &self.registers {
            println!("{} = {} (0x{:X}, 0b{:b})", r, v, v, v);
        }
        println!("--- Flags ---");
        for (f, val) in &self.flags {
            println!("{} = {}", f, val);
        }
    }
}
