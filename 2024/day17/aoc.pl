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
	@reg = ($rega,0,0);
	my @out = ();
	my $ip = 0;
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
			#last;
		}
		elsif ($inst == 6) { # bdv
			$reg[1] = $reg[0] >> combo($oper);
		}
		elsif ($inst == 7) { # cdv
			$reg[2] = $reg[0] >> combo($oper);
		}
	}
	return @out;
}

my @out;

@out = simulate($reg[0]);
$part1 = join(",",@out);

my @ut = ();

my @lookup = ();
for (my $rega = 0; $rega < 1024; $rega++) {
	@out = simulate($rega);
	$lookup[$rega] = $out[0];
	#printf("%4d -> %s\n", $rega, join(",",@out));
	push(@{$ut[$out[0]]}, $rega);
}
#printf("%s\n", join(" ",@lookup));

sub try
{
	my ($prefix, $bits, @prog) = @_;
	my $dig = shift @prog;
	my @alt = @{$ut[$dig]};
	foreach (@alt) {
		
	


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
