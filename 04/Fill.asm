// This file is part of the materials accompanying the book 
// "The Elements of Computing Systems" by Nisan and Schocken, 
// MIT Press. Book site: www.idc.ac.il/tecs
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

// Put your code here.

// 0x4000 = 16384
// 0x4020 = 16416
// 32 total words

(MAIN_LOOP)
@16384
D=A
@R0
M=D

@24576
D=M
@FILL_LOOP
D;JNE
@CLEAR_LOOP
0;JMP

(FILL_LOOP)
@R0
D=M
@24575
D=A-D
@MAIN_LOOP
D;JLT

@0
A=!A
D=A
@R0
A=M
M=D

@R0
M=M+1

@FILL_LOOP
0;JMP

(CLEAR_LOOP)
@R0
D=M
@24575
D=A-D
@MAIN_LOOP
D;JLT

@0
D=A
@R0
A=M
M=D

@R0
M=M+1

@CLEAR_LOOP
0;JMP
