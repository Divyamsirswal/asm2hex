[bits 64]
section .text
    global _start

_start:
    ; Write "Hello, World!" to stdout
    mov rax, 1        ; sys_write syscall number
    mov rdi, 1        ; File descriptor 1 (stdout)
    mov rsi, message  ; Pointer to the message
    mov rdx, 13       ; Length of the message
    syscall           ; Invoke the syscall

    ; Exit the program
    mov rax, 60       ; sys_exit syscall number
    xor rdi, rdi      ; Return 0 status
    syscall           ; Invoke the syscall

section .data
message db "Hello, World!", 0xA  ; The string with newline
