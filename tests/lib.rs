extern crate trustines;

mod memory {
    use trustines::memory::Memory;
    #[test]
    fn write() {
        let mut m = Memory::new();
        m.write(0, &[0,1,2]);
        assert_eq!(0,m.read8(0).unwrap());
        assert_eq!(1,m.read8(1).unwrap());
        assert_eq!(2,m.read8(2).unwrap());
        m.write(1, &[3]);
        assert_eq!(0,m.read8(0).unwrap());
        assert_eq!(3,m.read8(1).unwrap());
        assert_eq!(2,m.read8(2).unwrap());
    }

    #[test]
    fn read8_basic() {
        let mut m = Memory::new();
        m.write(0,&[5,10]);
        assert_eq!(5,m.read8(0).unwrap());
        assert_eq!(10,m.read8(1).unwrap());
    }
    #[test]
    fn write8_basic() {
        let mut m = Memory::new();
        m.write8(0,5);
        assert_eq!(5,m.mem[0]);

        m.write8(1,10);
        assert_eq!(5,m.mem[0]);
        assert_eq!(10,m.mem[1]);
    }

    #[test]
    fn read16_basic() {
        let mut m = Memory::new();
        m.write(0,&[2,1,0]);
        assert_eq!(258,m.read16(0).unwrap());
        assert_eq!(1,m.read16(1).unwrap());
    }
    #[test]
    fn write16_basic() {
        let mut m = Memory::new();
        m.write16(0,258);
        assert_eq!(2,m.mem[0]);
        assert_eq!(1,m.mem[1]);
    }
    #[test]
    fn readwrite8() {
        let mut m = Memory::new();
        m.write8(0,5);
        m.write8(1,10);
        assert_eq!(5,m.read8(0).unwrap());
        assert_eq!(10,m.read8(1).unwrap());
    }
    #[test]
    fn readwrite16() {
        let mut m = Memory::new();
        m.write16(0,258);
        m.write16(2,300);
        assert_eq!(258,m.read16(0).unwrap());
        assert_eq!(300,m.read16(2).unwrap());
    }
}

mod address_mode {
    use trustines::cpu;
    use trustines::memory::Memory;

    fn build_executor() -> cpu::CpuExecutor {
        let opcode_info = cpu::opcode::load_from_file("resources/opcodes.csv").unwrap();
        return cpu::CpuExecutor::new(opcode_info.0);
    }
    fn build_memory(instr:u8,ops: &[u8]) -> Memory {
        return Memory::new();
    }

    // NOTE: the opcodes specified in build_memory are not being exucuted, so the
    //       opcode class (ADC, ASL, BRK, etc) don't actually matter.  What matters
    //       is the addressing mode (Absolute, AbsoluteX, ZeroPage, etc)
    //

    #[test]
    fn absolute() {
        let exec = build_executor();
        let mut m = build_memory(0x6D,&[]); //0x6D = ADC Absolute
    }
    #[test]
    fn absolute_x() {
        let exec = build_executor();
        let mut m = build_memory(0x7D,&[]); // 0x7D = ADC AbsoluteX
    }
    #[test]
    fn absolute_y() {
        let exec = build_executor();
        let mut m = build_memory(0x79,&[]);// 0x79 = ADC AbsoluteY
    }
    #[test]
    fn accumulator() {
        let exec = build_executor();
        let mut m = build_memory(0x0A,&[]);// 0x0A = ASL Accumulator
    }
    #[test]
    fn immediate() {
        let exec = build_executor();
        let mut m = build_memory(0x69,&[]);// 0x69 = ADC Immediate
    }
    #[test]
    fn implied() {
        let exec = build_executor();
        let mut m = build_memory(0x00,&[]);// 0x00 = BRK Implied
    }
    #[test]
    fn indirect() {
        let exec = build_executor();
        let mut m = build_memory(0x6C,&[]);// 0x6C = JMP Indirect
    }
    #[test]
    fn indexed_indirect() {
        let exec = build_executor();
        let mut m = build_memory(0x61,&[]);// 0x61 = ADC IndexedIndirect
    }
    #[test]
    fn indirect_indexed() {
        let exec = build_executor();
        let mut m = build_memory(0x71,&[]);// 0x71 = ADC IndirectIndexed
    }
    #[test]
    fn relative() {
        let exec = build_executor();
        let mut m = build_memory(0x90,&[]);// 0x90 = BCC IndirectIndexed
    }
    #[test]
    fn zeropage() {
        let exec = build_executor();
        let mut m = build_memory(0x65,&[]);// 0x65 = ADC IndirectIndexed
    }
    #[test]
    fn zeropage_x() {
        let exec = build_executor();
        let mut m = build_memory(0x75,&[]);// 0x75 = ADC IndirectIndexed
    }
    #[test]
    fn zeropage_y() {
        let exec = build_executor();
        let mut m = build_memory(0xB6,&[]);// 0xB6 = LDX IndirectIndexed
    }
}

