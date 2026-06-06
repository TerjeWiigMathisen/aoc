set b 84
set c b
jnz a t0
jnz 1 t1
t0:
mul b 100
sub b -100000
set c b
sub c -17000

t1:
tm23:
set f 1
set d 2

tm13:
set e 2
tm8:
set g d
mul g e
sub g b
 jnz g t2
set f 0
t2:
sub e -1
set g e
sub g b
 jnz g tm8
 
sub d -1
set g d
sub g b
jnz g tm13

 jnz f t3
sub h -1
t3:
set g b
sub g c
 jnz g t4
 jmp done
t4:
sub b -17
 jmp tm23
done: