#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my $one = '';

while (<>) {
	$one .= $_;
}

my @lockkeys = split(/\n\n/, $one);
my @locks = ();
my @keys = ();

foreach (@lockkeys) {
	my @rows = split(/\n/);
	my @h = (-1,-1,-1,-1,-1);
	foreach (@rows) {
		my $col = 0;
		foreach (split(//)) {
			$h[$col] += $_ eq '#';
			$col++;
		}
	}
	if (/^#####/) { #Lock
		push(@locks, join(",",@h));
		#printf("Lock %s\n", $locks[-1]);
	}
	elsif (/^...../) {
		push(@keys, join(",",@h));
		#printf("key  %s\n", $keys[-1]);
	}
}

#printf("We have %d locks and %d keys\n", scalar(@locks), scalar(@keys));

sub fit
{
	my ($lock, $key) = @_;
	for (my $pin = 0; $pin < 5; $pin++) {
		if ($lock->[$pin] + $key->[$pin] > 5) {
			return 0;
		}
	}
	return 1;
}

foreach (@locks) {
	my @l = split(/,/);
	foreach (@keys) {
		my @k = split(/,/);
		if (fit(\@l,\@k)) {
			$part1++;
		}
	}
}

my $used = time - $start;

printf("%s\n", $part1);
printf("Used %5.3fms\n",$used*1000);
