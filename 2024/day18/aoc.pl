#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use List::PriorityQueue;


my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my ($W,$H) = (71,71);

my @b = ('#' x ($W+2))."\n";
my $b = '#'.('.' x $W)."#\n";
for (my $y = 1; $y <= $H; $y++) {
	push(@b,$b);
}
push(@b,$b[0]);

my @fall = ();
while (<>) {
	chomp;
	push(@fall,$_);
}

for (my $corr = 0; $corr < 1024; $corr++) {
	$_ = $fall[$corr];
	my ($x,$y) = split(/,/);
	substr($b[$y+1],$x+1,1) = "#";
}

#printf("%s\n",join("",@b));

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
		$seen{$xy} = $len;
		return $len if ($x == $W && $y == $H);
		
		#printf("%s\n",$k);
		$len++;
		$pq->insert(join(",",$x+1,$y,$len),$len);
		$pq->insert(join(",",$x-1,$y,$len),$len);
		$pq->insert(join(",",$x,$y+1,$len),$len);
		$pq->insert(join(",",$x,$y-1,$len),$len);
	}
	return 0;
}

sub bfs_path
{
	my $pq = new List::PriorityQueue;
	$pq->insert("1,1,0", 0);
	my %seen = ();
	while (my $k = $pq->pop()) {
		my ($x,$y,$len) = split(/,/,$k);

		my $xy = "$x,$y";
		next if (substr($b[$y],$x,1) eq '#');
		next if (defined($seen{$xy}));
		$seen{$xy} = $len;
		if ($x == $W && $y == $H) {
			my %path = ();
			my ($x,$y) = ($W,$H);
			$path{"$x,$y"}++;
			while ($len) {
				$len--;
				for (my $d =0; $d < 4; $d++) {
					my ($px,$py) = ($x+$dx[$d],$y+$dy[$d]);
					my $s = "$px,$py";
					if (defined($seen{$s}) && $seen{$s} == $len) {
						($x,$y) = ($px,$py);
						$path{"$x,$y"}++;
						last;
					}
				}
			}
			return %path;
		}
		
		#printf("%s\n",$k);
		$len++;
		$pq->insert(join(",",$x+1,$y,$len),$len);
		$pq->insert(join(",",$x-1,$y,$len),$len);
		$pq->insert(join(",",$x,$y+1,$len),$len);
		$pq->insert(join(",",$x,$y-1,$len),$len);
	}
	return ();
}

my %path = bfs_path();
$part1 = scalar(keys %path);
#printf("%d\n",$part1);
my @save = @b;

my $lo = 1024; 
my $hi = scalar(@fall)-1;
#printf("Searching from $lo to $hi\n");
my $fill = 1024;
my $corr;
while ($lo+1 < $hi) {
	my $mid = ($lo+$hi) >> 1;
	@b = @save;
	for ($corr = 1024; $corr < $mid; $corr++) {
		$_ = $fall[$corr];
		my ($x,$y) = split(/,/);
		substr($b[$y+1],$x+1,1) = "#";
	}

	if (bfs()) {
		#printf("$mid open\n");
		$lo = $mid;
	}
	else {
		#printf("$mid blocked\n");
		$hi = $mid;
	}
}
$part2 = $fall[$corr];

my $used = time - $start;

#printf("Closed path after %s\n",$fall[$corr]);

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
