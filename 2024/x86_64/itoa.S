.intel_syntax noprefix

.global itoa

itoa:
	mov rsi, 9

itoa_convert:
	xor rdx, rdx
	div rbx
	add rdx, '0'

	dec rsi
	mov [rdi + rsi], dl

	test rax, rax
	jne  itoa_convert

itoa_done:
	mov rcx, 9
	sub rcx, rsi
	ret
