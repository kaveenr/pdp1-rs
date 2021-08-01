#![crate_name = "pdp1_rs"]
/// Core Emulation Module
mod pdp1 {

    const DATA_MASK: u32 = 0b111111111111111111;

    /// Main Emulated State Of The PDP-1
    pub struct State {
        /// CM - Core Memory (4096 of 18 Bit Words)
        pub cm: [u32; 4096],
        /// MB - Memory Buffer Register (18 Bit)
        pub mb: u32,
        /// AC - Accumulator (18 Bit)
        pub ac: u32,
        /// IO - In-Out Register (18 Bit)
        pub io: u32,
        /// PC - Program Counter (12 Bit)
        pub pc: u32,
        /// MA - Memory Address (12 Bit)
        pub ma: u32,
        /// Instructions Register (5 Bit)
        pub ir: u32,
        /// Overflow flag
        pub ov: bool,
        /// Halt Flag
        pub halt: bool,
    }

    /// Instruction Layout Of The PDP-1
    pub struct Instruction {
        /// Memory Reference Or Augmented reference instruction
        pub instruction: u32,
        /// Indirect bit
        pub indirect: u32,
        /// Memory address bit Y
        pub address: u32
    }

    /// Decode Instruction Binary To Struct
    /// *Instruction Format*
    ///     0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17
    ///   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///   |      op      |in|              address              | memory reference
    ///   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///   <0:4> <5>    mnemonic        action  
    pub fn decode_instruction(ins: u32) -> Instruction {

        let out = Instruction {
            instruction: (ins >> 13),
            indirect: ((ins >> 12) & 0b000000001),
            address: (ins & 0b111111111111)
        };
        println!("------------------------------------");
        println!("DECODING INSTRUCTION");
        println!("------------------------------------");
        println!("MEM  -> val: {:#017b}", ins);
        println!("INST -> val: {:#04b}", out.instruction);
        println!("INDI -> val:        {:#01b}", out.indirect);
        println!("ADDR -> val:         {:#01b}", out.address);
        return out;
    }

    /// Execute Instruction
    pub fn execute_instruction(ins: Instruction, state: &mut State) {
        println!("------------------------------------");
        match ins.instruction {
            1 => {
                println!("AND Instruction");
                println!("C(AC) = C(AC) AND C(Y)");
                state.ac = state.ac & state.cm[ins.address as usize];
                state.pc += 1;
            }
            2 => {
                println!("IOR Instruction");
                println!("C(AC) = C(AC) OR C(Y)");
                state.ac = state.ac | state.cm[ins.address as usize];
                state.pc += 1;
            }
            3 => {
                println!("XOR Instruction");
                println!("C(AC) = C(AC) XOR C(Y)");
                state.ac = state.ac ^ state.cm[ins.address as usize];
                state.pc += 1;
            }
            4 => {
                println!("XCT Instruction");
                println!("Execute Instruction On C(Y)");
                let lookup = state.cm[ins.address as usize];
                let instruction = decode_instruction(lookup);
                execute_instruction(instruction, state);
                state.pc += 1;
            }
            10 => {
                println!("LAC Instruction");
                println!("C(AC) = C(Y)");
                state.ac = state.cm[ins.address as usize];
                state.pc += 1;
            }
            12 => {
                println!("DAC Instruction");
                println!("C(Y) = C(AC)");
                state.cm[ins.address as usize] = state.ac;
                state.pc += 1;
            }
            21 => {
                println!("SUB Instruction");
                println!("C(AC) = C(AC) - C(Y)");
                // Sign AC
                let signed_ac = state.ac ^ DATA_MASK;
                // One's Compliment Add
                state.ac = signed_ac + state.cm[ins.address as usize];
                // End Around Carry
                if state.ac > DATA_MASK {
                    state.ac = (state.ac + 1) & DATA_MASK;
                }
                // TODO: Implement Overflow
                state.ac = state.ac ^ DATA_MASK;
                state.pc += 1;
            }
            35 => {
                println!("IOT Instruction");
                state.pc += 1;
            }
            _ => {
                println!("Code {} Not Implemented", ins.instruction);
                println!("System Halted");
                state.halt = true;
            }
        }
        println!("------------------------------------");
        println!("PC -> val: {:#017b}", state.pc);
        println!("   -> dec: {}", state.pc);
        println!("AC -> val: {:#017b}", state.ac);
        println!("   -> dec: {}", state.ac);
        println!("IO -> val: {:#017b}", state.io);
    }

}


fn main() {

    // Test Program
    let mem_test_prog = [
        // LAC -> 01010 | 0 | 000 000 000 101
        0b010100000000000100,
        // SUB -> 10101 | 0 | 000 000 000 110
        0b101010000000000101,
        // DAC | 0 | 000 000 000 111
        0b011000000000000110,
        0,
        15,
        9,
        0
    ];

    // Main App State
    let mut state = pdp1::State {
        cm: [0; 4096],
        mb: 0,
        ac: 0,
        io: 0,
        pc: 0,
        ma: 0,
        ir: 0,
        ov: false,
        halt: false
    };

    // Add Test Program To Memory
    for i in 0..mem_test_prog.len() {
        state.cm[i] = mem_test_prog[i];
    }

    // Run Application
    while state.halt == false {
        let mem = state.cm[state.pc as usize];
        let instruction = pdp1::decode_instruction(mem);
        pdp1::execute_instruction(instruction, &mut state);
    }

    println!("------------------------------------");
    println!("Core Memory");
    println!("------------------------------------");
    for i in 0..mem_test_prog.len() {
        println!("{:04} -> dec: {}", i, state.cm[i]);
        println!("     -> bin: {:#018b}",state.cm[i]);      
    }

}
