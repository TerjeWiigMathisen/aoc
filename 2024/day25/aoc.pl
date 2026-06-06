#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;

my $start = time;

my $one = '';
while (<>) { $one .= $_; }

my @locks = ();
my @keys = ();

foreach (split(/\n\n/, $one)) { # Split into input blocks
	my @rows = split(/\n/);
	my @h = (-1,-1,-1,-1,-1); # One extra '#' at the border
	foreach (@rows) {
		my $col = 0;
		foreach (split(//)) {
			$h[$col] += $_ eq '#';
			$col++;
		}
	}
	my $bits = 0;
	foreach (@h) { $bits = ($bits << 4) | $_; }
	if ($rows[0] eq '#####') { #Lock
		push(@locks, $bits);
	}
	else {
		push(@keys, $bits + 0x22222);
	}
}

@locks = sort @locks; # Sorting saves a millisecond!
@keys =  sort @keys;

foreach (@locks) {
	my $l = $_;
	foreach (@keys) {
		my $sum = $l + $_;
		last if ($sum & 0x80000);
		unless ($sum & 0x88888) {
			$part1++;
		}
	}
}

my $used = time - $start;

printf("%s\n", $part1);
printf("Used %5.3fms\n",$used*1000);
