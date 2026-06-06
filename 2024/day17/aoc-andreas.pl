#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

while (<>) {
#	chomp;
	push(@lines, $_);
}
my $oneline = join("",@lines);
my ($regs,$prog) = split(/\n\n/,$oneline);

my @reg = ();
foreach (split(/\n/,$regs)) {
	if (/Register (\S): (\d+)\s*$/) {
		$reg[ord($1)-ord('A')] = $2;
	}
	else { die("Bad input $_"); }
}

my @sreg = @reg;

if ($prog =~ /Program: ([\d\,]*\d)\s*/) {
	$prog = $1;
}
else {die("prog error"); }
my @prog = split(/,/,$prog);

sub combo
{
	my ($lit) = @_;
	return $lit if ($lit <= 3);
	return $reg[$lit-4] if ($lit <= 6);
	die("combo(7) is illegal");
}

#part1
my $ip = 0;

sub debug
{
	my ($ip,$inst,$oper,@reg) = @_;
	my @iname = ("adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv");

	printf("A: %o, B: %o, C: %o, IP: %d, inst: %d (%s), oper: %d\n",
		@reg,$ip,$inst,$iname[$inst], $oper);
}

sub simulate
{
	my ($rega) = @_;
	my @reg = ($rega,0,0);
	my @out = ();
	my $ip = 0;
	while (1) {
		last if ($ip+1 >= scalar(@prog));
		my ($inst,$oper) = ($prog[$ip],$prog[$ip+1]);
		debug($ip,$inst,$oper,@reg);

		$ip += 2;
		if ($inst == 0) { # adv
			$reg[0] >>= combo($oper);
		}
		elsif ($inst == 1) { # bxl
			$reg[1] ^= $oper;
		}
		elsif ($inst == 2) { # bst
			$reg[1] = combo($oper) & 7;
		}
		elsif ($inst == 3) { # jnz
			if ($reg[0]) {
				#$ip = $oper;
			}
		}
		elsif ($inst == 4) { # bxc
			$reg[1] ^= $reg[2];
		}
		elsif ($inst == 5) { # out
			push(@out, combo($oper) & 7);
			printf("%o\n",$out[-1]);
		}
		elsif ($inst == 6) { # bdv
			$reg[1] = $reg[0] >> combo($oper);
		}
		elsif ($inst == 7) { # cdv
			$reg[2] = $reg[0] >> combo($oper);
		}
	}
	return ($out[0],$reg[0]);
}

sub simandreas
{
	my ($rega) =  @_;
	my ($a,$b,$c) = ($rega,0,0);
	$b = $a & 7;
	$b ^= 5;
	$c = $a >> $b;
	$b ^= $c;
	my $out = $b & 7;
	$a >>= 3;
	return ($out, $a);
}

my @lookup = ();
my @tail = ();
my @out;
for (my $rega = 0; $rega < 1024; $rega++) {
	my ($o, $a) = simandreas($rega);
	$lookup[$rega] = $o;
	$tail[$rega] = $a;
	#printf("%4o: %d %o\n",$rega,$o,$a); # if ($rega >= 1000);
}
#printf("%s\n", join(" ",@lookup));

my $lowest = '7' x 16;

my @inp = (0 x 16);
sub search
{
	my ($ip, $seven, $mask) = @_;
	my $p = $prog[$ip];
	for (my $i = 0; $i < scalar(@lookup); $i++) {
		next unless ((($i & $mask) == $seven) && ($p == $lookup[$i]));
		$inp[$ip] = $i & 7;
		if ($ip > 3) {
			printf("%s\n", join("", reverse @inp));
		}
		if ($ip == 15) {
			my $sol = join("", reverse @inp);
			
			if ($sol lt $lowest && $i <= 7) {
				$lowest = $sol;
				printf("New min %s\n", $sol);
			}
			
			#die;
		}
		else {
			search($ip+1, $i >> 3, 127);
		}
	}
}

search(0,0,0);

exit(1);

$part1 = join(",",@out);

#part2
my @as = (0);
#for (my $a = 0x2646abdf4; 1; $a+=1) {
my  @diff = ((0) x 16);
for (my $a = oct("1035517432536764"); 1; $a+=1024) {
	@reg = @sreg;
	@out = ();
	$reg[0] = $a;
	$ip = 0;
	#printf("A:%7d "); 
	#debug($ip,$inst,$oper,@reg);
	while (1) {
		last if ($ip+1 >= scalar(@prog));
		my ($inst,$oper) = ($prog[$ip],$prog[$ip+1]);
		#debug($ip,$inst,$oper,@reg);

		$ip += 2;
		if ($inst == 0) { # adv
			$reg[0] >>= combo($oper);
		}
		elsif ($inst == 1) { # bxl
			$reg[1] ^= $oper;
		}
		elsif ($inst == 2) { # bst
			$reg[1] = combo($oper) & 7;
		}
		elsif ($inst == 3) { # jnz
			if ($reg[0]) {
				$ip = $oper;
			}
		}
		elsif ($inst == 4) { # bxc
			$reg[1] ^= $reg[2];
		}
		elsif ($inst == 5) { # out
			push(@out, combo($oper) & 7);
			last if ($out[-1] != $prog[scalar(@out)-1]);
			my $len = scalar(@out);
			if ($len >= 11) {
				printf("A: %s, len = %d, diff= %o\n",$a,
					scalar(@out), $a-$diff[$len]);
				push(@as,$a);
				$diff[$len] = $a;
			}
		}
		elsif ($inst == 6) { # bdv
			$reg[1] = $reg[0] >> combo($oper);
		}
		elsif ($inst == 7) { # cdv
			$reg[2] = $reg[0] >> combo($oper);
		}
	}
	if (scalar(@out) == scalar(@prog)) {
		$part2 = $a;
		last;
	}
}


my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
