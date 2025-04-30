bits 16
; beginning of bootloader
mov ax, 0x7C0
; start data segment to beginning of bootloader
mov ds, ax
; end of beginning of ds
mov ax, 0x7E0
; top of stack 
mov ss, ax
; set stack pointer to addr
mov sp, 0x2000

clearscreen:
    push bp
    mov bp, sp
    pusha

    mov ah, 0x07        ; tells BIOS to scroll down window
    mov al, 0x00        ; clear entire window
    mov bh, 0x07            ; white on black
    mov cx, 0x00        ; specifies top left of screen as (0,0)
    mov dh, 0x18        ; 18h = 24 rows of chars
    mov dl, 0x4f        ; 4fh = 79 cols of chars
    int 0x10        ; calls video interrupt

    popa
    mov sp, bp
    pop bp
    ret
