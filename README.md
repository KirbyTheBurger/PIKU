# Introduction
PIKU stands for PIKU is kinda useless. It is a 16-bit CPU architecture with a small custom instruction set.  

# Memory layout
**Memory map:**
|    Region    | Purpose |
|--------------|---------|
|0x0000-0x3FFF |Program code (16KB)|
|0x4000-0xFBFF |Static memory (47KB)|
|0xFC00-0xFFFF |Stack (1KB)|

**Registers:**
| Register | Purpose |
|----------|---------|
|r0-r7     |General purpose registers|
|pc        |Program counter|
|sp        |Stack pointer|

# Instructions  
**Operands:**  
| Operand | Meaning |
|---------|---------|
|rx       |Register x|
|n        |Number   |
|[rx]     |Value at memory adress stored in rx|
|[n]      |Value at memory adress n|
|l        |Label    |

**Instructions:**  
| Opcode | Mnemonic | Operands  |
|--------|----------|-----------|
| 0x00   | HLT      |           |
| 0x04   | LD       | rx, ry    |
| 0x05   | LD       | rx, n     |
| 0x06   | LD       | rx, [ry]  |
| 0x07   | LD       | rx, [n]   |
| 0x08   | ST       | [rx], ry  |
| 0x0C   | ADD      | rx, ry    |
| 0x0D   | ADD      | rx, n     |
| 0x10   | SUB      | rx, ry    |
| 0x11   | SUB      | rx, n     |
| 0x14   | MUL      | rx, ry    |
| 0x18   | DIV      | rx, ry    |
| 0x1C   | NOT      | rx        |
| 0x20   | AND      | rx, ry    |
| 0x21   | AND      | rx, n     |
| 0x24   | OR       | rx, ry    |
| 0x25   | OR       | rx, n     |
| 0x28   | XOR      | rx, ry    |
| 0x29   | XOR      | rx, n     |
| 0x2C   | LSH      | rx, ry    |
| 0x2D   | LSH      | rx, n     |
| 0x30   | RSH      | rx, ry    |
| 0x31   | RSH      | rx, n     |
| 0x34   | JMP      | l         |
| 0x38   | CMP      | rx, ry    |
| 0x39   | CMP      | rx, n     |
| 0x3C   | JEQ      | l         |
| 0x40   | JNE      | l         |
| 0x44   | JLT      | l         |
| 0x48   | JLE      | l         |
| 0x4C   | JGT      | l         |
| 0x50   | JGE      | l         |
| 0x54   | OUT      | rx        |
| 0x55   | OUT      | n         |
| 0x58   | IN       | rx        |
| 0x5C   | PUSH     | rx        |
| 0x5D   | PUSH     | n         |
| 0x60   | POP      | rx        |
| 0x64   | CALL     | l         |
| 0x68   | RET      |           |
| 0x6C   | LDS      | rx, n     |
| 0x70   | STS      | n, rx     |
| 0x74   | INC      | rx        |
| 0x78   | DEC      | rx        |
| 0x7C   | MOD      | rx, ry    |
| 0x80   | JC       | l         |
| 0x84   | JNC      | l         |

**Mnemonics:**  
| Mnemonic | Meaning |
|----------|---------|
|HLT       |Stop the current program|
|LD        |Load value into register|
|ST        |Store value in memory|
|ADD       |Add a value to another value|
|SUB       |Subtract a value from another|
|MUL       |Multiply a value by another|
|DIV       |Divide a value by another|
|NOT       |Bitwise NOT|
|AND       |Bitwise AND|
|OR        |Bitwise OR|
|XOR       |Bitwise XOR|
|LSH       |Left shift|
|RSH       |Right shift|
|JMP       |Continue execution at a label|
|CMP       |Compare 2 values and set flags|
|JEQ       |Jump if equal|
|JNE       |Jump if not equal|
|JLT       |Jump if less than|
|JLE       |Jump if less than or equal|
|JGT       |Jump if greater than|
|JGE       |Jump if greater than or equal|
|OUT       |Output a single character|
|IN        |Wait for a single character input|
|PUSH      |Push value to stack|
|POP       |Pop value from stack|
|CALL      |Call a function|
|RET       |Return from a function|
|LDS       |Load value into register relative to SP|
|STS       |Store value in stack memory relative to SP|
|INC       |Increment register by 1|
|DEC       |Decrement register by 1|
|MOD       |Modulo operator|
|JC        |Jump if carry flag is set|
|JNC       |Jump if carry flag is not set|
