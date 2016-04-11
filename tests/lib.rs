extern crate trustines;

mod memory {
    use trustines::memory::Memory;

    #[test]
    fn read8_basic() {
        let mut m = Memory::new();
        m.mem[0] = 5;
        m.mem[1] = 10;
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
        m.mem[0] = 2;
        m.mem[1] = 1;
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
    #[test]
    fn absolute() {
    }
    #[test]
    fn absolute_x() {
    }
    #[test]
    fn absolute_y() {
    }
    #[test]
    fn accumulator() {
    }
    #[test]
    fn immediate() {
    }
    #[test]
    fn implied() {
    }
    #[test]
    fn indirect() {
    }
    #[test]
    fn indexed_indirect() {
    }
    #[test]
    fn indirect_indexed() {
    }
    #[test]
    fn relative() {
    }
    #[test]
    fn zeropage() {
    }
    #[test]
    fn zeropage_x() {
    }
    #[test]
    fn zeropage_y() {
    }
}

