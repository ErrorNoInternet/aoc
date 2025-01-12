.intel_syntax noprefix

.global quicksort

quicksort:
	mov rbp, rsp
	xor rax, rax
	mov rbx, r8
	dec rbx

quicksort_loop:
	mov rcx, rax
	dec rcx
	mov edx, [rsi + rbx * 4]
	mov rdi, rcx

partition_loop:
	inc rdi

	cmp rdi, rbx
	je  partition_done

	cmp [rsi + rdi * 4], edx
	jg  partition_loop

	inc rcx
	mov r9d, [rsi + rcx * 4]
	mov r10d, [rsi + rdi * 4]
	mov [rsi + rcx * 4], r10d
	mov [rsi + rdi * 4], r9d

	jmp partition_loop

partition_done:
	mov r9d, [rsi + rcx * 4 + 4]
	mov r10d, [rsi + rbx * 4]
	mov [rsi + rcx * 4 + 4], r10d
	mov [rsi + rbx * 4], r9d

	cmp rcx, rax
	jle check_right

	push rax
	push rcx

check_right:
	inc rcx
	cmp rcx, rbx
	jge pop_stack

	push rcx
	push rbx

pop_stack:
	cmp rbp, rsp
	je  quicksort_done

	pop rbx
	pop rax
	jmp quicksort_loop

quicksort_done:
	ret
