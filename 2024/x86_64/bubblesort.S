.intel_syntax noprefix

.global bubblesort

bubblesort:
	xor r9, r9
	mov rcx, 1

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
	xor rcx, rcx
	mov [rsi + r9 * 4 - 4], ebx
	mov [rsi + r9 * 4], eax
	jmp bubblesort_compare

bubblesort_done:
	test rcx, rcx
	je   bubblesort

	ret
