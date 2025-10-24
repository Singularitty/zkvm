mov x0, 10

loop:
    add x1, x1, x0
    addi x0, x0, -1
    jz x0, end
    jmp loop

end:
    halt