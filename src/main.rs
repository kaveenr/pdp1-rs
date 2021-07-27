#![crate_name = "pdp1_rs"]
/// Core Emulation Module
mod pdp1 {

    /// Main Emulated State Of The PDP-1
    pub struct State {
        /// CM - Core Memory (4096 of 18 Bit Words)
        cm: [i32; 4096],
        /// MB - Memory Buffer Register (18 Bit)
        mb: i32,
        /// AC - Accumulator (18 Bit)
        ac: i32,
        /// IO - In-Out Register (18 Bit)
        io: i32,
        /// PC - Program Counter (12 Bit)
        pc: i16,
        /// MA - Memory Address (12 Bit)
        ma: i16
        /// Instructions Register (5 Bit)
        ir: i8
    }
}


fn main() {
    println!("Hello, world!");
}
