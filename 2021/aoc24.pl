#!perl -w

use strict;
use Time::HiRes qw (time);
use List::PriorityQueue;
use warnings;
#no warnings 'recursion';

my $start = time;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $prog = join("\n",@inp);
my @progs = split(/inp w\n/, $prog);
my @c = ();
for (my $p = 1; $p <= 14; $p++) {
	push(@c,compile($p, $progs[$p]);
}
printf("%s\n", join("\n", @c));
die;

my $DEBUG = 0;
my $part1;
my $part2;

my %regs = ('w' => 0, 'x' => 0, 'y' => 0, 'z' => 0);
my @data = ((1) x 14);

#test(0,split(//,"91699391111111"));
my %pre4 = ();
my $pre7;
for (my $w = 9999999; $w >= 1111111; $w--) {
	next if ($w =~ /0/);
	@data = (split(//,$w));
	my $comp = compopt(@data);
	if ($comp < 26) {
		$pre4{$comp}=$w;
	}
}
my @pre4 = sort {$a<=>$b} keys %pre4;
my @p4 = ();
foreach (@pre4) {
	printf(STDERR " %d:%d",$_,$pre4{$_});
	push(@p4,$pre4{$_});
}
printf("Found %d prefix4\n", scalar(@p4));

%pre4 = ();
foreach (@p4) {
	$pre7 = $_;
	printf("Trying %d prefix\n", $pre7);
	for (my $w = 1111111; $w <= 9999999; $w++) {
		next if ($w =~ /0/);
		@data = (split(//,$pre7.$w));
		my $comp = compopt(@data);
		if ($comp == 0) {
			$pre4{$comp}=$w;
			printf("%s: %d\n",join("",@data),$comp);
			die;
		}
	}
}
die;
my @pre4 = sort {$a<=>$b} keys %pre4;
my @p4 = ();
foreach (@pre4) {
	printf(STDERR " %d:%d",$_,$pre4{$_});
	push(@p4,$pre4{$_});
}
printf("Found %d prefix4\n", scalar(@p4));

%pre4 = ();
foreach (@p4) {
	my $pre = $_;
	for (my $w = 111; $w <= 999; $w++) {
		next if ($w =~ /0/);
		@data = (split(//,$pre.$w));
		my $comp = compopt(@data);
		if ($comp < 100) {
			$pre4{$comp}=$pre.$w;
		}
	}
}
@pre4 = sort {$a<=>$b} keys %pre4;
@p4 = ();
foreach (@pre4) {
	printf(STDERR " %d:%d",$_,$pre4{$_});
	push(@p4,$pre4{$_});
}
printf("Found %d prefix4\n", scalar(@p4));

die;

sub val
{
	my ($v) = @_;
	return defined($regs{$v}) ? $regs{$v} : $v;
}

sub src
{
	my ($v) = @_;
	return defined($regs{$v}) ? $v : $v;
}

sub dmp
{
	my ($opc, @data) = @_;
	printf(STDERR "%10s ",$opc);
	foreach (sort keys %regs) {
		printf(STDERR "%s:%10d ", $_, $regs{$_});
	}
	printf(STDERR "%s\n",join("",@data));
}

sub test
{
	my ($z,@data) = @_;
	$regs{z} = $z;
	foreach (@inp) {
		next if (/#/);
		my ($op, $a, $b) = split;
		if ($op eq 'inp') {
			my $i = shift @data;
			return ($regs{x},$regs{y},$regs{z}) unless (defined($i));
			$regs{$a} = $i;
		}
		elsif ($op eq 'add') {
			$regs{$a} += val($b);
		}
		elsif ($op eq 'mov') {
			$regs{$a} = val($b);
		}
		elsif ($op eq 'mul') {
			$regs{$a} *= val($b);
		}
		elsif ($op eq 'div') {
			my $i = val($b);
			if ($i == 0) {
				return scalar(@data);
			}
			$regs{$a} = int($regs{$a} / val($b));
		}
		elsif ($op eq 'mod') {
			my $i = val($b);
			my $j = $regs{$a};
			if ($i <= 0 || $j < 0) {
				return scalar(@data);
			}
			$regs{$a} = $regs{$a} % val($b);
		}
		elsif ($op eq 'eql') {
			$regs{$a} = $regs{$a} == val($b);
		}
		elsif ($op eq 'ne') {
			$regs{$a} = $regs{$a} == val($b);
		}
		dmp($_, @data) if ($DEBUG);
	}
	return $regs{z};
}

my @progs = split(/inp w\n/, join("\n",@inp));

print $progs[0];

sub test1
{
	my ($z,$w, $prog) = @_;
	$regs{z} = $z; $regs{w} = $w;
	foreach (split(/\n/, $prog)) {
		next if (/#/);
		my ($op, $a, $b) = split;
		if ($op eq 'inp') {
			my $i = shift @data;
			return ($regs{x},$regs{y},$regs{z}) unless (defined($i));
			$regs{$a} = $i;
		}
		elsif ($op eq 'add') {
			$regs{$a} += val($b);
		}
		elsif ($op eq 'mov') {
			$regs{$a} = val($b);
		}
		elsif ($op eq 'mul') {
			$regs{$a} *= val($b);
		}
		elsif ($op eq 'div') {
			my $i = val($b);
			$regs{$a} = int($regs{$a} / val($b));
		}
		elsif ($op eq 'mod') {
			$regs{$a} %= val($b);
		}
		elsif ($op eq 'eql') {
			$regs{$a} = $regs{$a} == val($b);
		}
		dmp($_, @data) if ($DEBUG);
	}
	return $regs{z};
}

my ($w,$x,$y,$z) = (0,0,0,0);

sub fastmul
{
	my ($z,$w,$zdiv, $xadd,$yadd) = @_;
	$x = $z % 26;
	$z = int($z / $zdiv);
	$z *= 26;
	$z += $w+$yadd;
	return $z;
}

sub fast1
{
	my ($z,$w,$zdiv, $xadd,$yadd) = @_;
	$z += $w+$yadd;
	return $z;
}


sub fastmod
{
	my ($z,$w,$zdiv, $xadd,$yadd) = @_;
	$x = $z % 26;
	$z = int($z / $zdiv);
	$x += $xadd;
	$x = $x != $w;
	$y = 25*$x+1;
	$z *= $y;
	$y = $w+$yadd;
	$y *= $x;
	$z += $y;
	return $z;
}

for (my $w = 1; $w <= 9; $w++) {
	my $iz = test1(0,$w,$progs[1]);
	my $fz = fast1(0,$w,1,13,3);
	printf("w: %d iz: %d, fz: %d\n", $w, $iz, $fz);
	die("Wrong emulation!\n") if ($iz != $fz);
}
my %z = ();
for (my $z = 4; $z <= 12; $z++) {
	for (my $w = 1; $w <= 9; $w++) {
		my $iz = test1($z,$w,$progs[2]);
		my $fz = fast1($z,$w,1,11,12);
		printf("w: %d iz: %d, fz: %d\n", $w, $iz, $fz);
		die("Wrong emulation!\n") if ($iz != $fz);
		$z{$z}++;
	}
}
my @z = sort {$a <=> $b} keys %z;
%z = ();
foreach (@z) {
	$z = $_;
	for (my $w = 1; $w <= 9; $w++) {
		my $iz = test1(0,$w,$progs[3]);
		my $fz = fast1(0,$w,1,15,9);
		printf("w: %d iz: %d, fz: %d\n", $w, $iz, $fz);
		die("Wrong emulation!\n") if ($iz != $fz);
		$z{$z}++;
	}
}
	
die;

sub compile
{
	my ($pnr, $prog) = @_;
	
	my @inp = split(/\n/, $prog);
	my @ops = (sprintf(q(u64 func%d(u64 w, u64 z) {),$pnr),
			q(  u64 x, y;));

	foreach (@inp) {
		next if (/#/);
		my ($op, $a, $b) = split;
		next unless defined ($a);
		
		if ($op eq 'add') {
			push(@ops, sprintf("%s += %s;",$a, src($b)));
		}
		elsif ($op eq 'mul') {
			push(@ops, sprintf("%s *= %s;",$a, src($b)));
		}
		elsif ($op eq 'div') {
			push(@ops, sprintf("%s /= %s;", $a, src($b)));
		}
		elsif ($op eq 'mod') {
			push(@ops, sprintf("%s %%= %s;",$a, src($b)));
		}
		elsif ($op eq 'eql') {
			push(@ops, sprintf("%s = %s == %s;",$a, $a, src($b)));
		}
	}
	push(@ops,"}",'');
	return @ops;
}

my @c = compile((1) x 14);
printf("%s\n", join("\n",@c));
die;

my @inp8 = split(/\n/,q(inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y));

my @inp9 = split(/\n/,q(inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y));

my @inp10 = split(/\n/,q(inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y));

my @inp11 = split(/\n/,q(inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y));

my @inp12 = split(/\n/,
q(inp w
mul x 0
add x z
mod x 26
div z 26
add x 0
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y));

my @inp13 = split(/\n/,
q(inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y));

my @inp14 = split(/\n/,
q(inp w
mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y));
#print join("\n",compile(split(//,'12345678912345')));
#die;
my @inp5_7 = split(/\n/,q(inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y));

my @inp1_4 = split(/\n/,q(inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -6
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y));

my @o;

my %z1_4;
for (my $d = 0; $d <= 0; $d++) {
	for (my $w = 1111; $w <= 9999; $w++) {
		next if ($w =~ /0/);
		@inp = @inp1_4;
		my $z = test($d, split(//,$w));
		printf(STDERR "1-3 %d -> %d\n",$w,$z) if ($z < 800);
		if ($z < 800) {
			$z1_4{$z} = $w;
		}
	}
}
@o = ();
foreach (sort {$a <=> $b} keys %z1_4) { push(@o,sprintf("%d:%d",$_,$z1_4{$_})); }
printf("z1_4: w in %s\n",join(",",@o));

my %z5_7;
foreach (sort {$a <=> $b} keys %z1_4) {
	my $d = $_;
	for (my $w = 111; $w <= 999; $w++) {
		next if ($w =~ /0/);
		@inp = @inp5_7;
		my $z = test($d, split(//,$w));
		if ($z == 0) {
			$z5_7{$d} = $w;
		}
	}
}
@o = ();
foreach (sort {$a <=> $b} keys %z5_7) { push(@o,sprintf("%d:%d",$_,$z5_7{$_})); }
printf("z5_7: w in %s\n",join(",",@o));


die;

my %z14;

for (my $d = 0; $d < 100; $d++) {
	for (my $w = 1; $w <= 9; $w++) {
		next if ($w =~ /0/);
		@inp = @inp14;
		my $z = test($d, split(//,$w));
		if ($z == 0) {
			$z14{$d} = $w;
		}
	}
}
printf("z14: w in %s\n", join(",",sort {$a <=> $b} keys %z14));
	

my %z13;
for (my $d = 0; $d < 100; $d++) {
	for (my $w = 1; $w <= 9; $w++) {
		next if ($w =~ /0/);
		@inp = @inp13;
		my $z = test($d, split(//,$w));
		if (defined($z14{$z})) {
			$z13{$d} = $w;
		}
	}
}
printf("z13: z in %s\n", join(",",sort {$a <=> $b} keys %z13));

my %z12;
for (my $d = 0; $d < 2000; $d++) {
	for (my $w = 1; $w <= 9; $w++) {
		next if ($w =~ /0/);
		@inp = @inp12;
		my $z = test($d, split(//,$w));
		if (defined($z13{$z})) {
			$z12{$d} = $w;
		}
	}
}
printf("z12: z in %s\n", join(",",sort {$a <=> $b} keys %z12));

my %z11;
for (my $d = 0; $d < 18000; $d++) {
	for (my $w = 1; $w <= 9; $w++) {
		next if ($w =~ /0/);
		@inp = @inp11;
		my $z = test($d, split(//,$w));
		if (defined($z12{$z})) {
			$z11{$d} = $w;
		}
	}
}
printf("z11: z in %s\n", join(",",sort {$a <=> $b} keys %z11));

my %z10;
for (my $d = 0; $d < 700; $d++) {
	for (my $w = 1; $w <= 9; $w++) {
		next if ($w =~ /0/);
		@inp = @inp10;
		my $z = test($d, split(//,$w));
		if (defined($z11{$z})) {
			$z10{$d} = $w;
		}
	}
}
my %z9;
for (my $d = 0; $d < 26; $d++) {
	for (my $w = 1; $w <= 9; $w++) {
		next if ($w =~ /0/);
		@inp = @inp9;
		my $z = test($d, split(//,$w));
		if (defined($z10{$z})) {
			$z9{$d} = $w;
		}
	}
}
printf("z9: z in %s\n", join(",",sort {$a <=> $b} keys %z9));
my %z8;
for (my $d = 0; $d < 700; $d++) {
	for (my $w = 1; $w <= 9; $w++) {
		next if ($w =~ /0/);
		@inp = @inp8;
		my $z = test($d, split(//,$w));
		if (defined($z9{$z})) {
			$z8{$d} = $w;
		}
	}
}
printf("z8: z in %s\n", join(",",sort {$a <=> $b} keys %z8));


die;
		

for (my $d = 111111; $d <= 999999; $d++) {
	next if ($d =~ /0/);
	my ($x,$y,$z) = test(split(//,$d));
	printf(STDERR "%d %d %d %d\n", $d, $x, $y, $z) if ($z < 1000);
}

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %d\n", $part1);
printf("Part2: %1.0f\n", $part2);

#printf(STDERR "Move calls: %d, revisit: %d, skip: %d\n", $moves, $revisit,$skip);

sub compopt
{
	my @data = @_;

my ($w,$x,$y,$z)=(0,0,0,0);
$w=shift @data;
$x = 0;
$x += $z;
$x = $x % 26;
$x += 13;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 3;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
$x = 0;
$x += $z;
$x = $x % 26;
$x += 11;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 12;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
$x = 0;
$x += $z;
$x = $x % 26;
$x += 15;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 9;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
$x = 0;
$x += $z;
$x = $x % 26;
$z = int($z / 26);
$x += -6;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 12;
$y *= $x;
$z += $y;
#printf("z:%d\n",$z) if ($x == 0);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$x += 15;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 2;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$z = int($z / 26);
$x += -8;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 1;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$z = int($z / 26);
$x += -4;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 1;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$x += 15;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 13;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$x += 10;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 1;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$x += 11;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 6;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$z = int($z / 26);
$x += -11;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 2;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$z = int($z / 26);
$x += 0;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 11;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$z = int($z / 26);
$x += -8;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 10;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
$w=shift @data;
return $z unless(defined($w));
$x = 0;
$x += $z;
$x = $x % 26;
$z = int($z / 26);
$x += -7;
$x = $x == $w;
$x = $x == 0;
$y = 0;
$y += 25;
$y *= $x;
$y += 1;
$z *= $y;
$y = 0;
$y += $w;
$y += 3;
$y *= $x;
$z += $y;
printf("z:%d\n",$z) if ($DEBUG);
return $z;
}
