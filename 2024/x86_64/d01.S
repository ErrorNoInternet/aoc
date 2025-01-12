.intel_syntax noprefix

.lcomm input, 16384
.lcomm output, 10
.lcomm left_nums, 4000
.lcomm right_nums, 4000
.lcomm counts, 100000

.global _start

_start:
	mov rsi, offset input
	mov rdx, 16383
	syscall

	xor rdx, rdx
	mov rsi, offset input

parse_numbers:
	call atoi
	test rax, rax
	je   sort_numbers
	mov  [left_nums + r8 * 4], eax

	add  rsi, 2
	call atoi
	mov  [right_nums + r8 * 4], eax
	inc  byte ptr [counts + eax]

	inc r8
	jmp parse_numbers

sort_numbers:
	mov  rsi, offset left_nums
	call bubblesort

	mov  rsi, offset right_nums
	call bubblesort

solve:
	mov ebx, [left_nums + r10 * 4]
	mov ecx, [right_nums + r10 * 4]

	xor  rdx, rdx
	mov  dl, [counts + rbx]
	imul rdx, rbx
	add  r15, rdx

	sub ebx, ecx
	mov ecx, ebx
	sar ecx, 31
	xor ebx, ecx
	sub ebx, ecx
	add r14, rbx

	inc r10
	cmp r8, r10
	jne solve

	mov  rdi, offset output
	mov  rbx, 10
	mov  rax, r14
	call itoa

	mov rax, 1
	mov rdi, 1
	add rsi, offset output
	mov rdx, rcx
	inc rdx
	mov byte ptr [rsi + rcx], '\n'
	syscall

	mov  rdi, offset output
	mov  rax, r15
	call itoa

	mov rax, 1
	mov rdi, 1
	add rsi, offset output
	mov rdx, rcx
	inc rdx
	syscall

	mov rax, 60
	xor rdi, rdi
	syscall
