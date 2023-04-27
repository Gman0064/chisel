BITS 64

SECTION .text
global main

main: 
	push rax
	push rcx
	push rdx
	push rsi
	push rdi
	push r11

	mov rax,1
	mov rdi,1
	lea rsi,[rel $+hello-$]
	mov rdx,[rel $+len-$]
	syscall

	pop r11
	pop rdi
	pop rsi
	pop rdx
	pop rcx
	pop rax

	;push 0x1060	; jump to original entry point
	;				; could be replaced dynamically?
	;ret

hello: db "Calling injected code >:)",10
len: dd 26