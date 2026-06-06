#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;

my $part1 = 0;
my $part2 = 0;

# sub invalid
# {
	# my ($n) = @_;
	# my $l2 = length($n) >> 1;
	# return (substr($n,0,$l2) eq substr($n,$l2))? $n : 0;
# }

# sub invalid2
# {
	# my ($n) = @_;
	# for (my $div = 2; $div <= length($n); $div++) {
		# next if (length($n) % $div);
		# my $l = length($n) / $div;
		# my $pat = substr($n,0,$l);
		# my $ok = 1;
		# for (my $i = 1; $i < $div; $i++) {
			# if (substr($n,$l*$i,$l) ne $pat) {
				# $ok = 0;
				# last;
			# }
		# }
		# return $n if ($ok);
	# }
	# return 0;
# }

sub inv
{
	my ($f, $l) = @_;
	my $l2 = length($f) >> 1;
	my $pre = substr($f,0,$l2);
	while (1) {
		my $inv = $pre x 2;
		return 0 if ($inv > $l);
		return $inv if ($f <= $inv);
		$pre++;
	}
	return 0;
}

sub invdiv
{
	my ($f, $l, $div) = @_;
	my $l2 = int(length($f) / $div);
	my $pre = substr($f,0,$l2);
	$pre = 1 unless ($pre);
	my $res = 0;
	my %seen = ();
	while (1) {
		my $inv = $pre x $div;
		last if ($inv > $l);
		if ($f <= $inv) {
			$seen{$inv}++;
			# printf("$f $l $inv\n");
		}
		$pre++;
	}
	return %seen;
}

sub inv1
{
	my ($f, $l) = @_;
	my $res = 0;
	my %seen = invdiv($f,$l,2);
	foreach (keys %seen) {
		$res += $_;
	}
	return $res;
}

sub inv2
{
	my ($f, $l) = @_;
	my $res = 0;
	my %seen = ();
	for (my $div = 2; $div <= length($l); $div++) {
		my %s = invdiv($f,$l,$div);
		foreach (keys %s) {
			$seen{$_}++;
		}
	}
	foreach (keys %seen) {
		$res += $_;
	}
	return $res;
}

while (<>) {
	chomp;
	my @ranges = split(/,/);
	foreach (@ranges) {
		my ($first, $last) = split(/-/);	
		$part1 += inv1($first,$last);
		$part2 += inv2($first,$last);
	}
}

my $used = time-$t0;

printf("Test: 1227775554 4174379265\n");
printf("Input: 38437576669 49046150754\n");
printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
