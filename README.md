# Azeez Daoud - Assembly Language - VIVEC
Named after the Lord Vivec (one of the God-kings of Morrowind).

# Registers
There are two sets of registers, **Directly** and **Indirectly** Manipulated Registers (DMR and IMR). There are 2 DMRs and 15+1 IMRs ($0 is constant).

The idea is to use the DMRs to do calculations then save them in the IMRs to reuse them again by putting them in the DMRs.

The DMRs are accessed using `#0` and `#1`,
## INDIRECTLY MANIPULATED REGISTERS (IMR)
| Register(s)   | Description                   |
|---            |---                            |
|`$0`           |Always equals to 0             |
|`$1...$10`     |Registers for general usage    |
|`$11`          |Result of a system call        |
|`$12, $13`     |Argument 0-1 for system calls  |
|`$14, $15`     |Assembler temporaries          |


# Instructions
There are two main types of instructions. **Branching** and **Manipulation** Instructions. Branching instructions are used to branch in the instructions and manipulation instructions are for manipulating the registers. There exists the system call instruction which is a mix of both.

| Instruction | Syntax          | Operation                                                                             |
|---          |---              |---                                                                                    |
|`set`        |`set #DMR IMM`   |  `#DMR := immediate`                                                                  |
|`mov`        |`mov #DMR $IMR`  |  `$IMR := #DMR`                                                                       |
|`get`        |`get #DMR $IMR`  |  `#DMR := $IMR`                                                                       |
|`cmp`        |`cmp #DMR $IMR`  |  `($11, $12) := if #DMR > $IMR then (1,0) else if #DMR < $IMR then (0,1) else (0,0)`  |
|`jmp`        |`jmp label`      |  `PC := label`                                                                        |
|`jie`        |`jio label`      |  `PC := if ($14, $15) == (0,0) then label else PC`                                    | 
|`jig`        |`jig label`      |  `PC := if ($14, $15) == (1,0) then label else PC`                                    |
|`cal`        |`cal callcode`   |  See table below                                                                      |

| Name      | Code  | Parameters                        | 
|---        |---    |---                                |
|exit       |0      | None                              |    
|add        |1      | `$11 = $12 + $13`                 |
|sub        |2      | `$11 = $12 - $13`                 |
|and        |3      | `$11 = $12 & $13`                 |
|or         |4      | `$11 = $12 | $13`                 |
|xor        |5      | `$11 = $12 ^ $13`                 |
|not        |6      | `$11 = ¬$12`                      |
|increment  |7      | `$11 = $12 + 1`                   |
|decrement  |8      | `$11 = $12 - 1`                   |
|get integer|16     | `$11 = next 32-bit int from stdin`|
|put integer|17     | write value of $12 to `stdout`    |
