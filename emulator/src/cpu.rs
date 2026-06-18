//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*
//------------------C P U-------------------
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*

struct CPU {
    registers: Registers,
    pc: u16,
    //NOT IMPLEMENTED (rust will compain)
    //bus: MemoryBus,
}

//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*
//------------R E G I S T E R S-------------
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

//special single bit flag registers for register f
//implements thee copy trait so that shared reference FlagsRegister doesn't throw an error and a copy is automatically made
#[derive(Clone, Copy)]
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

//from a flag registers struct type to u8 type
impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

//from a u8 type to flag registers struct type
impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

//TODO add the other register operations: de and hl
impl Registers {
    fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (u8::from(self.f) as u16)
    }
    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
}

//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*
//---------I N S T R U C T I O N S----------
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*

enum Instruction {
    ADD(R8Target),
    SLA(R8Target),
    SWAP(R8Target),
    SRA(R8Target),
    RLC(R8Target),

}

enum R8Target {
    A, B, C, D, E, H, L,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                     R8Target::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => { /* TODO: support more targets */}
                }
            }
            Instruction::RLC(target) => {
                match target {
                    R8Target::A => {
                        self.registers.a = self.rlc(self.registers.a);
                    }
                    R8Target::B => {
                        self.registers.b = self.rlc(self.registers.b);
                    }
                    R8Target::C => {
                        self.registers.c = self.rlc(self.registers.c);
                    }
                    R8Target::D => {
                        self.registers.d = self.rlc(self.registers.d);
                    }
                    R8Target::E => {
                        self.registers.e = self.rlc(self.registers.e);
                    }
                    R8Target::H => {
                        self.registers.h = self.rlc(self.registers.h);
                    }
                    R8Target::L => {
                        self.registers.l = self.rlc(self.registers.l);
                    }
                }
            }
            Instruction::SRA(target) => {
                match target {
                    R8Target::A => {
                        self.registers.a = self.sra(self.registers.a);
                    }
                    R8Target::B => {
                        self.registers.b = self.sra(self.registers.b);
                    }
                    R8Target::C => {
                        self.registers.c = self.sra(self.registers.c);
                    }
                    R8Target::D => {
                        self.registers.d = self.sra(self.registers.d);
                    }
                    R8Target::E => {
                        self.registers.e = self.sra(self.registers.e);
                    }
                    R8Target::H => {
                        self.registers.h = self.sra(self.registers.h);
                    }
                    R8Target::L => {
                        self.registers.l = self.sra(self.registers.l);
                    }
                }
            }
            Instruction::SLA(target) => {
                match target {
                    R8Target::A => {
                        self.registers.a = self.sla(self.registers.a);
                    }
                    R8Target::B => {
                        self.registers.b = self.sla(self.registers.b);
                    }
                    R8Target::C => {
                        self.registers.c = self.sla(self.registers.c);
                    }
                    R8Target::D => {
                        self.registers.d = self.sla(self.registers.d);
                    }
                    R8Target::E => {
                        self.registers.e = self.sla(self.registers.e);
                    }
                    R8Target::H => {
                        self.registers.h = self.sla(self.registers.h);
                    }
                    R8Target::L => {
                        self.registers.l = self.sla(self.registers.l);
                    }
                }
            }
            Instruction::SWAP(target) => {
                match target {
                    R8Target::A => {
                        self.registers.a = self.swap(self.registers.a);
                    }
                    R8Target::B => {
                        self.registers.b = self.swap(self.registers.b); 
                    }
                    R8Target::C => {
                        self.registers.c = self.swap(self.registers.c);
                    }
                    R8Target::D => {
                        self.registers.d = self.swap(self.registers.d);
                    }
                    R8Target::E => {
                        self.registers.e = self.swap(self.registers.e);
                    }
                    R8Target::H => { 
                        self.registers.h = self.swap(self.registers.h);
                    }
                    R8Target::L => {
                        self.registers.l = self.swap(self.registers.l);
                    }

            _ => { /* TODO: support more instructions */}
                }
            }
        }
    }
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;
        new_value
    }

    fn swap(&mut self, value: u8) -> u8{
        let upper = (value & 0xF0) >> 4;
        let new_value = (value << 4) | upper;
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }

    fn sla(&mut self, value: u8) -> u8{
        let carry = value & 0x80;
        let new_value = value << 1;
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 1;
        new_value
    }

    fn sra(&mut self, value: u8) -> u8{
        let carry = value & 0x01;
        let lmb = value & 0x80;
        let new_value = lmb | (value >> 1);
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 1;
        new_value
    }

    fn rlc(&mut self, value: u8) -> u8{
        let carry = value & 0x80;
        let new_value = (value << 1) | carry;
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 1;
        new_value
    }

    fn rrc(&mut self, value: u8) -> u8{
        let carry = value & 0x01;
        let new_value = (value >> 1) | carry;
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 1;
        new_value
    }

    fn rl(&mut self, value: u8) -> u8{
        let carry = value & 0x80;
        let carry_flag: u8 = self.registers.f.carry as u8;
        let new_value = (value << 1) | carry_flag;
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 1;
        new_value
    }

    fn rr(&mut self, value: u8) -> u8{
        let carry = value & 0x01;
        let carry_flag: u8 = (self.registers.f.carry as u8) << 7;
        let new_value = (value >> 1) | carry_flag;
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 1;
        new_value
    }

    fn srl(&mut self, value: u8) -> u8 {
        let carry = value & 0x01;
        let new_value = value >> 1;
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 1;
        new_value
    }

    fn set(&mut self, value: u8, bit: u8) -> u8{
        if bit > 7 {
            panic!("bit placement is out of bounds (high). CPU FN SET")
        }
        else if bit < 0 {
            panic!("bit placement is out of bounds (low). CPU FN SET")
        }
        let new_value = value | (0x01 << bit);
        //flags remain unaffected
        new_value
    }

    fn res(&mut self, value: u8, bit: u8) -> u8{
        if bit > 7 {
            panic!("bit placement is out of bounds (high). CPU FN RES")
        } 
        else if bit < 0 {
            panic!("bit placement is out of counds (low). CPU FN RES")
        }
        let mut mask = 0xFE;
        for _i in 1.. bit {
            mask = (mask << 1) | 0x01;
        }
        let new_value = value & mask;
        new_value
    }

    fn bit(&mut self, value: u8, bit: u8) {
        if bit > 7 {
            panic!("bit to check is out of bounds (high). CPU FN BIT")
        }
        else if bit < 0 {
            panic!("bit to check is out of bounds (low). CPU FN BIT")
        }
        let mask = 0x01 << bit;
        let result = value & mask;
        //update flags
        if result == mask {
            self.registers.f.zero = false; 
        }else {
            self.registers.f.zero = true;
        }
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        //the carry flag is unaffected
    }

    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        //update flags
        //carry and zero flag are unaffected
        self.registers.f.subtract = true;
        self.registers.f.half_carry = false;
    }

    fn rlca(&mut self) {
        let carry_copy = (self.registers.a & 0x80) >> 7;
        self.registers.a = (self.registers.a << 1) | carry_copy;
        //update flags
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry_copy == 1;
    }

    fn rrca(&mut self) {
        let carry_copy = self.registers.a & 0x01;
        self.registers.a = (self.registers.a >> 1) | (carry_copy << 7);
        //update flags
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry_copy == 1;
    }
}


//*^*
fn main() {
    println!("gameboy emulator *^*");
}
