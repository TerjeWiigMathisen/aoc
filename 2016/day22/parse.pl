#!perl -w

use strict;
use warnings;

my %size = ();
my %used = ();
my %avail = ();

my ($mx,$my) = (0,0);

while (<>) {
	chomp;
	if (/node\-x(\d+)\-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T/) {
		my ($x,$y,$s,$u,$a) = ($1,$2,$3,$4,$5);
		my $idx = "$x;$y";
		$size{$idx} = $s;
		$used{$idx} = $u;
		$avail{$idx} = $a;
		$mx = $x if ($x > $mx);
		$my = $y if ($y > $my);
	}
}
printf("Array size = ($mx x $my)\n");

sub dmp
{
	my ($fn, %href) = @_;
	my $fh;
	open($fh,'>',$fn) || die;
	for (my $y = 0; $y <= $my; $y++) {
		my @line = ();
		for (my $x = 0; $x < $mx; $x++) {
			push(@line,$href{"$x;$y"});
		}
		printf($fh "%s\n",join("\t",@line));
	}
	close($fh);
}

dmp("size.csv",%size);
dmp("used.csv",%used);
dmp("avail.csv",%avail);
