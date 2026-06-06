#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use List::PriorityQueue;


my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my ($W,$H) = (7,7);

my @b = ('#' x ($W+2))."\n";
my $b = '#'.('.' x $W)."#\n";
for (my $y = 1; $y <= $H; $y++) {
	push(@b,$b);
}
push(@b,$b[0]);

my $corr = 0;
while (<>) {
	chomp;
	my ($x,$y) = split(/,/);
	substr(@b[$y+1],$x+1,1) = "#";
	$corr++;
	last if ($corr == 12);
}

printf("%s\n",join("",@b));

my @dx = (1,0,-1,0);
my @dy = (0,1,0,-1);

sub bfs
{
	my $pq = new List::PriorityQueue;
	$pq->insert("1,1,0", 0);
	my %seen = ();
	while (my $k = $pq->pop()) {
		my ($x,$y,$len) = split(/,/,$k);

		my $xy = "$x,$y";
		next if (substr($b[$y],$x,1) eq '#');
		next if (defined($seen{$xy}));
		return $len if ($x == $W && $y == $H);

		$seen{$xy}++;
		printf("%s\n",$k);
		$len++;
		$pq->insert(join(",",$x+1,$y,$len),$len);
		$pq->insert(join(",",$x-1,$y,$len),$len);
		$pq->insert(join(",",$x,$y+1,$len),$len);
		$pq->insert(join(",",$x,$y-1,$len),$len);
	}
	return 0;
}

printf("%d\n",bfs());

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
