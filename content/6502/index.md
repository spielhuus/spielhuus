+++
title = '6502'
description = 'Simulating the 6502 processor'
date = 2025-12-20T16:00:00+02:00
draft = true
tags = ['emulation']
links = ["6502", "nesdev"]
+++

the 6502 microprocessor was used in various home computer and game consoles like the c64 or the nintendo entertainment system.


The MOS Technology 6502 processor has a small, efficient set of six user-visible registers. The table below lists each register with its bit size and a description of its function.

### Register List

| Register Name | Abbr. | Size | Description |
| :--- | :---: | :---: | :--- |
| **Accumulator** | **A** | 8 bits | The primary register for arithmetic and logical operations. It is the destination for the results of most math instructions and is used for data transfer to and from memory. |
| **Index Register X** | **X** | 8 bits | General-purpose register often used as a counter or an offset for indexed addressing modes. It has a special function allowing it to copy data to/from the Stack Pointer. |
| **Index Register Y** | **Y** | 8 bits | General-purpose register similar to X, used for counters and offsets. It is specifically required for certain addressing modes (e.g., Indirect Indexed `(zp),Y`). |
| **Program Counter** | **PC** | 16 bits | Holds the memory address of the next instruction to be executed. It automatically increments as instructions are fetched and is modified by jumps, branches, and subroutine calls. |
| **Stack Pointer** | **S** / **SP**| 8 bits | Holds the low byte of the next free location on the stack. The 6502 stack is hardwired to memory page 1 (`$0100`–`$01FF`). Therefore, if `S` holds `$FF`, the next stack operation occurs at `$01FF`. |
| **Processor Status** | **P** / **SR**| 8 bits | A collection of flag bits that indicate the results of operations (like zero or negative) and control processor modes (like interrupt disable). |

---

### Processor Status Register (P) Breakdown

The **Processor Status** register (also called the Status Register or Flags Register) contains 7 active bits. Each bit has a specific meaning regarding the state of the processor.

| Bit | Flag Name | Symbol | Description |
| :---: | :--- | :---: | :--- |
| **7** | Negative | **N** | Set if the result of the last operation had bit 7 set (interpreting the byte as a negative signed integer). |
| **6** | Overflow | **V** | Set if a signed arithmetic operation resulted in a value too large to fit in a signed byte (overflow). |
| **5** | *Unused* | - | Not physically implemented as a distinct flip-flop but usually reads as `1`. |
| **4** | Break Command| **B** | Set mostly virtually when the `BRK` instruction is executed, distinguishing software interrupts from hardware interrupts. |
| **3** | Decimal Mode | **D** | If set, `ADC` and `SBC` instructions treat data as Binary Coded Decimal (BCD). If clear, they operate in standard binary. |
| **2** | Interrupt Disable| **I** | If set, the processor ignores standard hardware interrupts (IRQ). It does not prevent Non-Maskable Interrupts (NMI). |
| **1** | Zero | **Z** | Set if the result of the last operation was zero. |
| **0** | Carry | **C** | Set if the last arithmetic operation resulted in a carry out (for addition) or required a borrow (for subtraction), or during shift operations. |



### NES CPU Memory Map (16-bit Bus)
| Start  | End    | Size   | Description                                 | Notes                                      |
| :----- | :----- | :----- | :------------------------------------------ | :----------------------------------------- |
| 0x0000 | 0x07FF | 2 KB   | **Internal RAM**                            | Real physical RAM                          |
| 0x0800 | 0x1FFF | 6 KB   | **Mirrors of RAM**                          | Mirrors 0x0000-0x07FF (Repeats every 2KB)  |
| 0x2000 | 0x2007 | 8 B    | **PPU Registers**                           | Mapped I/O for PPU                         |
| 0x2008 | 0x3FFF | 8 KB   | **Mirrors of PPU Regs**                     | Mirrors 0x2000-0x2007 (Repeats every 8 B)  |
| 0x4000 | 0x4017 | 24 B   | **APU & I/O Registers**                     | Sound, Controllers, DMA                    |
| 0x4018 | 0x401F | 8 B    | **CPU Test Mode**                           | Normally disabled                          |
| 0x4020 | 0xFFFF | ~48 KB | **Cartridge Space**                         | PRG-ROM, PRG-RAM, Mapper Registers         |

### CPU Interrupt Vectors (at end of Cartridge Space)
| Address | Description |
| :------ | :---------- |
| 0xFFFA  | NMI Vector  |
| 0xFFFC  | Reset Vector|
| 0xFFFE  | IRQ/BRK Vector |


### NES Cartridge Space Layout (CPU View)

| Start  | End    | Size  | Name              | Description                                      | Offset Calculation (for array index) |
| :----- | :----- | :---- | :---------------- | :----------------------------------------------- | :----------------------------------- |
| 0x4020 | 0x5FFF | ~8 KB | **Expansion Area**| Rarely used. Sometimes Mappers map registers here.| `(Usually Open Bus)`                 |
| 0x6000 | 0x7FFF | 8 KB  | **SRAM / WRAM**   | Save RAM (e.g., Zelda battery save) or Work RAM. | `addr - 0x6000`                      |
| 0x8000 | 0xBFFF | 16 KB | **PRG-ROM Lower** | The first 16KB of game code.                     | `addr - 0x8000`                      |
| 0xC000 | 0xFFFF | 16 KB | **PRG-ROM Upper** | The last 16KB of game code (contains Vectors).   | `addr - 0x8000`                      |

---

### Handling NROM (Mapper 0) Mirroring
*Applies to 0x8000 - 0xFFFF*

| Game Size | Logical Mapping | Implementation Logic |
| :-------- | :-------------- | :------------------- |
| **32 KB** | **0x8000 - 0xFFFF** is one continuous block. | `return prgRom[addr - 0x8000];` |
| **16 KB** | **0xC000 - 0xFFFF** is a mirror of **0x8000**. | `return prgRom[(addr - 0x8000) % 16384];` |


### PPUCTRL (0x2000)

| Bit | Name | Description |
| :-- | :--- | :---------- |
| **7** | **Generate NMI** | **7** 6 5 4 3 2 1 0<br>Enable NMI at the start of the Vertical Blanking Interval (VBlank).<br>0 = Off<br>1 = On |
| **6** | **PPU Master/Slave** | 7 **6** 5 4 3 2 1 0<br>Selects PPU master/slave mode.<br>0 = Read backdrop from EXT pins<br>1 = Output color on EXT pins<br>*(Note: Usually unused in standard NES games)* |
| **5** | **Sprite Size** | 7 6 **5** 4 3 2 1 0<br>Size of sprites.<br>0 = 8x8 pixels<br>1 = 8x16 pixels |
| **4** | **Background Pattern Table** | 7 6 5 **4** 3 2 1 0<br>Pattern table address for background.<br>0 = $0000<br>1 = $1000 |
| **3** | **Sprite Pattern Table** | 7 6 5 4 **3** 2 1 0<br>Pattern table address for 8x8 sprites.<br>*(Ignored in 8x16 mode)*<br>0 = $0000<br>1 = $1000 |
| **2** | **VRAM Increment** | 7 6 5 4 3 **2** 1 0<br>Amount to add to VRAM address after access via $2007.<br>0 = Add 1 (going across)<br>1 = Add 32 (going down) |
| **1-0** | **Base Nametable Address** | 7 6 5 4 3 2 **1 0**<br>Base nametable address (most significant 2 bits of scrolling position).<br>00 = $2000<br>01 = $2400<br>10 = $2800<br>11 = $2C00 |

