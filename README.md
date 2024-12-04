# Koge29_H8-3069F_Emulator

## How to Run example
```
cargo run --release -- --elf=./example/one.elf -m
```

## Implemented

<details><summary>Instructions</summary>

| Instruction | Implemented                            |
| ----------- | -------------------------------------- |
| MOV         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| POP         | <ul><li>[x] W<li> [x] L</ul>           |
| PUSH        | <ul><li>[x] W<li> [x] L</ul>           |
| MOVFPE      | <ul><li>[ ] B</ul>                     |
| MOVTPE      | <ul><li>[ ] B</ul>                     |
| ADD         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| CMP         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| SUB         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| ADDX        | <ul><li>[x] B</ul>                     |
| SUBX        | <ul><li>[ ] B</ul>                     |
| ADDS        | <ul><li>[x] L</ul>                     |
| SUBS        | <ul><li>[x] L</ul>                     |
| INC         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| DEC         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| DAA         | <ul><li>[ ] B</ul>                     |
| DAS         | <ul><li>[ ] B</ul>                     |
| MULXU       | <ul><li>[x] B<li> [x] W</ul>           |
| DIVXU       | <ul><li>[x] B<li> [x] W</ul>           |
| MULXS       | <ul><li>[ ] B<li> [ ] W</ul>           |
| DIVXS       | <ul><li>[ ] B<li> [ ] W</ul>           |
| NEG         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| EXTU        | <ul><li>[x] W<li> [x] L</ul>           |
| EXTS        | <ul><li>[ ] W<li> [ ] L</ul>           |
| AND         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| OR          | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| XOR         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| SHAL        | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| SHAR        | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| SHLL        | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| SHLR        | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| ROTL        | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| ROTR        | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| ROTXL       | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| ROTXR       | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| BSET        | <ul><li>[x] B</ul>                     |
| BCLR        | <ul><li>[x] B</ul>                     |
| BNOT        | <ul><li>[x] B</ul>                     |
| BTST        | <ul><li>[x] B</ul>                     |
| BAND        | <ul><li>[x] B</ul>                     |
| BIAND       | <ul><li>[x] B</ul>                     |
| BOR         | <ul><li>[x] B</ul>                     |
| BIOR        | <ul><li>[x] B</ul>                     |
| BXOR        | <ul><li>[x] B</ul>                     |
| BIXOR       | <ul><li>[x] B</ul>                     |
| BLD         | <ul><li>[x] B</ul>                     |
| BILD        | <ul><li>[x] B</ul>                     |
| BST         | <ul><li>[x] B</ul>                     |
| BIST        | <ul><li>[x] B</ul>                     |
| Bcc         | <ul><li>[x] </ul>                      |
| BSR         | <ul><li>[x] </ul>                      |
| JMP         | <ul><li>[x] </ul>                      |
| JSR         | <ul><li>[x] </ul>                      |
| RTS         | <ul><li>[x] </ul>                      |
| TRAPA       | <ul><li>[x] </ul>                      |
| RTE         | <ul><li>[x] </ul>                      |
| SLEEP       | <ul><li>[ ] </ul>                      |
| LDC         | <ul><li>[ ] B<li> [ ] W<li> [ ] L</ul> |
| STC         | <ul><li>[x] B<li> [x] W<li> [x] L</ul> |
| ANDC        | <ul><li>[ ] B</ul>                     |
| ORC         | <ul><li>[ ] B</ul>                     |
| XORC        | <ul><li>[ ] B</ul>                     |
| NOP         | <ul><li>[ ] </ul>                      |
| Block       | <ul><li>[ ] B<li> [ ] W<li> [ ] L</ul> |

</details>

## License

Copyright 2023 Kogepan229</br>
Koge29_H8-3069F_Emulator is licenced under the Apache License Version 2.0. See the [LICENSE](https://github.com/Kogepan229/Koge29_H8-3069F_Emulator/blob/main/LICENSE) file for details.
