//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*
//------------------C P U-------------------
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
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

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*
//---------I N S T R U C T I O N S----------
//_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*_*

enum Instruction {
    ADD(AddType),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    XOR(ArithmeticTarget),
    OR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(IncDecTarget),
    DEC(IncDecTarget),
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
    CCF,
    SCF,
    RRA,
    RLA,
    JP(JumpTest),
    JPHL,
    JR(JumpTest),
    LD(LoadType),
    PUSH(PushPopTarget),
    POP(PushPopTarget),
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
            0x01 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16))),
            0x02 => Some(Instruction::LD(LoadType::IndirectFromA(LoadIFATarget::BC))),
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x07 => Some(Instruction::RLCA),
            0x08 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::A16, LoadWordSource::SP))),
            0x09 => Some(Instruction::ADD(AddType::Word(ArithmeticWordTarget::BC))),
            0x0A => Some(Instruction::LD(LoadType::AFromIndirect(LoadAFISource::BC))),
            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x0F => Some(Instruction::RRCA),
            0x10 => todo!(),
            0x11 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16))),
            0x12 => Some(Instruction::LD(LoadType::IndirectFromA(LoadIFATarget::DE))),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x17 => Some(Instruction::RLA),
            0x18 => Some(Instruction::JR(JumpTest::Always)),
            0x19 => Some(Instruction::ADD(AddType::Word(ArithmeticWordTarget::DE))),
            0x1A => Some(Instruction::LD(LoadType::AFromIndirect(LoadAFISource::DE))),
            0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x1F => Some(Instruction::RRA),
            0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16))),
            0x22 => Some(Instruction::LD(LoadType::IndirectFromA(LoadIFATarget::HLI))),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x27 => todo!(),
            0x28 => Some(Instruction::JR(JumpTest::Zero)), 
            0x29 => Some(Instruction::ADD(AddType::Word(ArithmeticWordTarget::HL))),
            0x2A => Some(Instruction::LD(LoadType::AFromIndirect(LoadAFISource::HLI))),
            0x2B => Some(Instruction::DEC(IncDecTarget::HL)),
            0x2C => Some(Instruction::INC(IncDecTarget::L)),
            0x2D => Some(Instruction::DEC(IncDecTarget::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x2F => Some(Instruction::CPL),
            0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16))),
            0x32 => Some(Instruction::LD(LoadType::IndirectFromA(LoadIFATarget::HLD))),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),
            0x34 => Some(Instruction::INC(IncDecTarget::AHL)),
            0x35 => Some(Instruction::DEC(IncDecTarget::AHL)),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D8))),
            0x37 => Some(Instruction::SCF),
            0x38 => Some(Instruction::JR(JumpTest::Carry)),
            0x39 => Some(Instruction::ADD(AddType::Word(ArithmeticWordTarget::SP))),
            0x3A => Some(Instruction::LD(LoadType::AFromIndirect(LoadAFISource::HLD))),
            0x3B => Some(Instruction::DEC(IncDecTarget::SP)),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),
            0x3D => Some(Instruction::DEC(IncDecTarget::A)),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),
            0x3F => Some(Instruction::CCF),
            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HL))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),
            0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HL))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),
            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HL))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),
            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HL))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),
            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HL))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),
            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HL))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),
            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::L))),
            0x76 => todo!(),
            0x77 => Some(Instruction::LD(LoadType::IndirectFromA(LoadIFATarget::HL))),
            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::AFromIndirect(LoadAFISource::HL))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),
            0x80 => Some(Instruction::ADD(AddType::Byte(ArithmeticTarget::B))),
            0x81 => Some(Instruction::ADD(AddType::Byte(ArithmeticTarget::C))),
            0x82 => Some(Instruction::ADD(AddType::Byte(ArithmeticTarget::D))),
            0x83 => Some(Instruction::ADD(AddType::Byte(ArithmeticTarget::E))),
            0x84 => Some(Instruction::ADD(AddType::Byte(ArithmeticTarget::H))),
            0x85 => Some(Instruction::ADD(AddType::Byte(ArithmeticTarget::L))),
            0x86 => Some(Instruction::ADD(AddType::Byte(ArithmeticTarget::HL))),
            0x87 => Some(Instruction::ADD(AddType::Byte(ArithmeticTarget::A))),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)), 
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HL)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HL)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HL)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),
            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HL)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HL)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::HL)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::HL)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),
            0xC0 => todo!(),
            0xC1 => Some(Instruction::POP(PushPopTarget::BC)),
            0xC2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xC3 => Some(Instruction::JP(JumpTest::Always)),
            0xC4 => todo!(),
            0xC5 => Some(Instruction::PUSH(PushPopTarget::BC)),
            0xC6 => Some(Instruction::ADD(AddType::Byte(ArithmeticTarget::N8))),
            0xC7 => todo!(),
            0xC8 => todo!(),
            0xC9 => todo!(),
            0xCA => Some(Instruction::JP(JumpTest::Zero)),
            0xCB => todo!(),
            0xCC => todo!(),
            0xCD => todo!(),
            0xCE => Some(Instruction::ADC(ArithmeticTarget::N8)),
            0xCF => todo!(),
            0xD0 => todo!(),
            0xD1 => Some(Instruction::POP(PushPopTarget::DE)),
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry)),
            0xD3 => todo!(),
            0xD4 => todo!(),
            0xD5 => Some(Instruction::PUSH(PushPopTarget::DE)),
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::N8)),
            0xD7 => todo!(),
            0xD8 => todo!(),
            0xD9 => todo!(), 
            0xDA => Some(Instruction::JP(JumpTest::Carry)),
            0xDB => todo!(),
            0xDC => todo!(),
            0xDD => todo!(),
            0xDE => Some(Instruction::SBC(ArithmeticTarget::N8)),
            0xDF => todo!(),
            0xE0 => Some(Instruction::LD(LoadType::ByteAddressFromA(LoadBAFATarget::A8))),
            0xE1 => Some(Instruction::POP(PushPopTarget::HL)),
            0xE2 => Some(Instruction::LD(LoadType::ByteAddressFromA(LoadBAFATarget::C))),
            0xE3 => todo!(),
            0xE4 => todo!(),
            0xE5 => Some(Instruction::PUSH(PushPopTarget::HL)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::N8)),
            0xE7 => todo!(),
            0xE8 => Some(Instruction::ADD(AddType::Word(ArithmeticWordTarget::E8))),
            0xE9 => Some(Instruction::JPHL),
            0xEA => Some(Instruction::LD(LoadType::IndirectFromA(LoadIFATarget::A16))),
            0xEB => todo!(),
            0xEC => todo!(),
            0xED => todo!(),
            0xEE => Some(Instruction::XOR(ArithmeticTarget::N8)),
            0xEF => todo!(),
            0xF0 => Some(Instruction::LD(LoadType::AFromByteAddress(LoadAFBASource::A8))),
            0xF1 => Some(Instruction::POP(PushPopTarget::AF)),
            0xF2 => Some(Instruction::LD(LoadType::AFromByteAddress(LoadAFBASource::C))),
            0xF3 => todo!(),
            0xF4 => todo!(),
            0xF5 => Some(Instruction::PUSH(PushPopTarget::AF)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::N8)),
            0xF7 => todo!(),
            0xF8 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::SPR8))),
            0xF9 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::HL))),
            0xFA => Some(Instruction::LD(LoadType::AFromIndirect(LoadAFISource::A16))),
            0xFB => todo!(),
            0xFC => todo!(),
            0xFD => todo!(),
            0xFE => Some(Instruction::CP(ArithmeticTarget::N8)),
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

enum IncDecTarget {
    A, B, C, D, E, H, L, AHL, BC, DE, HL, SP
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL, N8
}

enum ArithmeticWordTarget {
   BC, DE, HL, SP, E8
}

enum BitManTarget {
    A(u8), B(u8), C(u8), D(u8), E(u8), H(u8), L(u8), HL(u8),
}

enum DeRefR8Target {
    A, B, C, D, E, H, L, HL,
}

enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

enum LoadByteTarget {
    A, B, C, D, E, H, L, HL
}

enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HL
}

enum LoadWordTarget {
    BC, DE, HL, SP, A16
}

enum LoadWordSource {
    D16, SPR8, HL, SP
}

enum LoadAFISource {
    BC, DE, HL, HLI, HLD, A16
}
 
enum LoadIFATarget {
    BC, DE, HL, HLI, HLD, A16
}

enum LoadAFBASource {
    C, A8
}

enum LoadBAFATarget {
    C, A8
}

enum PushPopTarget {
    BC, DE, HL, AF

}

enum AddType {
    Byte(ArithmeticTarget),
    Word(ArithmeticWordTarget),
}

enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
    AFromIndirect(LoadAFISource),
    IndirectFromA(LoadIFATarget),
    AFromByteAddress(LoadAFBASource),
    ByteAddressFromA(LoadBAFATarget),

}

impl CPU {
     fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc.wrapping_add(1))
    }

    fn step (&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
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
            Instruction::CCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
                self.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
                self.pc.wrapping_add(1)
            }
            Instruction::ADD(add_type) => {
                match add_type {
                    AddType::Byte(target) => {
                        match target {
                            ArithmeticTarget::A => {
                                let value = self.registers.a;
                                let new_value = self.add(value);
                                self.registers.a = new_value;
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticTarget::B => {
                                let value = self.registers.b;
                                let new_value = self.add(value);
                                self.registers.a = new_value;
                                self.pc.wrapping_add(1)

                            }
                            ArithmeticTarget::C => {
                                let value = self.registers.c;
                                let new_value = self.add(value);
                                self.registers.a = new_value;
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticTarget::D => {
                                let value = self.registers.d;
                                let new_value = self.add(value);
                                self.registers.a = new_value;
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticTarget::E => {
                                let value = self.registers.e;
                                let new_value = self.add(value);
                                self.registers.a = new_value;
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticTarget::H => {
                                let value = self.registers.h;
                                let new_value = self.add(value);
                                self.registers.a = new_value;
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticTarget::L => {
                                let value = self.registers.l;
                                let new_value = self.add(value);
                                self.registers.a = new_value;
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticTarget::HL => {
                                let value = self.bus.read_byte(self.registers.get_hl());
                                let new_value = self.add(value);
                                self.registers.a = new_value;
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticTarget::N8 => {
                                let value = self.bus.read_byte(self.pc + 1);
                                let new_value = self.add(value);
                                self.registers.a = new_value;
                                self.pc.wrapping_add(2)
                            }
                        }
                    }
                    AddType::Word(target) => {
                        match target {
                            ArithmeticWordTarget::BC => {
                                self.registers.f.subtract = false;
                                self.registers.f.half_carry = ((self.registers.get_bc() & 0xFFF) + (self.registers.get_hl() & 0xFFF)) > 0xFFF;
                                self.registers.f.carry = ((self.registers.get_bc() as u32) + (self.registers.get_hl() as u32)) > 0xFFFF;
                                self.registers.set_hl(self.registers.get_bc().wrapping_add(self.registers.get_hl()));
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticWordTarget::DE => {
                                self.registers.f.subtract = false;
                                self.registers.f.half_carry = ((self.registers.get_de() & 0xFFF) + (self.registers.get_hl() & 0xFFF)) > 0xFFF;
                                self.registers.f.carry = ((self.registers.get_de() as u32) + (self.registers.get_hl() as u32)) > 0xFFFF;
                                self.registers.set_hl(self.registers.get_de().wrapping_add(self.registers.get_hl()));
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticWordTarget::HL => {
                                self.registers.f.subtract = false;
                                self.registers.f.half_carry = ((self.registers.get_hl() & 0xFFF) + (self.registers.get_hl() & 0xFFF)) > 0xFFF;
                                self.registers.f.carry = ((self.registers.get_hl() as u32) + (self.registers.get_hl() as u32)) > 0xFFFF;
                                self.registers.set_hl(self.registers.get_hl().wrapping_add(self.registers.get_hl()));
                                self.pc.wrapping_add(1)

                            }
                            ArithmeticWordTarget::SP => {
                                self.registers.f.subtract = false;
                                self.registers.f.half_carry = ((self.sp & 0xFFF) + (self.registers.get_hl() & 0xFFF)) > 0xFFF;
                                self.registers.f.carry = ((self.sp as u32) + (self.registers.get_hl() as u32)) > 0xFFFF;
                                self.registers.set_hl(self.sp.wrapping_add(self.registers.get_hl()));
                                self.pc.wrapping_add(1)
                            }
                            ArithmeticWordTarget::E8 => {
                                let value = self.bus.read_byte(self.pc + 1) as i16;
                                if (value >= 0) {
                                    self.sp = self.sp.wrapping_add(value as u16);
                                } else {
                                    self.sp = self.sp.wrapping_sub(value as u16);
                                }
                                //flags
                                self.registers.f.zero = false;
                                self.registers.f.subtract = false;
                                self.registers.f.half_carry = ((value as u16) + (self.sp & 0xF)) > 0xF;
                                self.registers.f.carry = ((value as u16) + (self.sp & 0xFF)) > 0xFF;
                                self.pc.wrapping_add(2)
                            }
                        }
                    }
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.bus.read_byte(self.pc + 1);
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.bus.read_byte(self.pc + 1);
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(2)

                    }
                }
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.bus.read_byte(self.pc + 1);
                        let new_value = self.sbc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(2)

                    }
                }
            }
            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)

                    }
                    ArithmeticTarget::N8 => {
                        let value = self.bus.read_byte(self.pc + 1);
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.bus.read_byte(self.pc + 1);
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::OR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.bus.read_byte(self.pc + 1);
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::CP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        self.cp(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.bus.read_byte(self.pc + 1);
                        self.cp(value);
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::INC(target) => {
                match target {
                    IncDecTarget::A => {
                       self.registers.a = self.inc(self.registers.a);
                       self.pc.wrapping_add(1)
                    }
                    IncDecTarget::B => {
                        self.registers.b = self.inc(self.registers.b);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::C => {
                        self.registers.c = self.inc(self.registers.c);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::D => {
                        self.registers.d = self.inc(self.registers.d);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::E => {
                        self.registers.e = self.inc(self.registers.e);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::H => {
                        self.registers.h = self.inc(self.registers.h);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::L => {
                        self.registers.l = self.inc(self.registers.l);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::AHL => {
                        let value = self.inc(self.bus.read_byte(self.registers.get_hl()));
                        self.bus.write_byte(self.registers.get_hl(), value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::BC => {
                        let value = (self.registers.get_bc()).wrapping_add(1);
                        self.registers.set_bc(value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::DE => {
                        let value = (self.registers.get_de()).wrapping_add(1);
                        self.registers.set_de(value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::HL => {
                        let value = (self.registers.get_hl()).wrapping_add(1);
                        self.registers.set_hl(value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::SP => {
                        self.sp = self.sp.wrapping_add(1);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::DEC(target) => {
                match target {
                    IncDecTarget::A => {
                        self.registers.a = self.dec(self.registers.a);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::B => {
                        self.registers.b = self.dec(self.registers.b);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::C => {
                        self.registers.c = self.dec(self.registers.c);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::D => {
                        self.registers.d = self.dec(self.registers.d);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::E => {
                        self.registers.e = self.dec(self.registers.e);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::H => {
                        self.registers.h = self.dec(self.registers.h);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::L => {
                        self.registers.l = self.dec(self.registers.l);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::AHL => {
                        let value = self.dec(self.bus.read_byte(self.registers.get_hl()));
                        self.bus.write_byte(self.registers.get_hl(), value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::BC => {
                        let value = (self.registers.get_bc()).wrapping_sub(1);
                        self.registers.set_bc(value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::DE => {
                        let value = (self.registers.get_de()).wrapping_sub(1);
                        self.registers.set_de(value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::HL => {
                        let value = (self.registers.get_hl()).wrapping_sub(1);
                        self.registers.set_hl(value);
                        self.pc.wrapping_add(1)
                    }
                    IncDecTarget::SP => {
                        self.sp = self.sp.wrapping_sub(1);
                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    PushPopTarget::BC => self.registers.set_bc(result),
                    PushPopTarget::DE => self.registers.set_de(result),
                    PushPopTarget::HL => self.registers.set_hl(result),
                    PushPopTarget::AF => self.registers.set_af(result)
                };
                self.pc.wrapping_add(1)
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    PushPopTarget::BC => self.registers.get_bc(),
                    PushPopTarget::DE => self.registers.get_de(),
                    PushPopTarget::HL => self.registers.get_hl(),
                    PushPopTarget::AF => self.registers.get_af()
                };

                self.push(value);
                self.pc.wrapping_add(1)

            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::HL => self.bus.read_byte(self.registers.get_hl()),
                            LoadByteSource::D8 => self.read_next_byte()
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value, 
                            LoadByteTarget::HL => self.bus.write_byte(self.registers.get_hl(), source_value)
                        };
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _                  => self.pc.wrapping_add(1),
                        }
                    }
                    LoadType::Word(target, source) => {
                        let source_value = match source {
                            LoadWordSource::D16 => {
                                //3
                                let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
                                let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
                                (most_significant_byte << 8) | least_significant_byte
                            }
                            LoadWordSource::SPR8 => {
                                //2
                                //you could use read_next_byte here
                                let offset = self.bus.read_byte(self.pc + 1) as u16;
                                let (new_value, did_overflow) = self.sp.overflowing_add(offset);
                                self.registers.f.half_carry = (self.sp & 0xF) + (offset & 0xF) > 0xF;
                                self.registers.f.carry = did_overflow;
                                new_value
                                
                            }
                            LoadWordSource::HL => {
                                //1
                                self.registers.get_hl()
                            }
                            LoadWordSource::SP => {
                                //3
                                self.sp
                            }
                        };
                        match target {
                            LoadWordTarget::BC => self.registers.set_bc(source_value),
                            LoadWordTarget::DE => self.registers.set_de(source_value),
                            LoadWordTarget::HL => self.registers.set_hl(source_value),
                            LoadWordTarget::SP => {
                                //might have to fix this later derp :b
                                self.sp = source_value;
                            }
                            LoadWordTarget::A16 => {
                            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
                            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
                            let address = (most_significant_byte << 8) | least_significant_byte;
                            let first_byte = (source_value & 0xFF) as u8;
                            let second_byte = (source_value >> 8) as u8;
                            self.bus.write_byte(address, first_byte);
                            self.bus.write_byte(address.wrapping_add(1), second_byte);
                            }
                        };
                        match source {
                            LoadWordSource::D16 => self.pc.wrapping_add(3),
                            LoadWordSource::SPR8 => self.pc.wrapping_add(2),
                            LoadWordSource::HL => self.pc.wrapping_add(1),
                            LoadWordSource::SP => self.pc.wrapping_add(3),
                        }
                    }
                    LoadType::AFromIndirect(source) => {
                        match source {
                            LoadAFISource::BC => {
                                self.registers.a = self.bus.read_byte(self.registers.get_bc());
                                self.pc.wrapping_add(1)
                            }
                            LoadAFISource::DE => {
                                self.registers.a = self.bus.read_byte(self.registers.get_de());
                                self.pc.wrapping_add(1)
                            }
                            LoadAFISource::HL => {
                                self.registers.a = self.bus.read_byte(self.registers.get_hl());
                                self.pc.wrapping_add(1)
                            }
                            LoadAFISource::HLI => {
                                self.registers.a = self.bus.read_byte(self.registers.get_hl());
                                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                                self.pc.wrapping_add(1)
                            }
                            LoadAFISource::HLD => {
                                self.registers.a = self.bus.read_byte(self.registers.get_hl());
                                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                                self.pc.wrapping_add(1)
                            }
                            LoadAFISource::A16 => {
                                let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
                                let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
                                let address = (most_significant_byte << 8) | least_significant_byte;
                                self.registers.a = self.bus.read_byte(address);
                                self.pc.wrapping_add(3)

                            }
                        }
                    }
                    LoadType::IndirectFromA(target) => {
                        match target {
                            LoadIFATarget::BC => {
                                self.bus.write_byte(self.registers.get_bc(), self.registers.a);
                                self.pc.wrapping_add(1)
                            }
                            LoadIFATarget::DE => {
                                self.bus.write_byte(self.registers.get_de(), self.registers.a);
                                self.pc.wrapping_add(1)
                            }
                            LoadIFATarget::HL => {
                                self.bus.write_byte(self.registers.get_hl(), self.registers.a);
                                self.pc.wrapping_add(1)
                            }
                            LoadIFATarget::HLI => {
                                self.bus.write_byte(self.registers.get_hl(), self.registers.a);
                                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                                self.pc.wrapping_add(1)
                            }
                            LoadIFATarget::HLD => {
                                self.bus.write_byte(self.registers.get_hl(), self.registers.a);
                                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                                self.pc.wrapping_add(1)
                            }
                            LoadIFATarget::A16 => {
                                let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
                                let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
                                let address = (most_significant_byte << 8) | least_significant_byte;
                                self.bus.write_byte(address, self.registers.a);
                                self.pc.wrapping_add(3)
                            }
                        }
                    }
                    LoadType::AFromByteAddress(source) => {
                        match source {
                            LoadAFBASource::C => {
                                let source_byte = self.bus.read_byte(0xFF00 | (self.registers.c as u16));
                                self.registers.a = source_byte;
                                self.pc.wrapping_add(1)
                            }
                            LoadAFBASource::A8 => {
                                let source_byte = self.bus.read_byte(0xFF00 | (self.bus.read_byte(self.pc + 1) as u16));
                                self.registers.a = source_byte;
                                self.pc.wrapping_add(2)

                            }
                        }
                    }
                    LoadType::ByteAddressFromA(target) => {
                        match target {
                            LoadBAFATarget::C => {
                                let address = (0xFF00 | (self.registers.c as u16));
                                self.bus.write_byte(address, self.registers.a);
                                self.pc.wrapping_add(1)
                            }
                            LoadBAFATarget::A8 => {
                                let address = (0xFF00 | (self.bus.read_byte(self.pc + 1) as u16));
                                self.bus.write_byte(address, self.registers.a);
                                self.pc.wrapping_add(2)
                            }
                        }
                    }
                }

            }
            Instruction::JR(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump_relative(jump_condition)
            }
            Instruction::JPHL => {
                ((self.registers.h as u16) << 8) | (self.registers.l as u16)
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
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
            Instruction::RRA => {
                self.registers.a = self.rra();
                self.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                self.registers.a = self.rla();
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
                            panic!("bit placement is out of bounds (low). CPU FN RES")
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
    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb

    }
    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn jump_relative(&self, should_jump: bool) -> u16 {
        if should_jump {
            let offset = self.bus.read_byte(self.pc + 1) as i16;
            if (offset >= 0) {
                self.pc.wrapping_add(offset as u16)
            } else {
                self.pc.wrapping_sub(offset as u16)
            }
        } else {
            self.pc.wrapping_add(2)
        }
    }
    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }
    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) == 0xF;
        new_value
    }
    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) == 0x0;
        new_value
    }
    fn cp(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = did_overflow;
    }
    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        //update falgs
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }
    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        new_value
    }

    fn sbc(&mut self, value: u8) -> u8 {
        let (step_value, add_overflow) = (self.registers.f.carry as u8).overflowing_add(value);
        let (new_value, sub_underflow) = self.registers.a.overflowing_sub(step_value);
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (self.registers.f.carry as u8)) > (value & 0xF);
        self.registers.f.carry = add_overflow | sub_underflow;
        new_value
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = did_overflow;
        new_value
    }

    fn adc(&mut self, value: u8) -> u8 {
        let (step_value, did_overflow) = self.registers.a.overflowing_add(value);
        let (new_value, did_overflow2) = step_value.overflowing_add(self.registers.f.carry as u8);
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract= false;
        self.registers.f.half_carry = (((self.registers.a & 0xF) + (value & 0xF)) > 0xF) | (((self.registers.f.carry as u8) + (step_value & 0xF)) > 0xF);
        self.registers.f.carry = did_overflow | did_overflow2;
        new_value

    }
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        //update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) > 0xF;
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

    fn rla(&mut self) -> u8 {
        let carry = self.registers.a & 0x80;
        let carry_flag: u8 = self.registers.f.carry as u8;
        let new_value = (self.registers.a << 1) | carry_flag;
        //update flags
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 0x80;
        new_value
    }
    
    fn rra(&mut self) -> u8 {
        let carry = self.registers.a & 0x01;
        let carry_flag: u8 = (self.registers.f.carry as u8) << 7;
        let new_value = (self.registers.a >> 1) | carry_flag;
        //update flags
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 0x01;
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
