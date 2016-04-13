extern crate trustines;
use std::u16;

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
    use trustines::cpu::OpcodeClass;
    use trustines::cpu::AddressMode;

    fn build_executor() -> cpu::CpuExecutor {
        let opcode_info = cpu::opcode::load_from_file("resources/opcodes.csv").unwrap();
        return cpu::CpuExecutor::new(opcode_info.0);
    }

    // NOTE: the opcodes specified in build_memory are not being executed, so the
    //       opcode class (ADC, ASL, BRK, etc) don't actually matter.  What matters
    //       is the addressing mode (Absolute, AbsoluteX, ZeroPage, etc)
    //

// ------------------ Non-Indexed, Non-Memory ------------------ //
    
    #[test]
    fn accumulator() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x0A); // 0x0A = ASL Accumulator

        cpu.pc = 0;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x0A);
        assert_eq!(OpcodeClass::ASL,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::Accumulator,cpu.decode_register.info.address_mode);
    }
    #[test]
    fn immediate() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x69); // 0x69 = ADC Immediate

        cpu.pc = 0;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x69);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::Immediate,cpu.decode_register.info.address_mode);
    }
    #[test]
    fn implied() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x00); // 0x00 = BRK Implied

        cpu.pc = 0;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x00);
        assert_eq!(OpcodeClass::BRK,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::Implied,cpu.decode_register.info.address_mode);
    }

// ------------------ Non-Indexed, Memory ------------------ //
    
    #[test]
    fn absolute() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();
        
        mem.write8(0,0x6D); //0x6D = ADC Absolute
        mem.write16(1,300);
        mem.write8(300,5);

        cpu.pc = 0;
        exec.fetch_and_decode(&mut cpu,&mut mem);
        let s = OpcodeClass::ADC;

        assert_eq!(0x6D,cpu.instruction_register);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::Absolute,cpu.decode_register.info.address_mode);

        assert_eq!(300,cpu.decode_register.addr_final.unwrap());
        assert_eq!(5,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn indirect() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x6C); // 0x6C = JMP Indirect
        mem.write16(1,300);
        mem.write16(300,500);

        cpu.pc = 0;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x6C);
        assert_eq!(OpcodeClass::JMP,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::Indirect,cpu.decode_register.info.address_mode);

        assert_eq!(300,cpu.decode_register.addr_intermediate.unwrap());
        assert_eq!(500,cpu.decode_register.addr_final.unwrap());
    }
    #[test]
    fn relative() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x90); // 0x90 = BCC IndirectIndexed
        mem.write8(1,100);

        cpu.pc = 0;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x90);
        assert_eq!(OpcodeClass::BCC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::Relative,cpu.decode_register.info.address_mode);

        assert_eq!(1,cpu.decode_register.addr_final.unwrap());
        assert_eq!(100,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn zeropage() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x65); // 0x65 = ADC IndirectIndexed
        mem.write8(1,100);
        mem.write8(100,5);

        cpu.pc = 0;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x65);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::ZeroPage,cpu.decode_register.info.address_mode);

        assert_eq!(100,cpu.decode_register.addr_final.unwrap());
        assert_eq!(5,cpu.decode_register.value_final.unwrap());
    }
    
// ------------------ Indexed, Memory ------------------ //

    #[test]
    fn absolute_x() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x7D); // 0x7D = ADC AbsoluteX
        mem.write16(1,300);
        mem.write8(305,50);

        cpu.pc = 0;
        cpu.x = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x7D);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::AbsoluteX,cpu.decode_register.info.address_mode);

        assert_eq!(305,cpu.decode_register.addr_final.unwrap());
        assert_eq!(50,cpu.decode_register.value_final.unwrap());
    }

    #[test]
    fn absolute_x_non_wraparound_by_1() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u16MAX = ::std::u16::MAX;

        // non wrap-around by 1
        mem.write8(0,0x7D); // 0x7D = ADC AbsoluteX
        mem.write16(1,u16MAX-6);
        mem.write8(u16MAX-1,50);

        cpu.pc = 0;
        cpu.x = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x7D);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::AbsoluteX,cpu.decode_register.info.address_mode);

        assert_eq!(u16MAX-1,cpu.decode_register.addr_final.unwrap());
        assert_eq!(50,cpu.decode_register.value_final.unwrap());
    }

    #[test]
    fn absolute_x_wraparound_to_0() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u16MAX = ::std::u16::MAX;

        mem.write8(0,0x7D); // 0x7D = ADC AbsoluteX
        mem.write16(1,u16MAX-5);

        cpu.pc = 0;
        cpu.x = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x7D);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::AbsoluteX,cpu.decode_register.info.address_mode);

        assert_eq!(0,cpu.decode_register.addr_final.unwrap());
        assert_eq!(0x7D,cpu.decode_register.value_final.unwrap());

    }
    #[test]
    fn absolute_x_wraparound_to_1() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u16MAX = ::std::u16::MAX;

        mem.write8(0,0x7D); // 0x7D = ADC AbsoluteX
        mem.write16(1,u16MAX-4);

        cpu.pc = 0;
        cpu.x = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x7D);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::AbsoluteX,cpu.decode_register.info.address_mode);

        assert_eq!(1,cpu.decode_register.addr_final.unwrap());
        assert_eq!(mem.read8(1).unwrap(),cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn absolute_y() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x79); // 0x79 = ADC AbsoluteY
        mem.write16(1,300);
        mem.write8(305,50);

        cpu.pc = 0;
        cpu.y = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x79);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::AbsoluteY,cpu.decode_register.info.address_mode);

        assert_eq!(305,cpu.decode_register.addr_final.unwrap());
        assert_eq!(50,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn absolute_y_non_wraparound_by_1() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u16MAX = ::std::u16::MAX;

        // non wrap-around by 1
        mem.write8(0,0x79); // 0x79 = ADC AbsoluteY
        mem.write16(1,u16MAX-6);
        mem.write8(u16MAX-1,50);

        cpu.pc = 0;
        cpu.y = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x79);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::AbsoluteY,cpu.decode_register.info.address_mode);

        assert_eq!(u16MAX-1,cpu.decode_register.addr_final.unwrap());
        assert_eq!(50,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn absolute_y_wraparound_to_0() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u16MAX = ::std::u16::MAX;

        mem.write8(0,0x79); // 0x79 = ADC AbsoluteY
        mem.write16(1,u16MAX-5);

        cpu.pc = 0;
        cpu.y = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x79);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::AbsoluteY,cpu.decode_register.info.address_mode);

        assert_eq!(0,cpu.decode_register.addr_final.unwrap());
        assert_eq!(0x79,cpu.decode_register.value_final.unwrap());

    }
    #[test]
    fn absolute_y_wraparound_to_1() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u16MAX = ::std::u16::MAX;

        mem.write8(0,0x79); // 0x7D = ADC AbsoluteX
        mem.write16(1,u16MAX-4);

        cpu.pc = 0;
        cpu.y = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x79);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::AbsoluteY,cpu.decode_register.info.address_mode);

        assert_eq!(1,cpu.decode_register.addr_final.unwrap());
        assert_eq!(mem.read8(1).unwrap(),cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn zeropage_x() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x75); // 0x75 = ADC ZeroPageX
        mem.write8(1,100);
        mem.write8(105,50);

        cpu.pc = 0;
        cpu.x = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x75);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::ZeroPageX,cpu.decode_register.info.address_mode);

        assert_eq!(105,cpu.decode_register.addr_final.unwrap());
        assert_eq!(50,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn zeropage_x_non_wraparound_by_1() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u8MAX = ::std::u8::MAX;

        mem.write8(0,0x75); // 0x75 = ADC ZeroPageX
        mem.write8(1,u8MAX-6);
        mem.write8((u8MAX-1) as u16,50);

        cpu.pc = 0;
        cpu.x = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x75);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::ZeroPageX,cpu.decode_register.info.address_mode);

        assert_eq!((u8MAX-1) as u16,cpu.decode_register.addr_final.unwrap());
        assert_eq!(50,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn zeropage_x_wraparound_to_0() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u8MAX = ::std::u8::MAX;

        mem.write8(0,0x75); // 0x75 = ADC ZeroPageX
        mem.write8(1,u8MAX-5);

        cpu.pc = 0;
        cpu.x = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x75);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::ZeroPageX,cpu.decode_register.info.address_mode);

        assert_eq!(0,cpu.decode_register.addr_final.unwrap());
        assert_eq!(0x75,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn zeropage_x_wraparound_to_1() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u8MAX = ::std::u8::MAX;

        mem.write8(0,0x75); // 0x75 = ADC ZeroPageX
        mem.write8(1,u8MAX-4);

        cpu.pc = 0;
        cpu.x = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x75);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::ZeroPageX,cpu.decode_register.info.address_mode);

        assert_eq!(1,cpu.decode_register.addr_final.unwrap());
        assert_eq!(mem.read8(1).unwrap(),cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn zeropage_y() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0xB6); // 0xB6 = LDX IndirectIndexed
        mem.write8(1,100);
        mem.write8(105,50);

        cpu.pc = 0;
        cpu.y = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0xB6);
        assert_eq!(OpcodeClass::LDX,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::ZeroPageY,cpu.decode_register.info.address_mode);

        assert_eq!(105,cpu.decode_register.addr_final.unwrap());
        assert_eq!(50,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn zeropage_y_non_wraparound_by_1() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u8MAX = ::std::u8::MAX;

        mem.write8(0,0xB6); // 0xB6 = LDX IndirectIndexed
        mem.write8(1,u8MAX-6);
        mem.write8((u8MAX-1) as u16,50);

        cpu.pc = 0;
        cpu.y = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0xB6);
        assert_eq!(OpcodeClass::LDX,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::ZeroPageY,cpu.decode_register.info.address_mode);

        assert_eq!((u8MAX-1) as u16,cpu.decode_register.addr_final.unwrap());
        assert_eq!(50,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn zeropage_y_wraparound_to_0() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u8MAX = ::std::u8::MAX;

        mem.write8(0,0xB6); // 0xB6 = LDX IndirectIndexed
        mem.write8(1,u8MAX-5);

        cpu.pc = 0;
        cpu.y = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0xB6);
        assert_eq!(OpcodeClass::LDX,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::ZeroPageY,cpu.decode_register.info.address_mode);

        assert_eq!(0,cpu.decode_register.addr_final.unwrap());
        assert_eq!(0xB6,cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn zeropage_y_wraparound_to_1() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        let u8MAX = ::std::u8::MAX;

        mem.write8(0,0xB6); // 0xB6 = LDX IndirectIndexed
        mem.write8(1,u8MAX-4);

        cpu.pc = 0;
        cpu.y = 5;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0xB6);
        assert_eq!(OpcodeClass::LDX,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::ZeroPageY,cpu.decode_register.info.address_mode);

        assert_eq!(1,cpu.decode_register.addr_final.unwrap());
        assert_eq!(mem.read8(1).unwrap(),cpu.decode_register.value_final.unwrap());
    }
    #[test]
    fn indexed_indirect() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x61); // 0x61 = ADC IndexedIndirect

        cpu.pc = 0;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x61);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::IndexedIndirect,cpu.decode_register.info.address_mode);
    }
    #[test]
    fn indirect_indexed() {
        let mut cpu: cpu::CpuState = Default::default();
        let mut mem = Memory::new();
        let exec = build_executor();

        mem.write8(0,0x71); // 0x71 = ADC IndirectIndexed

        cpu.pc = 0;
        exec.fetch_and_decode(&mut cpu,&mut mem);

        assert_eq!(cpu.instruction_register,0x71);
        assert_eq!(OpcodeClass::ADC,cpu.decode_register.info.opcode_class);
        assert_eq!(AddressMode::IndirectIndexed,cpu.decode_register.info.address_mode);
    }
}

