#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use List::PriorityQueue;


my $part1 = 0;
my $part2 = 0;

my $start = time;

my @b = ('','');
my $GUARD = '##';
my ($sx,$sy,$ex,$ey);
while (<>) {
	chomp;
	push(@b,$GUARD.$_.$GUARD);
	if (index($_,'S')>=0) {
		($sx,$sy) = (index($_,'S')+2,scalar(@b)-1);
	}
	if (index($_,"E")>=0) {
		($ex,$ey) = (index($_,"E")+2,scalar(@b)-1);
	}
	chomp;
}
$b[0] = '#' x length($b[2]);
$b[1] = $b[0];
push(@b,$b[0],$b[0]);

sub bfs
{
	my ($sx,$sy,$ex,$ey,@b) = @_;
	#printf("Searching from ($sx,$sy) to ($ex,$ey) on map\n%s\n", join("\n",@b));
	my $pq = new List::PriorityQueue;
	$pq->insert("$sx,$sy,1", 0);
	my %seen = ();
	while (my $k = $pq->pop()) {
		my ($x,$y,$len) = split(/,/,$k);
		die("Bad key $k") unless (defined($len));

		my $xy = "$x,$y";
		next if (substr($b[$y],$x,1) eq '#');
		next if (defined($seen{$xy}));
		$seen{$xy} = $len;
		
		if ($x == $ex && $y == $ey) {
			return ($len,%seen);
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

my @dx = (1,0,-1,0);
my @dy = (0,1,0,-1);

my ($forward,%se) = bfs($sx,$sy,$ex,$ey,@b);
my ($backward,%es) = bfs($ex,$ey,$sx,$sy,@b);
die("bad search") unless (defined($forward));
printf("Shortest path f & b: $forward,$backward\n");
my @f = sort keys %se;
my %save = ();
foreach (keys %se) {
	my $f = $se{$_};
	my ($x,$y) = split(/,/);
	for (my $d = 0; $d<4; $d++) {
		my ($cx,$cy) = ($x+$dx[$d],$y+$dy[$d]);
		next if (defined($se{"$cx,$cy"}));
		for (my $i = 2; $i <= 3; $i++) {
			my ($cx,$cy) = ($x+$dx[$d]*$i,$y+$dy[$d]*$i);
			my $cheatstart = "$x,$y,$cx,$cy";
			my $b = $es{"$cx,$cy"};
			if (defined($b)) {
				my $plen = $f+$b+$i-1;
				my $save = $forward-$plen;
				if ($save >= 100) {
					$save{$cheatstart} = $save; # unless (defined($save{$cheatstart}) && $save{$cheatstart} >= $save);
					#if ($save >= 100) {
						#printf("Found cheat of $save ps at ($x,$y) to ($cx,$cy) dir $d len $i)\n");
						#printf("%s\n",join("\n",@b));
					#}
				}
				last;
			}
		}
	}
}
my %scnt = ();
foreach (keys %save) {
	my $s = $save{$_};
	$scnt{$s}++;
}
for (my $s = 100; $s < $forward; $s++) {
	if (defined($scnt{$s})) {
		#printf("%s %s\n",$s, $scnt{$s});
		$part1 += $scnt{$s};
	}
}

#part2

%save = ();
foreach (keys %se) {
	my $f = $se{$_};
	my ($x,$y) = split(/,/);
	for (my $cy = $y-20; $cy <= $y+20; $cy++) {
		my $span = 20 - (abs($y-$cy));
		for (my $cx = $x-$span; $cx <= $x+$span; $cx++) {
			my $b = $es{"$cx,$cy"};
			my $cheat = "$x,$y,$cx,$cy";
			next unless (defined($b));
			#printf("Trying $cheat\n");
			my $i = abs($x-$cx)+abs($y-$cy); 
			my $plen = $f+$b+$i-1;
			my $save = $forward-$plen;
			if ($save >= 50) {
				$save{$cheat} = $save; # unless (defined($save{$cheatstart}) && $save{$cheatstart} >= $save);
				#if ($save == 50) {
					#printf("Found cheat of $save ps at ($x,$y) to ($cx,$cy) len $i)\n");
					#printf("%s\n",join("\n",@b));
				#}
			}
		}
	}
}
%scnt = ();
foreach (keys %save) {
	my $s = $save{$_};
	$scnt{$s}++;
}
for (my $s = 100; $s < $forward; $s++) {
	if (defined($scnt{$s})) {
		#printf("%s %s\n",$s, $scnt{$s});
		$part2 += $scnt{$s};
	}
}


my $used = time - $start;
printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
