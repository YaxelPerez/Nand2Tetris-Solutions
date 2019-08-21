// This file is part of the materials accompanying the book 
// "The Elements of Computing Systems" by Nisan and Schocken, 
// MIT Press. Book site: www.idc.ac.il/tecs
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[3], respectively.)

// Put your code here.

//if a=0, goto END
@R0
D=M
@END
D;JEQ

// if b=0, goto END
@R1
D=M
@END
D;JEQ

@0
D=A
@R2
M=D

// if b>0, step = -1, else step = 1
@R1
D=M
@B_GTZ
D;JGT
@B_LTZ
D;JLT

(B_GTZ)
@R1
D=M
@END
D;JEQ

@R0
D=M
@R2
M=D+M

@R1
M=M-1

@B_GTZ
0;JMP

(B_LTZ)
@R1
D=M
@END
D;JEQ

@R0
D=M
@R2
M=D-M

@R1
M=M+1

@B_LTZ
0;JMP

(END)
