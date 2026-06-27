//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*
//------------------C P U-------------------
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*
//-----------M E M O R Y - B U S------------
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }


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
    ADD(ArithmeticTarget),
    SLA(DeRefR8Target),
    SWAP(DeRefR8Target),
    SRA(DeRefR8Target),
    RLC(DeRefR8Target),
    RRC(DeRefR8Target),
    RL(DeRefR8Target),
    RR(DeRefR8Target),
    SRL(DeRefR8Target),
    SET(BitManTarget),
    RES(BitManTarget),
    BIT(BitManTarget),
    CPL,
    RLCA,
    RRCA,

}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> { 
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte{
            0x00 => todo!(),
            0x01 => todo!(),
            0x02 => todo!(),
            0x03 => todo!(),
            0x04 => todo!(),
            0x05 => todo!(),
            0x06 => todo!(),
            0x07 => Some(Instruction::RLCA),
            0x08 => todo!(),
            0x09 => todo!(),
            0x0A => todo!(),
            0x0B => todo!(),
            0x0C => todo!(),
            0x0D => todo!(),
            0x0E => todo!(),
            0x0F => Some(Instruction::RRCA),
            0x10 => todo!(),
            0x11 => todo!(),
            0x12 => todo!(),
            0x13 => todo!(),
            0x14 => todo!(),
            0x15 => todo!(),
            0x16 => todo!(),
            0x17 => todo!(),
            0x18 => todo!(),
            0x19 => todo!(),
            0x1A => todo!(),
            0x1B => todo!(),
            0x1C => todo!(),
            0x1D => todo!(),
            0x1E => todo!(),
            0x1F => todo!(),
            0x20 => todo!(),
            0x21 => todo!(),
            0x22 => todo!(),
            0x23 => todo!(),
            0x24 => todo!(),
            0x25 => todo!(),
            0x26 => todo!(),
            0x27 => todo!(),
            0x28 => todo!(), 
            0x29 => todo!(),
            0x2A => todo!(),
            0x2B => todo!(),
            0x2C => todo!(),
            0x2D => todo!(),
            0x2E => todo!(),
            0x2F => Some(Instruction::CPL),
            0x30 => todo!(),
            0x31 => todo!(),
            0x32 => todo!(),
            0x33 => todo!(),
            0x34 => todo!(),
            0x35 => todo!(),
            0x36 => todo!(),
            0x37 => todo!(),
            0x38 => todo!(),
            0x39 => todo!(),
            0x3A => todo!(),
            0x3B => todo!(),
            0x3C => todo!(),
            0x3D => todo!(),
            0x3E => todo!(),
            0x3F => todo!(),
            0x40 => todo!(),
            0x41 => todo!(),
            0x42 => todo!(),
            0x43 => todo!(),
            0x44 => todo!(),
            0x45 => todo!(),
            0x46 => todo!(),
            0x47 => todo!(),
            0x48 => todo!(),
            0x49 => todo!(),
            0x4A => todo!(),
            0x4B => todo!(),
            0x4C => todo!(),
            0x4D => todo!(),
            0x4E => todo!(),
            0x4F => todo!(),
            0x50 => todo!(),
            0x51 => todo!(),
            0x52 => todo!(),
            0x53 => todo!(),
            0x54 => todo!(),
            0x55 => todo!(),
            0x56 => todo!(),
            0x57 => todo!(),
            0x58 => todo!(),
            0x59 => todo!(),
            0x5A => todo!(),
            0x5B => todo!(),
            0x5C => todo!(),
            0x5D => todo!(),
            0x5E => todo!(),
            0x5F => todo!(),
            0x60 => todo!(),
            0x61 => todo!(),
            0x62 => todo!(),
            0x63 => todo!(),
            0x64 => todo!(), 
            0x65 => todo!(),
            0x66 => todo!(),
            0x67 => todo!(),
            0x68 => todo!(),
            0x69 => todo!(),
            0x6A => todo!(),
            0x6B => todo!(),
            0x6C => todo!(),
            0x6D => todo!(),
            0x6E => todo!(),
            0x6F => todo!(),
            0x70 => todo!(),
            0x71 => todo!(),
            0x72 => todo!(),
            0x73 => todo!(),
            0x74 => todo!(),
            0x75 => todo!(),
            0x76 => todo!(),
            0x77 => todo!(),
            0x78 => todo!(),
            0x79 => todo!(),
            0x7A => todo!(),
            0x7B => todo!(),
            0x7C => todo!(),
            0x7D => todo!(),
            0x7E => todo!(),
            0x7F => todo!(),
            0x80 => todo!(),
            0x81 => todo!(),
            0x82 => todo!(),
            0x83 => todo!(),
            0x84 => todo!(),
            0x85 => todo!(),
            0x86 => todo!(),
            0x87 => todo!(),
            0x88 => todo!(), 
            0x89 => todo!(),
            0x8A => todo!(),
            0x8B => todo!(),
            0x8C => todo!(),
            0x8D => todo!(),
            0x8E => todo!(),
            0x8F => todo!(),
            0x90 => todo!(),
            0x91 => todo!(),
            0x92 => todo!(),
            0x93 => todo!(),
            0x94 => todo!(),
            0x95 => todo!(),
            0x96 => todo!(),
            0x97 => todo!(),
            0x98 => todo!(),
            0x99 => todo!(),
            0x9A => todo!(),
            0x9B => todo!(),
            0x9C => todo!(),
            0x9D => todo!(),
            0x9E => todo!(),
            0x9F => todo!(),
            0xA0 => todo!(),
            0xA1 => todo!(),
            0xA2 => todo!(),
            0xA3 => todo!(),
            0xA4 => todo!(),
            0xA5 => todo!(),
            0xA6 => todo!(),
            0xA7 => todo!(),
            0xA8 => todo!(),
            0xA9 => todo!(),
            0xAA => todo!(),
            0xAB => todo!(),
            0xAC => todo!(),
            0xAD => todo!(),
            0xAE => todo!(),
            0xAF => todo!(),
            0xB0 => todo!(),
            0xB1 => todo!(),
            0xB2 => todo!(),
            0xB3 => todo!(),
            0xB4 => todo!(),
            0xB5 => todo!(),
            0xB6 => todo!(),
            0xB7 => todo!(),
            0xB8 => todo!(),
            0xB9 => todo!(),
            0xBA => todo!(),
            0xBB => todo!(),
            0xBC => todo!(),
            0xBD => todo!(),
            0xBE => todo!(),
            0xBF => todo!(),
            0xC0 => todo!(),
            0xC1 => todo!(),
            0xC2 => todo!(),
            0xC3 => todo!(),
            0xC4 => todo!(),
            0xC5 => todo!(),
            0xC6 => todo!(),
            0xC7 => todo!(),
            0xC8 => todo!(),
            0xC9 => todo!(),
            0xCA => todo!(),
            0xCB => todo!(),
            0xCC => todo!(),
            0xCD => todo!(),
            0xCE => todo!(),
            0xCF => todo!(),
            0xD0 => todo!(),
            0xD1 => todo!(),
            0xD2 => todo!(),
            0xD3 => todo!(),
            0xD4 => todo!(),
            0xD5 => todo!(),
            0xD6 => todo!(),
            0xD7 => todo!(),
            0xD8 => todo!(),
            0xD9 => todo!(), 
            0xDA => todo!(),
            0xDB => todo!(),
            0xDC => todo!(),
            0xDD => todo!(),
            0xDE => todo!(),
            0xDF => todo!(),
            0xE0 => todo!(),
            0xE1 => todo!(),
            0xE2 => todo!(),
            0xE3 => todo!(),
            0xE4 => todo!(),
            0xE5 => todo!(),
            0xE6 => todo!(),
            0xE7 => todo!(),
            0xE8 => todo!(),
            0xE9 => todo!(),
            0xEA => todo!(),
            0xEB => todo!(),
            0xEC => todo!(),
            0xED => todo!(),
            0xEE => todo!(),
            0xEF => todo!(),
            0xF0 => todo!(),
            0xF1 => todo!(),
            0xF2 => todo!(),
            0xF3 => todo!(),
            0xF4 => todo!(),
            0xF5 => todo!(),
            0xF6 => todo!(),
            0xF7 => todo!(),
            0xF8 => todo!(),
            0xF9 => todo!(),
            0xFA => todo!(),
            0xFB => todo!(),
            0xFC => todo!(),
            0xFD => todo!(),
            0xFE => todo!(),
            0xFF => todo!(),
            _ => panic!("opcode not found! :<")
        }       
    }


    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte{
            0x00 => Some(Instruction::RLC(DeRefR8Target::B)),
            0x01 => Some(Instruction::RLC(DeRefR8Target::C)),
            0x02 => Some(Instruction::RLC(DeRefR8Target::D)),
            0x03 => Some(Instruction::RLC(DeRefR8Target::E)),
            0x04 => Some(Instruction::RLC(DeRefR8Target::H)),
            0x05 => Some(Instruction::RLC(DeRefR8Target::L)),
            0x06 => Some(Instruction::RLC(DeRefR8Target::HL)),
            0x07 => Some(Instruction::RLC(DeRefR8Target::A)),
            0x08 => Some(Instruction::RRC(DeRefR8Target::B)),
            0x09 => Some(Instruction::RRC(DeRefR8Target::C)),
            0x0A => Some(Instruction::RRC(DeRefR8Target::D)),
            0x0B => Some(Instruction::RRC(DeRefR8Target::E)),
            0x0C => Some(Instruction::RRC(DeRefR8Target::H)),
            0x0D => Some(Instruction::RRC(DeRefR8Target::L)),
            0x0E => Some(Instruction::RRC(DeRefR8Target::HL)),
            0x0F => Some(Instruction::RRC(DeRefR8Target::A)),
            0x10 => Some(Instruction::RL(DeRefR8Target::B)),
            0x11 => Some(Instruction::RL(DeRefR8Target::C)),
            0x12 => Some(Instruction::RL(DeRefR8Target::D)),
            0x13 => Some(Instruction::RL(DeRefR8Target::E)),
            0x14 => Some(Instruction::RL(DeRefR8Target::H)),
            0x15 => Some(Instruction::RL(DeRefR8Target::L)),
            0x16 => Some(Instruction::RL(DeRefR8Target::HL)),
            0x17 => Some(Instruction::RL(DeRefR8Target::A)),
            0x18 => Some(Instruction::RR(DeRefR8Target::B)),
            0x19 => Some(Instruction::RR(DeRefR8Target::C)),
            0x1A => Some(Instruction::RR(DeRefR8Target::D)),
            0x1B => Some(Instruction::RR(DeRefR8Target::E)),
            0x1C => Some(Instruction::RR(DeRefR8Target::H)),
            0x1D => Some(Instruction::RR(DeRefR8Target::L)),
            0x1E => Some(Instruction::RR(DeRefR8Target::HL)),
            0x1F => Some(Instruction::RR(DeRefR8Target::A)),
            0x20 => Some(Instruction::SLA(DeRefR8Target::B)),
            0x21 => Some(Instruction::SLA(DeRefR8Target::C)),
            0x22 => Some(Instruction::SLA(DeRefR8Target::D)),
            0x23 => Some(Instruction::SLA(DeRefR8Target::E)),
            0x24 => Some(Instruction::SLA(DeRefR8Target::H)),
            0x25 => Some(Instruction::SLA(DeRefR8Target::L)),
            0x26 => Some(Instruction::SLA(DeRefR8Target::HL)),
            0x27 => Some(Instruction::SLA(DeRefR8Target::A)),
            0x28 => Some(Instruction::SRA(DeRefR8Target::B)), 
            0x29 => Some(Instruction::SRA(DeRefR8Target::C)),
            0x2A => Some(Instruction::SRA(DeRefR8Target::D)),
            0x2B => Some(Instruction::SRA(DeRefR8Target::E)),
            0x2C => Some(Instruction::SRA(DeRefR8Target::H)),
            0x2D => Some(Instruction::SRA(DeRefR8Target::L)),
            0x2E => Some(Instruction::SRA(DeRefR8Target::HL)),
            0x2F => Some(Instruction::SRA(DeRefR8Target::A)),
            0x30 => Some(Instruction::SWAP(DeRefR8Target::B)),
            0x31 => Some(Instruction::SWAP(DeRefR8Target::C)),
            0x32 => Some(Instruction::SWAP(DeRefR8Target::D)),
            0x33 => Some(Instruction::SWAP(DeRefR8Target::E)),
            0x34 => Some(Instruction::SWAP(DeRefR8Target::H)),
            0x35 => Some(Instruction::SWAP(DeRefR8Target::L)),
            0x36 => Some(Instruction::SWAP(DeRefR8Target::HL)),
            0x37 => Some(Instruction::SWAP(DeRefR8Target::A)),
            0x38 => Some(Instruction::SRL(DeRefR8Target::B)),
            0x39 => Some(Instruction::SRL(DeRefR8Target::C)),
            0x3A => Some(Instruction::SRL(DeRefR8Target::D)),
            0x3B => Some(Instruction::SRL(DeRefR8Target::E)),
            0x3C => Some(Instruction::SRL(DeRefR8Target::H)),
            0x3D => Some(Instruction::SRL(DeRefR8Target::L)),
            0x3E => Some(Instruction::SRL(DeRefR8Target::HL)),
            0x3F => Some(Instruction::SRL(DeRefR8Target::A)),
            0x40 => Some(Instruction::BIT(BitManTarget::B(0))),
            0x41 => Some(Instruction::BIT(BitManTarget::C(0))),
            0x42 => Some(Instruction::BIT(BitManTarget::D(0))),
            0x43 => Some(Instruction::BIT(BitManTarget::E(0))),
            0x44 => Some(Instruction::BIT(BitManTarget::H(0))),
            0x45 => Some(Instruction::BIT(BitManTarget::L(0))),
            0x46 => Some(Instruction::BIT(BitManTarget::HL(0))),
            0x47 => Some(Instruction::BIT(BitManTarget::A(0))),
            0x48 => Some(Instruction::BIT(BitManTarget::B(1))),
            0x49 => Some(Instruction::BIT(BitManTarget::C(1))),
            0x4A => Some(Instruction::BIT(BitManTarget::D(1))),
            0x4B => Some(Instruction::BIT(BitManTarget::E(1))),
            0x4C => Some(Instruction::BIT(BitManTarget::H(1))),
            0x4D => Some(Instruction::BIT(BitManTarget::H(1))),
            0x4E => Some(Instruction::BIT(BitManTarget::HL(1))),
            0x4F => Some(Instruction::BIT(BitManTarget::A(1))),
            0x50 => Some(Instruction::BIT(BitManTarget::B(2))),
            0x51 => Some(Instruction::BIT(BitManTarget::C(2))),
            0x52 => Some(Instruction::BIT(BitManTarget::D(2))),
            0x53 => Some(Instruction::BIT(BitManTarget::E(2))),
            0x54 => Some(Instruction::BIT(BitManTarget::H(2))),
            0x55 => Some(Instruction::BIT(BitManTarget::H(2))),
            0x56 => Some(Instruction::BIT(BitManTarget::HL(2))),
            0x57 => Some(Instruction::BIT(BitManTarget::A(2))),
            0x58 => Some(Instruction::BIT(BitManTarget::B(3))),
            0x59 => Some(Instruction::BIT(BitManTarget::C(3))),
            0x5A => Some(Instruction::BIT(BitManTarget::D(3))),
            0x5B => Some(Instruction::BIT(BitManTarget::E(3))),
            0x5C => Some(Instruction::BIT(BitManTarget::H(3))),
            0x5D => Some(Instruction::BIT(BitManTarget::H(3))),
            0x5E => Some(Instruction::BIT(BitManTarget::HL(3))),
            0x5F => Some(Instruction::BIT(BitManTarget::A(3))),
            0x60 => Some(Instruction::BIT(BitManTarget::B(4))),
            0x61 => Some(Instruction::BIT(BitManTarget::C(4))),
            0x62 => Some(Instruction::BIT(BitManTarget::D(4))),
            0x63 => Some(Instruction::BIT(BitManTarget::E(4))),
            0x64 => Some(Instruction::BIT(BitManTarget::H(4))),
            0x65 => Some(Instruction::BIT(BitManTarget::H(4))),
            0x66 => Some(Instruction::BIT(BitManTarget::HL(4))),
            0x67 => Some(Instruction::BIT(BitManTarget::A(4))),
            0x68 => Some(Instruction::BIT(BitManTarget::B(5))),
            0x69 => Some(Instruction::BIT(BitManTarget::C(5))),
            0x6A => Some(Instruction::BIT(BitManTarget::D(5))),
            0x6B => Some(Instruction::BIT(BitManTarget::E(5))),
            0x6C => Some(Instruction::BIT(BitManTarget::H(5))),
            0x6D => Some(Instruction::BIT(BitManTarget::H(5))),
            0x6E => Some(Instruction::BIT(BitManTarget::HL(6))),
            0x6F => Some(Instruction::BIT(BitManTarget::A(5))),
            0x70 => Some(Instruction::BIT(BitManTarget::B(6))),
            0x71 => Some(Instruction::BIT(BitManTarget::C(6))),
            0x72 => Some(Instruction::BIT(BitManTarget::D(6))),
            0x73 => Some(Instruction::BIT(BitManTarget::E(6))),
            0x74 => Some(Instruction::BIT(BitManTarget::H(6))),
            0x75 => Some(Instruction::BIT(BitManTarget::H(6))),
            0x76 => Some(Instruction::BIT(BitManTarget::HL(6))),
            0x77 => Some(Instruction::BIT(BitManTarget::A(6))),
            0x78 => Some(Instruction::BIT(BitManTarget::B(7))),
            0x79 => Some(Instruction::BIT(BitManTarget::C(7))),
            0x7A => Some(Instruction::BIT(BitManTarget::D(7))),
            0x7B => Some(Instruction::BIT(BitManTarget::E(7))),
            0x7C => Some(Instruction::BIT(BitManTarget::H(7))),
            0x7D => Some(Instruction::BIT(BitManTarget::H(7))),
            0x7E => Some(Instruction::BIT(BitManTarget::HL(7))),
            0x7F => Some(Instruction::BIT(BitManTarget::A(7))),
            0x80 => Some(Instruction::RES(BitManTarget::B(0))),
            0x81 => Some(Instruction::RES(BitManTarget::C(0))),
            0x82 => Some(Instruction::RES(BitManTarget::D(0))),
            0x83 => Some(Instruction::RES(BitManTarget::E(0))),
            0x84 => Some(Instruction::RES(BitManTarget::H(0))),
            0x85 => Some(Instruction::RES(BitManTarget::L(0))),
            0x86 => Some(Instruction::RES(BitManTarget::HL(0))),
            0x87 => Some(Instruction::RES(BitManTarget::A(0))),
            0x88 => Some(Instruction::RES(BitManTarget::B(1))), 
            0x89 => Some(Instruction::RES(BitManTarget::C(1))),
            0x8A => Some(Instruction::RES(BitManTarget::D(1))),
            0x8B => Some(Instruction::RES(BitManTarget::E(1))),
            0x8C => Some(Instruction::RES(BitManTarget::H(1))),
            0x8D => Some(Instruction::RES(BitManTarget::L(1))),
            0x8E => Some(Instruction::RES(BitManTarget::HL(1))),
            0x8F => Some(Instruction::RES(BitManTarget::A(1))),
            0x90 => Some(Instruction::RES(BitManTarget::B(2))),
            0x91 => Some(Instruction::RES(BitManTarget::C(2))),
            0x92 => Some(Instruction::RES(BitManTarget::D(2))),
            0x93 => Some(Instruction::RES(BitManTarget::E(2))),
            0x94 => Some(Instruction::RES(BitManTarget::H(2))),
            0x95 => Some(Instruction::RES(BitManTarget::L(2))),
            0x96 => Some(Instruction::RES(BitManTarget::HL(2))),
            0x97 => Some(Instruction::RES(BitManTarget::A(2))),
            0x98 => Some(Instruction::RES(BitManTarget::B(3))),
            0x99 => Some(Instruction::RES(BitManTarget::C(3))),
            0x9A => Some(Instruction::RES(BitManTarget::D(3))),
            0x9B => Some(Instruction::RES(BitManTarget::E(3))),
            0x9C => Some(Instruction::RES(BitManTarget::H(3))),
            0x9D => Some(Instruction::RES(BitManTarget::L(3))),
            0x9E => Some(Instruction::RES(BitManTarget::HL(3))),
            0x9F => Some(Instruction::RES(BitManTarget::A(3))),
            0xA0 => Some(Instruction::RES(BitManTarget::B(4))),
            0xA1 => Some(Instruction::RES(BitManTarget::C(4))),
            0xA2 => Some(Instruction::RES(BitManTarget::D(4))),
            0xA3 => Some(Instruction::RES(BitManTarget::E(4))),
            0xA4 => Some(Instruction::RES(BitManTarget::H(4))),
            0xA5 => Some(Instruction::RES(BitManTarget::L(4))),
            0xA6 => Some(Instruction::RES(BitManTarget::HL(4))),
            0xA7 => Some(Instruction::RES(BitManTarget::A(4))),
            0xA8 => Some(Instruction::RES(BitManTarget::B(5))),
            0xA9 => Some(Instruction::RES(BitManTarget::C(5))),
            0xAA => Some(Instruction::RES(BitManTarget::D(5))),
            0xAB => Some(Instruction::RES(BitManTarget::E(5))),
            0xAC => Some(Instruction::RES(BitManTarget::H(5))),
            0xAD => Some(Instruction::RES(BitManTarget::L(5))),
            0xAE => Some(Instruction::RES(BitManTarget::HL(5))),
            0xAF => Some(Instruction::RES(BitManTarget::A(5))),
            0xB0 => Some(Instruction::RES(BitManTarget::B(6))),
            0xB1 => Some(Instruction::RES(BitManTarget::C(6))),
            0xB2 => Some(Instruction::RES(BitManTarget::D(6))),
            0xB3 => Some(Instruction::RES(BitManTarget::E(6))),
            0xB4 => Some(Instruction::RES(BitManTarget::H(6))),
            0xB5 => Some(Instruction::RES(BitManTarget::L(6))),
            0xB6 => Some(Instruction::RES(BitManTarget::HL(6))),
            0xB7 => Some(Instruction::RES(BitManTarget::A(6))),
            0xB8 => Some(Instruction::RES(BitManTarget::B(7))),
            0xB9 => Some(Instruction::RES(BitManTarget::C(7))),
            0xBA => Some(Instruction::RES(BitManTarget::D(7))),
            0xBB => Some(Instruction::RES(BitManTarget::E(7))),
            0xBC => Some(Instruction::RES(BitManTarget::H(7))),
            0xBD => Some(Instruction::RES(BitManTarget::L(7))),
            0xBE => Some(Instruction::RES(BitManTarget::HL(7))),
            0xBF => Some(Instruction::RES(BitManTarget::A(7))),
            0xC0 => Some(Instruction::SET(BitManTarget::B(0))),
            0xC1 => Some(Instruction::SET(BitManTarget::C(0))),
            0xC2 => Some(Instruction::SET(BitManTarget::D(0))),
            0xC3 => Some(Instruction::SET(BitManTarget::E(0))),
            0xC4 => Some(Instruction::SET(BitManTarget::H(0))),
            0xC5 => Some(Instruction::SET(BitManTarget::H(0))),
            0xC6 => Some(Instruction::SET(BitManTarget::HL(0))),
            0xC7 => Some(Instruction::SET(BitManTarget::A(0))),
            0xC8 => Some(Instruction::SET(BitManTarget::B(1))),
            0xC9 => Some(Instruction::SET(BitManTarget::C(1))),
            0xCA => Some(Instruction::SET(BitManTarget::D(1))),
            0xCB => Some(Instruction::SET(BitManTarget::E(1))),
            0xCC => Some(Instruction::SET(BitManTarget::H(1))),
            0xCD => Some(Instruction::SET(BitManTarget::H(1))),
            0xCE => Some(Instruction::SET(BitManTarget::HL(1))),
            0xCF => Some(Instruction::SET(BitManTarget::A(1))),
            0xD0 => Some(Instruction::SET(BitManTarget::B(2))),
            0xD1 => Some(Instruction::SET(BitManTarget::C(2))),
            0xD2 => Some(Instruction::SET(BitManTarget::D(2))),
            0xD3 => Some(Instruction::SET(BitManTarget::E(2))),
            0xD4 => Some(Instruction::SET(BitManTarget::H(2))),
            0xD5 => Some(Instruction::SET(BitManTarget::H(2))),
            0xD6 => Some(Instruction::SET(BitManTarget::HL(2))),
            0xD7 => Some(Instruction::SET(BitManTarget::A(2))),
            0xD8 => Some(Instruction::SET(BitManTarget::B(3))),
            0xD9 => Some(Instruction::SET(BitManTarget::C(3))),
            0xDA => Some(Instruction::SET(BitManTarget::D(3))),
            0xDB => Some(Instruction::SET(BitManTarget::E(3))),
            0xDC => Some(Instruction::SET(BitManTarget::H(3))),
            0xDD => Some(Instruction::SET(BitManTarget::H(3))),
            0xDE => Some(Instruction::SET(BitManTarget::HL(3))),
            0xDF => Some(Instruction::SET(BitManTarget::A(3))),
            0xE0 => Some(Instruction::SET(BitManTarget::B(4))),
            0xE1 => Some(Instruction::SET(BitManTarget::C(4))),
            0xE2 => Some(Instruction::SET(BitManTarget::D(4))),
            0xE3 => Some(Instruction::SET(BitManTarget::E(4))),
            0xE4 => Some(Instruction::SET(BitManTarget::H(4))),
            0xE5 => Some(Instruction::SET(BitManTarget::H(4))),
            0xE6 => Some(Instruction::SET(BitManTarget::HL(4))),
            0xE7 => Some(Instruction::SET(BitManTarget::A(4))),
            0xE8 => Some(Instruction::SET(BitManTarget::B(5))),
            0xE9 => Some(Instruction::SET(BitManTarget::C(5))),
            0xEA => Some(Instruction::SET(BitManTarget::D(5))),
            0xEB => Some(Instruction::SET(BitManTarget::E(5))),
            0xEC => Some(Instruction::SET(BitManTarget::H(5))),
            0xED => Some(Instruction::SET(BitManTarget::H(5))),
            0xEE => Some(Instruction::SET(BitManTarget::HL(5))),
            0xEF => Some(Instruction::SET(BitManTarget::A(5))),
            0xF0 => Some(Instruction::SET(BitManTarget::B(6))),
            0xF1 => Some(Instruction::SET(BitManTarget::C(6))),
            0xF2 => Some(Instruction::SET(BitManTarget::D(6))),
            0xF3 => Some(Instruction::SET(BitManTarget::E(6))),
            0xF4 => Some(Instruction::SET(BitManTarget::H(6))),
            0xF5 => Some(Instruction::SET(BitManTarget::H(6))),
            0xF6 => Some(Instruction::SET(BitManTarget::HL(6))),
            0xF7 => Some(Instruction::SET(BitManTarget::A(6))),
            0xF8 => Some(Instruction::SET(BitManTarget::B(7))),
            0xF9 => Some(Instruction::SET(BitManTarget::C(7))),
            0xFA => Some(Instruction::SET(BitManTarget::D(7))),
            0xFB => Some(Instruction::SET(BitManTarget::E(7))),
            0xFC => Some(Instruction::SET(BitManTarget::H(7))),
            0xFD => Some(Instruction::SET(BitManTarget::H(7))),
            0xFE => Some(Instruction::SET(BitManTarget::HL(7))),
            0xFF => Some(Instruction::SET(BitManTarget::A(7))),
            _ => panic!("opcode not found! :<")
        }       
    }
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL, N8,
}

enum BitManTarget {
    A(u8), B(u8), C(u8), D(u8), E(u8), H(u8), L(u8), HL(u8),
}

enum DeRefR8Target {
    A, B, C, D, E, H, L, HL,
}

impl CPU {
    fn step (&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for : {}", description);
        };

        self.pc = next_pc;
    }


    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                     ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        self.pc.wrapping_add(1)
                        /* TODO: support more targets */
                    }
                }
            }
            Instruction::RRCA => {
                self.rrca();
                self.pc.wrapping_add(1)
            }
            Instruction::RLCA => {
                self.rlca();
                self.pc.wrapping_add(1)
            }
            Instruction::CPL => {
                self.cpl();
                self.pc.wrapping_add(1)
            }
            Instruction::BIT(target) => {
                match target{
                    //fix
                    BitManTarget::A(bit) => {
                        self.bit(self.registers.a, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::B(bit) => {
                        self.bit(self.registers.b, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::C(bit) => {
                        self.bit(self.registers.c, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::D(bit) => {
                        self.bit(self.registers.d, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::E(bit) => {
                        self.bit(self.registers.e, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::H(bit) => {
                        self.bit(self.registers.h, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::L(bit) => {
                        self.bit(self.registers.l, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::HL(bit) => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
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
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::RES(target) => {
                match target {
                    //fix
                    BitManTarget::A(bit) => {
                        self.registers.a = self.res(self.registers.a, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::B(bit) => {
                        self.registers.b = self.res(self.registers.b, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::C(bit) => {
                        self.registers.c = self.res(self.registers.c, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::D(bit) => {
                        self.registers.d = self.res(self.registers.d, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::E(bit) => {
                        self.registers.e = self.res(self.registers.e, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::H(bit) => {
                        self.registers.h = self.res(self.registers.h, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::L(bit) => {
                        self.registers.l = self.res(self.registers.l, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::HL(bit) => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
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
                        //flags remain unaffected
                        self.bus.write_byte(HL, new_value);
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::SET(target) => {
                match target {
                    //fix
                    BitManTarget::A(bit) => {
                        self.registers.a = self.set(self.registers.a, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::B(bit) => {
                        self.registers.b = self.set(self.registers.b, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::C(bit) => {
                        self.registers.c = self.set(self.registers.c, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::D(bit) => {
                        self.registers.d = self.set(self.registers.d, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::E(bit) => {
                        self.registers.e = self.set(self.registers.e, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::H(bit) => {
                        self.registers.h = self.set(self.registers.h, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::L(bit) => {
                        self.registers.l = self.set(self.registers.l, bit);
                        self.pc.wrapping_add(2)
                    }
                    BitManTarget::HL(bit) => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
                        if bit > 7 {
                            panic!("bit placement is out of bounds (high). CPU FN SET")
                        }
                        else if bit < 0 {
                            panic!("bit placement is out of bounds (low). CPU FN SET")
                        }
                        let new_value = value | (0x01 << bit);
                        //flags remain unaffected
                        self.bus.write_byte(HL, new_value);
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::SRL(target) => {
                match target {
                    //fix
                    DeRefR8Target::A => {
                        self.registers.a = self.srl(self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::B => {
                        self.registers.b = self.srl(self.registers.b);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::C => {
                        self.registers.c = self.srl(self.registers.c);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::D => {
                        self.registers.d = self.srl(self.registers.d);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::E => {
                        self.registers.e = self.srl(self.registers.e);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::H => {
                        self.registers.h = self.srl(self.registers.h);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::L => {
                        self.registers.l = self.srl(self.registers.l);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::HL => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
                        let carry = value & 0x01;
                        let new_value = value >> 1;
                        //update flags
                        self.registers.f.zero = new_value == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = carry == 1;
                        self.bus.write_byte(HL, new_value);   
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::RR(target) => {
                match target {
                    //fix
                    DeRefR8Target::A => {
                        self.registers.a = self.rr(self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::B => {
                        self.registers.b = self.rr(self.registers.b);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::C => {
                        self.registers.c = self.rr(self.registers.c);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::D => {
                        self.registers.d = self.rr(self.registers.d);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::E => {
                        self.registers.e = self.rr(self.registers.e);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::H => {
                        self.registers.h = self.rr(self.registers.h);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::L => {
                        self.registers.l = self.rr(self.registers.l);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::HL => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
                        let carry = value & 0x01;
                        let carry_flag: u8 = (self.registers.f.carry as u8) << 7;
                        let new_value = (value >> 1) | carry_flag;
                        //update flags
                        self.registers.f.zero = new_value == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = carry == 1;
                        self.bus.write_byte(HL, new_value);                
                        self.pc.wrapping_add(2)
                    }    
                }
            }
            Instruction::RL(target) => {
                match target {
                    //fix
                    DeRefR8Target::A => {
                        self.registers.a = self.rl(self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::B => {
                        self.registers.b = self.rl(self.registers.b);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::C => {
                        self.registers.c = self.rl(self.registers.c);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::D => {
                        self.registers.d = self.rl(self.registers.d);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::E => {
                        self.registers.e = self.rl(self.registers.e);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::H => {
                        self.registers.h = self.rl(self.registers.h);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::L => {
                        self.registers.l = self.rl(self.registers.l);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::HL => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
                        let carry = value & 0x80;
                        let carry_flag: u8 = self.registers.f.carry as u8;
                        let new_value = (value << 1) | carry_flag;
                        //update flags
                        self.registers.f.zero = new_value == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = carry == 1;
                        self.bus.write_byte(HL, new_value);
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::RRC(target) => {
                match target {
                    //fix
                     DeRefR8Target::A => {
                        self.registers.a = self.rrc(self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::B =>  {
                        self.registers.b = self.rrc(self.registers.b);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::C => {
                        self.registers.c = self.rrc(self.registers.c);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::D => {
                        self.registers.d = self.rrc(self.registers.d);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::E => {
                        self.registers.e = self.rrc(self.registers.e);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::H => {
                        self.registers.h = self.rrc(self.registers.h);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::L => {
                        self.registers.l = self.rrc(self.registers.l);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::HL => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
                        let carry = value & 0x01;
                        let new_value = (value >> 1) | carry;
                        //update flags
                        self.registers.f.zero = new_value == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = carry == 1;
                        self.bus.write_byte(HL, new_value);                
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::RLC(target) => {
                match target {
                    DeRefR8Target::A => {
                        self.registers.a = self.rlc(self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::B => {
                        self.registers.b = self.rlc(self.registers.b);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::C => {
                        self.registers.c = self.rlc(self.registers.c);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::D => {
                        self.registers.d = self.rlc(self.registers.d);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::E => {
                        self.registers.e = self.rlc(self.registers.e);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::H => {
                        self.registers.h = self.rlc(self.registers.h);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::L => {
                        self.registers.l = self.rlc(self.registers.l);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::HL => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
                        let carry = value & 0x80;
                        let new_value = (value << 1) | carry;
                        //update flags
                        self.registers.f.zero = new_value == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = carry == 1;
                        self.bus.write_byte(HL, new_value);               
                        self.pc.wrapping_add(2)
                    }


                }
            }
            Instruction::SRA(target) => {
                match target {
                    DeRefR8Target::A => {
                        self.registers.a = self.sra(self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::B => {
                        self.registers.b = self.sra(self.registers.b);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::C => {
                        self.registers.c = self.sra(self.registers.c);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::D => {
                        self.registers.d = self.sra(self.registers.d);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::E => {
                        self.registers.e = self.sra(self.registers.e);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::H => {
                        self.registers.h = self.sra(self.registers.h);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::L => {
                        self.registers.l = self.sra(self.registers.l);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::HL =>  {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
                        let carry = value & 0x01;
                        let lmb = value & 0x80;
                        let new_value = lmb | (value >> 1);
                        //update flags
                        self.registers.f.zero = new_value == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = carry == 1;
                        self.bus.write_byte(HL, new_value);   
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::SLA(target) => {
                match target {
                    DeRefR8Target::A => {
                        self.registers.a = self.sla(self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::B => {
                        self.registers.b = self.sla(self.registers.b);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::C => {
                        self.registers.c = self.sla(self.registers.c);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::D => {
                        self.registers.d = self.sla(self.registers.d);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::E => {
                        self.registers.e = self.sla(self.registers.e);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::H => {
                        self.registers.h = self.sla(self.registers.h);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::L => {
                        self.registers.l = self.sla(self.registers.l);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::HL => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
                        let carry = value & 0x80;
                        let new_value = value << 1;
                        //update flags
                        self.registers.f.zero = new_value == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = carry == 1;
                        self.bus.write_byte(HL, new_value);
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::SWAP(target) => {
                match target {
                    DeRefR8Target::A => {
                        self.registers.a = self.swap(self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::B => {
                        self.registers.b = self.swap(self.registers.b); 
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::C => {
                        self.registers.c = self.swap(self.registers.c);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::D => {
                        self.registers.d = self.swap(self.registers.d);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::E => {
                        self.registers.e = self.swap(self.registers.e);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::H => { 
                        self.registers.h = self.swap(self.registers.h);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::L => {
                        self.registers.l = self.swap(self.registers.l);
                        self.pc.wrapping_add(2)
                    }
                    DeRefR8Target::HL => {
                        let HL: u16 = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        let value: u8 = self.bus.read_byte(HL);
                        let upper = (value & 0xF0) >> 4;
                        let new_value = (value << 4) | upper;
                        //update flags
                        self.registers.f.zero = new_value == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = false;
                        self.bus.write_byte(HL, new_value);
                        self.pc.wrapping_add(2)
                    }
            //don't mind this :> just uh... something that will go away once I'm sure it's not needed
           // _ => { /* TODO: support more instructions */}
                }
            }
            _ => {
                /* TODO: support more instructions */
                self.pc.wrapping_add(1)
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
        //flags remain unaffected
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
