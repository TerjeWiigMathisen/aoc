#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);

my $moves = shift @inp; shift @inp;
$moves =~ tr/LR/01/;
my $m = 0;
my @moves = split(//,$moves);

my %lr = ();
foreach (@inp) {
	die $_ unless (/^(\S+)\s*=\s*\((\S+), (\S+)\)/);
	my @lr = ($2,$3);
	$lr{$1} = \@lr;
}
my $pos = 'AAA';
for ($part1 = 0; $pos ne 'ZZZ'; $part1++) {
	my $move = $moves[$m++];
	if ($m >= scalar(@moves)) { $m=0; }
	my @lr = @{$lr{$pos}};
	$pos = $lr[$move];
#	printf("%d %d %s\n", $part1, $m, $pos);
}

my @pos = ();
foreach (keys %lr) {
	if (/A$/) {
		push(@pos,$_);
	}
}

my %loops = ();

foreach (@pos) {
	$pos = $_;
	my $start = $_;
	$m = 0;
	my %seen = (); #($_ => 0);
	for ($part2 = 0; 1; $part2++) {
		my $move = $moves[$m++];
		if ($m >= scalar(@moves)) { $m=0; }

		my @lr = @{$lr{$pos}};
		$pos = $lr[$move];
		if (substr($pos,-1) eq 'Z') {
			if (defined($seen{$pos})) {
				my $first = $seen{$pos};
				my $l = $part2-$first;
#				printf("%s: Loop of length %d found\n", $start, $l);
				if ($l % scalar(@moves) == 0) {
					$loops{$start} = $first."\t".$l;
					last;
				}
			}
			else {
				$seen{$pos} = $part2;
#				printf("%s: %s in move %d\n", $start, $pos, $part2);
			}
		}

#		printf("%s: %d %d %s\n", $start, $part2, $m, $pos);
	}
#	printf("%s: %d %d %s\n", $start, $part2, $m, $pos);
#	die;
	
	# for ( ; $m || $pos ne $start; $part2++) {
		# my $move = $moves[$m++];
		# if ($m >= scalar(@moves)) { $m=0; }

		# my @lr = @{$lr{$pos}};
		# $pos = $lr[$move];

		# printf("%s: %d %d %s\n", $start, $part2, $m, $pos);
	# }
	# printf("%s: %d %d %s\n", $start, $part2, $m, $pos);
}

my @first = ();
my @loops = ();
foreach (keys %loops) {
	my ($f, $l) = split(/\t/,$loops{$_});
	push(@first, $f);
	push(@loops, $l);
}

use bigint;

sub factors
{
	my ($n) = @_;
	my @f = ();
	for (my $i = 3; $i*$i <= $n; $i += 2) {
		while ($n % $i == 0) {
			push(@f, $i);
			$n /= $i;
		}
	}
	push(@f,$n) if ($n > 1);
	return @f;
}

my %all_factors = ();
my $prod = 1;
foreach (@loops) {
	my @f = factors($_);
	foreach (@f) {
		$all_factors{$_}++;
	}
#	printf("%d -> (%s)\n", $_, join(",",@f));
}
printf("Factors = %s\n", join(",", sort {$a<=>$b} (keys %all_factors)));
$prod = 1;
foreach (keys %all_factors) {
	$prod *= $_;
}
$part2 = $prod;

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
