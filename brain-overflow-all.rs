use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{Zero, One};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
enum CpuWidth {
    Bit32,
    Bit64,
    Bit128,
    Bit256,
    Bit512,
    Bit1024,
    Custom(u32),
}

#[derive(Debug)]
struct CPU {
    registers: HashMap<String, BigUint>,
    bits: CpuWidth,
    flags: HashMap<String, bool>,
    memory: Vec<BigUint>,
    pc: BigUint,
}

impl CPU {
    fn new(bits: CpuWidth, reg_count: usize, mem_size: usize) -> Self {
        let mut registers = HashMap::new();
        for i in 0..reg_count {
            registers.insert(format!("R{}", i), BigUint::zero());
        }
        let mut flags = HashMap::new();
        for f in &["ZERO", "CARRY", "OVERFLOW", "SIGN"] {
            flags.insert(f.to_string(), false);
        }
        CPU {
            registers,
            bits,
            flags,
            memory: vec![BigUint::zero(); mem_size],
            pc: BigUint::zero(),
        }
    }

    fn mask(&self) -> BigUint {
        match self.bits {
            CpuWidth::Bit32 => (BigUint::one() << 32usize) - BigUint::one(),
            CpuWidth::Bit64 => (BigUint::one() << 64usize) - BigUint::one(),
            CpuWidth::Bit128 => (BigUint::one() << 128usize) - BigUint::one(),
            CpuWidth::Bit256 => (BigUint::one() << 256usize) - BigUint::one(),
            CpuWidth::Bit512 => (BigUint::one() << 512usize) - BigUint::one(),
            CpuWidth::Bit1024 => (BigUint::one() << 1024usize) - BigUint::one(),
            CpuWidth::Custom(n) => (BigUint::one() << n as usize) - BigUint::one(),
        }
    }

    fn to_masked(&self, value: &BigUint) -> BigUint {
        value & self.mask()
    }

    fn set_flag(&mut self, name: &str, value: bool) {
        if let Some(flag) = self.flags.get_mut(name) {
            *flag = value;
        }
    }

    fn add(&mut self, reg: &str, val: &BigUint) {
        let r_val = self.registers.get(reg).unwrap();
        let sum = r_val + val;
        self.registers.insert(reg.to_string(), self.to_masked(&sum));
        self.set_flag("ZERO", sum.is_zero());
    }

    fn sub(&mut self, reg: &str, val: &BigUint) {
        let r_val = self.registers.get(reg).unwrap();
        let result = if r_val >= val { r_val - val } else { BigUint::zero() };
        self.registers.insert(reg.to_string(), self.to_masked(&result));
        self.set_flag("ZERO", result.is_zero());
    }

    fn mul(&mut self, reg: &str, val: &BigUint) {
        let r_val = self.registers.get(reg).unwrap();
        let result = r_val * val;
        self.registers.insert(reg.to_string(), self.to_masked(&result));
        self.set_flag("ZERO", result.is_zero());
    }

    fn div(&mut self, reg: &str, val: &BigUint) {
        if val.is_zero() { return; }
        let r_val = self.registers.get(reg).unwrap();
        let result = r_val / val;
        self.registers.insert(reg.to_string(), self.to_masked(&result));
        self.set_flag("ZERO", result.is_zero());
    }

    fn bitwise_op(&mut self, reg: &str, val: &BigUint, op: &str) {
        let r_val = self.registers.get(reg).unwrap();
        let result = match op {
            "AND" => r_val & val,
            "OR" => r_val | val,
            "XOR" => r_val ^ val,
            _ => r_val.clone(),
        };
        self.registers.insert(reg.to_string(), self.to_masked(&result));
        self.set_flag("ZERO", result.is_zero());
    }

    fn shl(&mut self, reg: &str, bits: usize) {
        let r_val = self.registers.get(reg).unwrap();
        let result = r_val << bits;
        self.registers.insert(reg.to_string(), self.to_masked(&result));
    }

    fn shr(&mut self, reg: &str, bits: usize) {
        let r_val = self.registers.get(reg).unwrap();
        let result = r_val >> bits;
        self.registers.insert(reg.to_string(), self.to_masked(&result));
    }

    fn load(&mut self, reg: &str, addr: usize) {
        if addr < self.memory.len() {
            self.registers.insert(reg.to_string(), self.memory[addr].clone());
        }
    }

    fn store(&mut self, reg: &str, addr: usize) {
        if addr < self.memory.len() {
            let val = self.registers.get(reg).unwrap().clone();
            self.memory[addr] = self.to_masked(&val);
        }
    }

    fn print_state(&self) {
        println!("PC: {}", self.pc);
        for (k, v) in &self.registers {
            println!("{} = {}", k, v);
        }
        println!("FLAGS: {:?}", self.flags);
    }
}

fn parse_biguint(s: &str) -> BigUint {
    BigUint::parse_bytes(s.as_bytes(), 10).unwrap_or_else(|| BigUint::zero())
}

fn main() {
    let mut cpu = CPU::new(CpuWidth::Bit1024, 16, 1024);

    println!("Advanced CPU Simulator (RISC-V style, 1024-bit capable)");
    println!("Instructions: ADD, SUB, MUL, DIV, AND, OR, XOR, SHL, SHR, LOAD, STORE, EXIT");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0].to_uppercase().as_str() {
            "EXIT" => break,
            "ADD" => {
                let reg = parts[1];
                let val = parse_biguint(parts[2]);
                cpu.add(reg, &val);
            }
            "SUB" => {
                let reg = parts[1];
                let val = parse_biguint(parts[2]);
                cpu.sub(reg, &val);
            }
            "MUL" => {
                let reg = parts[1];
                let val = parse_biguint(parts[2]);
                cpu.mul(reg, &val);
            }
            "DIV" => {
                let reg = parts[1];
                let val = parse_biguint(parts[2]);
                cpu.div(reg, &val);
            }
            "AND" | "OR" | "XOR" => {
                let reg = parts[1];
                let val = parse_biguint(parts[2]);
                cpu.bitwise_op(reg, &val, parts[0].to_uppercase().as_str());
            }
            "SHL" => {
                let reg = parts[1];
                let bits: usize = parts[2].parse().unwrap_or(0);
                cpu.shl(reg, bits);
            }
            "SHR" => {
                let reg = parts[1];
                let bits: usize = parts[2].parse().unwrap_or(0);
                cpu.shr(reg, bits);
            }
            "LOAD" => {
                let reg = parts[1];
                let addr: usize = parts[2].parse().unwrap_or(0);
                cpu.load(reg, addr);
            }
            "STORE" => {
                let reg = parts[1];
                let addr: usize = parts[2].parse().unwrap_or(0);
                cpu.store(reg, addr);
            }
            "STATE" => cpu.print_state(),
            _ => println!("Unknown instruction"),
        }
    }
}
