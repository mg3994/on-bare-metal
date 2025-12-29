use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{Zero, One};
use std::collections::HashMap;

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
            CpuWidth::Bit32 => BigUint::from(1u32) << 32usize - BigUint::one(),
            CpuWidth::Bit64 => BigUint::from(1u64) << 64usize - BigUint::one(),
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
        // Note: CARRY and OVERFLOW for BigUint would require extra logic
    }

    fn sub(&mut self, reg: &str, val: &BigUint) {
        let r_val = self.registers.get(reg).unwrap();
        let result = if r_val > val { r_val - val } else { BigUint::zero() };
        self.registers.insert(reg.to_string(), self.to_masked(&result));
        self.set_flag("ZERO", result.is_zero());
    }

    // Similarly implement MUL, DIV, AND, OR, XOR, SHL, SHR, LOAD, STORE
}
