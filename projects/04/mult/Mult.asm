// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Put your code here.
// Initialization, sum=0, R2=0
@sum
M=0
@2
M=0

// Loop
(LOOP)
// If R1 > 0, Jump to (ADD)
@1
D=M
@ADD
D;JGT
// else, jump to (END)
@END
0;JMP

(ADD)
// sum = sum+R1, R1= R1-1
@0
D=M // D = R1
@sum
M=M+D // sum = sum + D
// Set R2=sum
D=M // D = sum
@2
M=D
@1
M=M-1
// Back to (LOOP)
@LOOP
0;JMP

(END)
@END
0;JMP
