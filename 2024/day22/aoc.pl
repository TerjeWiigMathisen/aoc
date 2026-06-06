#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use List::PriorityQueue;


my $part1 = 0;
my $part2 = 0;

my $start = time;

sub iter
{
	my ($sec) = @_;
	$sec = (($sec << 6) ^ $sec) & 16777215;
	$sec = (($sec >> 5) ^ $sec) & 16777215;
	$sec = (($sec << 11) ^ $sec) & 16777215;
	return $sec;
}

my %sum = ();

while (<>) {
	chomp;
	my $secret = $_;
#	my $init = $_;

	my $s = $secret % 10;
	my $prev = $s;
	my @list = ($s);
	my %seen = ();
	for (my $i = 1; $i <= 2000; $i++) {
		$secret = iter($secret);
		$s = $secret % 10;
		push(@list,$s - $prev);
		$prev = $s;

		next if ($i < 3);
		
		my $price = $s;
		my $seq = join(',',@list);
		shift @list;
		next if (defined($seen{$seq}));

		$seen{$seq} = $price; 
		$sum{$seq} += $price;
	}
	$part1 += $secret;
}
# Pick the max:
my @seqmax = sort {$sum{$b} <=> $sum{$a}} keys %sum;
$part2 = $sum{$seqmax[0]};

my $used = time - $start;
printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

