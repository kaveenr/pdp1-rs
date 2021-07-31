# PDP1 RS

Trying to write an PDP-1 Emulator In Rust.

## Documentation
```sh
cargo doc
```

# Notes

Quick run through of the PDP-1.

## Memory System
- Holds 4096 words, with a word length of 18 bits.

## Number System
- Fixed point machine using binary arithmetic.
- Negative numbers are represented with one's complement.
  - Bit 0 is the sign bit, zero for positive.
  - Bit 1 MSB, BIT 17 LSB
  - -0 is converted to +0 in certain operators.

## Instruction Format
- Bit 0..4 instruction code. Instructions falls under,
  - Memory Reference Instructions.
  - Augmented Instructions.
- Bit 5 indirect address bit.
- Bit 6..17 memory address Y.
- Indirect Addressing
  - A memory reference instruction with bit 5 as 1 denotes it
  - ???


## References
###  Bits, Bytes & Words
- Bit -> Binary digit 0 or 1
- Byte -> Sequence of 8 bits
- Word -> Sequence of N bits

### Low Memory Test Program
```json
[
    720074,
    760007,
    700024,
    240023,
    640001,
    600014,
    440023,
    250023,
    520024,
    600006,
    760011,
    600002,
    440023,
    230023,
    530023,
    760401,
    520024,
    600014,
    600002,
    0,
    7777
]
```

## External Referrences
- [Handbook](http://www.bitsavers.org/pdf/dec/pdp1/F15D_PDP1_Handbook_Oct63.pdf)
- [Software Paper Tapes](https://github.com/hrvach/fpg1/tree/master/paper_tapes)
- [Test Program](https://www.computerhistory.org/pdp-1/_media/pdf/DEC.pdp_1.1963.102650493.pdf)
