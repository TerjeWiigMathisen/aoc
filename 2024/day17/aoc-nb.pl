#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @prog = (2,4,1,2,7,5,1,3,4,4,5,5,0,3,3,0);
my $target = join("",@prog);
push(@prog,0,0,0);

sub simulate
{
	my ($rega,$b,$c) = @_;
	my $len = 0;
	my $a = $rega;
	my $out = '';
	while ($a) {
		$b = ($a & 7) ^ 2; # Bottom 3 bits
		$c = ($a >> $b) & 7; # Mask is applied later
		$b = ($b ^ 3) ^ $c;
		if ($b != $prog[$len]) {
			return 0 unless ($len);
			return join("",(@prog)[0..($len-1)]);
		}
		$len++;
		$a >>= 3;
		last if ($len > 16);
		#printf("%d: %o\n", $len, $rega) if ($len >= 8);
	}
	return join("",(@prog)[0..($len-1)],$a);
}

sub simulate3
{
	my ($rega,$b,$c,$slen) = @_;
	
	my $len = 0;
	my $a = $rega;
	my $out = '';
	while ($a) {
		$b = ($a & 7) ^ 2; # Bottom 3 bits
		$c = ($a >> $b) & 7; # Mask is applied later
		$b = ($b ^ 3) ^ $c;
		if ($b != $prog[$len]) {
			return 0 unless ($len);
			return join("",(@prog)[0..($len-1)]);
		}
		$len++;
		$a >>= 3;
		#printf("%d: %o\n", $len, $rega) if ($len >= 8);
	}
	return join("",(@prog)[0..($len-1)]);
}

sub oct2dec
{
	my ($dec) = @_;
	my $r = 0;
	my $n = $dec;
	while (length($n) > 9) {
		$r = ($r<<3) + substr($n,0,1);
		$n = substr($n,1);
	}
	$r = ($r<<27) + oct($n);
	die("conversion error! $dec -> $r") unless $dec == sprintf("%o",$r);
	return $r;
}

sub search
{
	my (@suffixes) = (0);
	my $solution = 1e38;
	#my $octaldigits = 0;
	my $bits = 0;
	my $slen = 3;
	while (scalar(@suffixes)) {
		my %found = ();
		printf("searching through '%s' for $slen digits\n", join(" ",@suffixes));
		foreach (@suffixes) {
			my $suffix = oct2dec($_);
			
			#$suffix = 0 unless ($suffix);
			for (my $prefix = 64; $prefix < 65536; $prefix++) {
				my $rega = ($prefix << $bits) | $suffix;
				my $l = simulate3($rega,0,0,$slen);
				if (length($l) == 16) {
					if ($rega < $solution) {
						printf("Possible solution: $rega (%o) %s\r", $rega, $l);
						$solution = $rega;
					}
				}
				elsif (length($l) == $slen) {
					my $o = sprintf("%o",$rega);
					my $suf = substr($o,-$slen);
					if (!defined($found{$suf})) {
						printf("New suffix found: $suf for rega = $rega (%o)\n",$rega);
					}
					$found{$suf}++;
				}
			}
			#$octaldigits += 3;
		}
		return $solution if $solution < 1e38;
		@suffixes = sort keys %found;
		$bits += 9;
		$slen += 3;
		$slen = 16 if ($slen > 16);
	}
	return $solution;
}

my @sol = search(0);
$part2 = $sol[0];

#part1

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("My target: 37222273957364 (%o)\n",37222273957364);
printf("Used %5.3fms\n",$used*1000);
