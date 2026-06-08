//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*
//------------------C P U-------------------
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*
//------------R E G I S T E R S-------------
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 8;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

//special single bit flag registers for register f
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

//from separate flag registers to one 8bit register
impl std::convert::From<FlagRegisters> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

//from an 8bit register to separate flag registers 
impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION & 0b1) != 0;
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
        (self.a as u16) << 8 | self.f as u16
    }
    fn set_af(&self) -> u16 {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    fn set_bc(&self) -> u16 {
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
    SRA( )

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
            Instruction::SRA(target) => {
                match target {
                    R8Target::A => {
                        self.registers.a = self.sra(self.resgisters.a);
                    }
                    R8Target::B => {
                        self.registers.b = self.sra(self.resgisters.b);
                    }
                    R8Target::C => {
                        self.registers.c = self.sra(self.resgisters.c);
                    }
                    R8Target::D => {
                        self.registers.d = self.sra(self.resgisters.d);
                    }
                    R8Target::E => {
                        self.registers.e = self.sra(self.resgisters.e);
                    }
                    R8Target::L => {
                        self.registers.l = self.sra(self.resgisters.l);
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
                        self.registers.d = self.swap(self.registers.h);
                    }
                    R8Target::L => {
                        self.registers.l = self.swap(self.registers.l);
                    }

            _ => { /* TODO: support more instructions */}

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
        let new_value = value << 4;
        new_value = new_value | upper;
        //update flags
        self.registers.f.zero = self.registers.value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }

    fn sla(&mut self, value: u8) {
        let carry = value & 0x80;
        let new_value = value << 1;
        //update flags
        self.registers.f.zero = self.registers.value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 1;
        new_value
    }

    fn sra(&mut self, value: u8) {
        let carry = value & 0x01;
        let lmb = value & 0x80;
        let new_value = lmb | (value >> 1);
        //update flags
        self.registers.f.zero = self.registers.value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 1;
        new_value
    }
}


//*^*
fn main() {
    println!("gameboy emulator *^*");
}
