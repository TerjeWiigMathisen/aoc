#!perl -w

use strict;
use Time::HiRes qw (time);

my $start = time;

my @inp = (<>);
chomp(@inp);

my $DEBUG = 0;

for (my $iter = 0; $iter < 1; $iter++) {

my @disp = ();
my @dec = ();

foreach (@inp) {
	my ($mix, $out) = split(/\s*\|\s*/);
	push(@disp, $mix);
	push(@dec,$out);
}

my @segments = ("abcefg","cf","acdeg","acdfg","bcdf","abdfg","abdefg","acf","abcdefg","abcdfg");

my @lengths = ();
my @charin = ();
foreach (@segments) {
	my $l = length($_);
	$lengths[$l]++;
	foreach (split//) {
		${$charin[$l]}->{$_}++;
	}
}

my $u = 0;
my @d = ();
foreach (@dec) {
	my $d = $_;
	foreach (split/ /, $d) {
		if ($lengths[length($_)] == 1) {
			$u++;
		}
	}
	printf(STDERR "aoc8a: %s %d\n", $d, $u) if ($DEBUG);
}

printf("aoc8a: %d\n", $u);

my $sum = 0;
for (my $i = 0; $i < scalar(@dec); $i++) {
	my $res = decode($disp[$i], $dec[$i]);
	printf(STDERR "%s %d\n", $disp[$i], $res) if ($DEBUG);
	$sum += $res;
}

printf("aoc8b: %d\n", $sum);
}

my $stop = time;
printf(STDERR "Total time = %f\n", $stop-$start);

sub alfa
{
	my @i = @_;
	my @o = ();
	foreach (@i) {
		my @s = sort split(//);
		push(@o, join("", @s));
	}
	return @o;
}

sub decode
{
	my ($inp, $out) = @_;
	my @pat = alfa(split(/ /, $inp));
	my @out = alfa(split(/ /, $out));
	
	my ($d0, $d1, $d2, $d3, $d4, $d5, $d6, $d7, $d8, $d9); 
	my %s2d = ();
	my @len5 = ();
	my @len6 = ();
	
	foreach (@pat) {
		$d1 = $_ if (length($_) == 2);
		$d7 = $_ if (length($_) == 3);
		$d4 = $_ if (length($_) == 4);
		push(@len5, $_) if (length($_) == 5);
		push(@len6, $_) if (length($_) == 6);
		$d8 = $_ if (length($_) == 7);
	}
	printf(STDERR "1 -> %s\n",$d1) if ($DEBUG);
	printf(STDERR "4 -> %s\n",$d4) if ($DEBUG);
	printf(STDERR "7 -> %s\n",$d7) if ($DEBUG);
	printf(STDERR "8 -> %s\n",$d8) if ($DEBUG);
	
	my %map = ();
	my %seg = ();
	# Start with len = 2 &3 to find A and (cf)
	my %cf = ();
	foreach (split(//, $d7)) {
		if (index($d1, $_) < 0) {
			$map{$_} = 'A';
			$seg{'a'} = $_;
			printf(STDERR "A = %s\n", $_) if ($DEBUG);
		}
		else {
			$cf{$_}++;
		}
	}
	printf(STDERR "CF = %s\n", join('', keys %cf)) if ($DEBUG);
	
	# len 4 == 4 locates (bd)
	my %bd = ();
	foreach (split(//, $d4)) {
		if (index($d1, $_) < 0) {
			$bd{$_}++;
		}
	}
	printf(STDERR "BD = %s\n", join('', keys %bd)) if ($DEBUG);
	
	# Locate '6' -> C segment:
	foreach (@len6) { # 0,6,9
		my $d = $_;
		my $cf = 0;
		my $bd = 0;
		foreach (split(//)) {
			$cf += defined($cf{$_});
			$bd += defined($bd{$_});
		}
		if ($cf < 2) {
			die("d6 already defined!") if (defined($d6));
			$d6 = $d;
			printf(STDERR "6 -> %s\n",$d6) if ($DEBUG);
			# Use this to find 'C
			for ($a = 'a'; $a le 'g'; $a++) {
				next if (index($d, $a)>= 0);
				$map{$a} = 'C';
				$seg{'c'} = $a;
			}
		}
		elsif ($bd < 2) {
			die("d0 already defined!") if (defined($d0));
			$d0 = $d;
			printf(STDERR "0 -> %s\n",$d0) if ($DEBUG);
		}
		else {
			die("d9 already defined!") if (defined($d9));
			$d9 = $d;
			printf(STDERR "9 -> %s\n",$d9) if ($DEBUG);
		}
	}

	# len = 5 -> 2, 3, 5
	foreach (@len5) { # 2,3,5
		my $n = $_;
		my $cf = 0;
		my $D;
		foreach (split(//)) {
			$cf += defined($cf{$_});
			$D = $_ if (defined($bd{$_}));
		}
		if ($cf < 2) { # Cannot be '3'
			# Do we have 'C' segment?, if so 2
			if (index($n,$seg{'c'}) >= 0) { # This is '2'
				die("d2 already defined!") if (defined($d2));

				$d2 = $n;
				printf(STDERR "2 -> %s\n",$d2) if ($DEBUG);
				
				$map{$D} = 'D';
				$seg{'d'} = $D;
			}
			else {
				die("d5 already defined!") if (defined($d5));
				$d5 = $n;
				printf(STDERR "5 -> %s\n",$d5) if ($DEBUG);
			}
		}
		else {
			die("d3 already defined!") if (defined($d3));
			$d3 = $n;
			printf(STDERR "3 -> %s\n",$d3) if ($DEBUG);
		}
	}
	# Now we can decode and accumulate!
	$s2d{$d0} = '0';
	$s2d{$d1} = '1';
	$s2d{$d2} = '2';
	$s2d{$d3} = '3';
	$s2d{$d4} = '4';
	$s2d{$d5} = '5';
	$s2d{$d6} = '6';
	$s2d{$d7} = '7';
	$s2d{$d8} = '8';
	$s2d{$d9} = '9';

	my $disp = '';
	foreach (@out) {
		$disp .= $s2d{$_};
	}
	return $disp;
}

sub bin
{
	my (@p) = @_;
	my $b = 0;
	foreach (split(//,$p)) {
		$b += 1 << (ord($_)-ord('a'));
	}
	return ((length($p)<<8)+$b);

sub decodebin
{
	my ($inp, $out) = @_;
	my @pat = alfa(split(/ /, $inp));
	my @out = alfa(split(/ /, $out));
