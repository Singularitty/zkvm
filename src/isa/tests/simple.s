MOV X0, 10

loop:
    ADD X1, X1, X0
    ADDI X0, X0, -1
    JZ X0, end
    JMP loop

end:
    HALT