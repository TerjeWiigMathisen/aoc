mov rbx,84
mov rcx,rbx
test rax,rax
jnz t0 		; jnz a 2
jmp t5		; jnz 1 5
t0:
imul rbx,100
sub rbx,-100000
mov rcx,rbx	;; set c b
sub rcx,-17000
t5:
tm23:
	mov r1,1
	mov rdx,2
tm13:
	mov r0,2
tm8:
	mov r2,rdx	;set g d
	imul r2,r0	;mul g e
	sub r2,rbx	;sub g b
	test r2,r2
	 jnz tg 		;-  jnz g 2
	mov r1,0	; set f 0
tg:
	sub r0,-1	;sub e -1
	mov r2,r0	;set g e
	sub r2,rbx	;sub g b
	test r2,r2
	 jnz tm8		;jnz g -8

	sub rdx,-1	;sub d -1
	mov r2,rdx	;set g d
	sub r2, rbx	;sub g b
	test r2,r2
	 jnz tm13	;jnz g -13
	 
	test r1,r1
	 jnz tp2		;jnz f 2
	 
	sub r3,-1	; sub h -1
	mov r2,rbx	;set g b
	sub r2,rcx	;sub g c
	test r2,r2
	 jnz tg2		;jnz g 2
jmp done	; jnz 1 3
tg2:
	sub rbx,-17	;sub b -17
jmp tm23	;jnz 1 -23
done: