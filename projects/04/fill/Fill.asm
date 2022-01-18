// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
(LOOP)
    @24576
    D=M
    @BLACK
    D;JGT
    @WHITE
    0;JMP
(BLACK)
    @16384
    D=A
    @pixels
    M=D
    (BLACKLOOP)
        // Draw pixel[A] black
        @pixels
        A=M
        M=-1
        // Repeat until finish
        @pixels
        M=M+1
        D=M
        @24576
        D=A-D
        @BLACKLOOP
        D;JGT
        @LOOP
        0;JMP
(WHITE)
    @16384
    D=A
    @pixels
    M=D
    (WHITELOOP)
        // Draw pixel[A] black
        @pixels
        A=M
        M=0
        // Repeat until finish
        @pixels
        M=M+1
        D=M
        @24576
        D=A-D
        @WHITELOOP
        D;JGT
        @LOOP
        0;JMP
