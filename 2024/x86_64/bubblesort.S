.intel_syntax noprefix

.global bubblesort

bubblesort:
	xor r9, r9
	xor rcx, rcx

bubblesort_compare:
	mov eax, [rsi + r9 * 4]
	mov ebx, [rsi + r9 * 4 + 4]

	inc r9
	cmp r8, r9
	je  bubblesort_done

	cmp eax, ebx
	jg  bubblesort_swap
	jmp bubblesort_compare

bubblesort_swap:
	inc rcx
	mov [rsi + r9 * 4 - 4], ebx
	mov [rsi + r9 * 4], eax
	jmp bubblesort_compare

bubblesort_done:
	test rcx, rcx
	jne  bubblesort

	ret
