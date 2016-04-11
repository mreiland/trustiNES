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

    #[test]
    fn absolute() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn absolute_x() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn absolute_y() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn accumulator() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn immediate() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn implied() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn indirect() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn indexed_indirect() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn indirect_indexed() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn relative() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn zeropage() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn zeropage_x() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
    #[test]
    fn zeropage_y() {
        let exec = build_executor();
        let mut m = Memory::new();
    }
}

