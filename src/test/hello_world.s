	.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	2, %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
