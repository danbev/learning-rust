### Embedded examples

### Building/Running
```console
$ cargo run
```

Lets take a closer look at this app:
```console
$ readelf -W -e target/thumbv7m-none-eabi/debug/app
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00 
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x401
  Start of program headers:          52 (bytes into file)
  Start of section headers:          790956 (bytes into file)
  Flags:                             0x5000200, Version5 EABI, soft-float ABI
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         22
  Section header string table index: 20

Section Headers:
  [Nr] Name              Type            Addr     Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            00000000 000000 000000 00      0   0  0
  [ 1] .vector_table     PROGBITS        00000000 0000d4 000400 00   A  0   0  4
  [ 2] .text             PROGBITS        00000400 0004d4 0010a0 00  AX  0   0  4
  [ 3] .rodata           PROGBITS        000014a0 001580 00031c 00  AM  0   0 16
  [ 4] .data             PROGBITS        20000000 00189c 000000 00   A  0   0  4
  [ 5] .bss              NOBITS          20000000 00189c 000008 00  WA  0   0  4
  [ 6] .uninit           NOBITS          20000008 00189c 000000 00  WA  0   0  4
  [ 7] .debug_abbrev     PROGBITS        00000000 00189c 00297a 00      0   0  1
  [ 8] .debug_info       PROGBITS        00000000 004216 02a15b 00      0   0  1
  [ 9] .debug_aranges    PROGBITS        00000000 02e378 0023e0 00      0   0  8
  [10] .debug_str        PROGBITS        00000000 030758 034645 01  MS  0   0  1
  [11] .debug_pubnames   PROGBITS        00000000 064d9d 00ef2c 00      0   0  1
  [12] .debug_pubtypes   PROGBITS        00000000 073cc9 00259d 00      0   0  1
  [13] .ARM.attributes   ARM_ATTRIBUTES  00000000 076266 000032 00      0   0  1
  [14] .debug_frame      PROGBITS        00000000 076298 006c58 00      0   0  4
  [15] .debug_line       PROGBITS        00000000 07cef0 027b9d 00      0   0  1
  [16] .debug_ranges     PROGBITS        00000000 0a4a8d 0199c8 00      0   0  1
  [17] .debug_loc        PROGBITS        00000000 0be455 000272 00      0   0  1
  [18] .comment          PROGBITS        00000000 0be6c7 00006d 01  MS  0   0  1
  [19] .symtab           SYMTAB          00000000 0be734 001010 10     21 183  4
  [20] .shstrtab         STRTAB          00000000 0bf744 0000e9 00      0   0  1
  [21] .strtab           STRTAB          00000000 0bf82d 00197e 00      0   0  1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  y (purecode), p (processor specific)

Program Headers:
  Type           Offset   VirtAddr   PhysAddr   FileSiz MemSiz  Flg Align
  LOAD           0x0000d4 0x00000000 0x00000000 0x00400 0x00400 R   0x4
  LOAD           0x0004d4 0x00000400 0x00000400 0x010a0 0x010a0 R E 0x4
  LOAD           0x001580 0x000014a0 0x000014a0 0x0031c 0x0031c R   0x10
  GNU_STACK      0x000000 0x00000000 0x00000000 0x00000 0x00000 RW  0

 Section to Segment mapping:
  Segment Sections...
   00     .vector_table 
   01     .text 
   02     .rodata 
   03     
```
Notice that the `Entry Point Address` is `0x401` and this start of the .text
segment and if the function Reset:
```console
$ cargo objdump --release -- --disassemble-symbols=Reset
    Finished release [optimized] target(s) in 0.06s

app:	file format elf32-littlearm

Disassembly of section .text:

00000400 <Reset>:
     400: 80 b5        	push	{r7, lr}
     402: 6f 46        	mov	r7, sp
     404: 00 f0 aa f8  	bl	#340
     408: 40 f2 08 00  	movw	r0, #8
     40c: 40 f2 00 01  	movw	r1, #0
     410: c2 f2 00 00  	movt	r0, #8192
     414: c2 f2 00 01  	movt	r1, #8192
     418: 81 42        	cmp	r1, r0
     41a: 13 d2        	bhs	#38 <Reset+0x44>
     41c: 40 f2 00 01  	movw	r1, #0
     420: 00 22        	movs	r2, #0
     422: c2 f2 00 01  	movt	r1, #8192
     426: 41 f8 04 2b  	str	r2, [r1], #4
     42a: 81 42        	cmp	r1, r0
     42c: 3f bf        	itttt	lo
     42e: 41 f8 04 2b  	strlo	r2, [r1], #4
     432: 81 42        	cmplo	r1, r0
     434: 41 f8 04 2b  	strlo	r2, [r1], #4
     438: 81 42        	cmplo	r1, r0
     43a: 03 d2        	bhs	#6 <Reset+0x44>
     43c: 41 f8 04 2b  	str	r2, [r1], #4
     440: 81 42        	cmp	r1, r0
     442: f0 d3        	blo	#-32 <Reset+0x26>
     444: 40 f2 00 01  	movw	r1, #0
     448: 40 f2 00 00  	movw	r0, #0
     44c: c2 f2 00 01  	movt	r1, #8192
     450: c2 f2 00 00  	movt	r0, #8192
     454: 88 42        	cmp	r0, r1
     456: 0e d2        	bhs	#28 <Reset+0x76>
     458: 02 1d        	adds	r2, r0, #4
     45a: 8a 42        	cmp	r2, r1
     45c: 38 bf        	it	lo
     45e: 0a 46        	movlo	r2, r1
     460: c1 43        	mvns	r1, r0
     462: 11 44        	add	r1, r2
     464: 21 f0 03 01  	bic	r1, r1, #3
     468: 0a 1d        	adds	r2, r1, #4
     46a: 40 f2 9c 61  	movw	r1, #1692
     46e: c0 f2 00 01  	movt	r1, #0
     472: 00 f0 80 f8  	bl	#256
     476: 00 f0 01 f8  	bl	#2
     47a: fe de        	trap
```

```console
$ cargo objdump --release -- --disassemble --no-show-raw-insn
```




