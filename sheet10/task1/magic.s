magic:
    push    rbp         # save base stack pointer
    mov     rbp, rsp    # move value from stack pointer register to base stack pointer (64-bit)
    mov     rcx, rdi    # move value from first function parameter to counter register (64-bit)
    mov     esi, 2      # move value '2' into esi, source register (32-bit)
.LBB0_1:                # while loop
    mov     al, 1       # move value '1' into accumulator register (makes sure 'true' is returned
                        # if next conditional jump is successful)
    cmp     rsi, rcx    # compare values from counter and source register, sets Z-flag to '0'
                        # if they are not equal, else it is set to '1'
    jae     .LBB0_4     # conditional jump to the end of the function when Z-flag is '1'
    xor     edx, edx    # bit-wise XOR of data register (32-bit), sets edx to '0'
    mov     rax, rcx    # move value from counter to accumulator register
    div     rsi         # divide value in accumulator by rsi
                        # quotient stored in eax, remainder stored in edx
    inc     rsi         # increment value in rsi
    test    rdx, rdx    # bit-wise AND of data register, sets Z-flag to '1' except if
                        # the value in rdx is '0', then the Z-flag becomes '0'
    jne     .LBB0_1     # conditional jump back to 'LBB0_1' if Z-flag is '0' (not equal)
    xor     eax, eax    # bit-wise XOR of accumulator register (32-bit), sets eax to '0'
                        # (makes sure 'false' is returned)
.LBB0_4:
    pop     rbp         # gets former value of rbp from stack and stores it in rbp
    ret                 # return to program


# rough draft of what the function "might" look like
fn magic(param: u64) -> bool {
    let mut a = param;
    let mut b = 2;
    let mut div = 0;

    do  {
        if b >= a {
            true
        }
        div = a % b;
        a /= b;
        b += 1;
    } while (div != 0)

    false
}
