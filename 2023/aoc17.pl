#!perl -w
use strict;
use Time::HiRes qw(time);
use List::PriorityQueue;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

my @tries = ();
my @path = ();

sub solve
{
	my ($minl, $maxl) = @_;

	my %seen = ();
	my ($tx,$ty) = (length($inp[0])-1,scalar(@inp)-1);
	my $prio = new List::PriorityQueue;
	# Start by entering (0,0) from left and above:
	$prio->insert(sprintf("%d,%d,%d,%s",0,0,0,0),0);
	$prio->insert(sprintf("%d,%d,%d,%s",0,0,0,1),0);
	my @dx = (1,0,-1,0);
	my @dy = (0,1,0,-1);
	while (my $m = $prio->pop()) {
		my ($loss,$x,$y,$d) = split(/,/,$m);
		my $k = join(",",$x,$y,$d);
		if (defined($seen{$k})) {
			next;
		}
		$seen{$k} = $loss;
		
		if ($x == $tx && $y == $ty) { # First time we hit the target must be the least loss!
			return $loss;
		}

		for (my $td = 1; $td < 4; $td += 2) { # Tries left and right from current dir
			my $nd = ($d+$td)&3;
			my $l = $loss;
			my ($dx,$dy) = ($dx[$nd],$dy[$nd]);
			my ($nx,$ny) = ($x,$y);
			for (my $r = 1; $r <= $maxl; $r++) {
				$nx += $dx; $ny += $dy;
				last if ($nx < 0 || $ny < 0 || $nx > $tx || $ny > $ty);

				$l += substr($inp[$ny],$nx,1);
				next if ($r < $minl);

				my $s = sprintf("%d,%d,%d,%d", $l,$nx,$ny,$nd&1);
				$prio->insert($s,$l*100+$tx+$ty-$nx-$ny);
			}
		}
	}
	die("No solution found!");
}

sub solve_star
{
	my ($minl, $maxl) = @_;

	my %seen = ();
	my ($tx,$ty) = (length($inp[0])-1,scalar(@inp)-1);
	my $prio = new List::PriorityQueue;
	# Start by entering (0,0) from left and above:
	$prio->insert(sprintf("%d,%d,%d,%d,%s",0,$tx+$ty,0,0,0),0);
	$prio->insert(sprintf("%d,%d,%d,%d,%s",0,$tx+$ty,0,0,1),0);
	my @dx = (1,0,-1,0);
	my @dy = (0,1,0,-1);
	my $minloss = 1e38;
	my ($pushes, $pops, $cached, $retries) = (2,0,0,0);
	while (my $m = $prio->pop()) {
		$pops++;
		my ($loss,$manh,$x,$y,$p) = split(/,/,$m);
		next if ($loss+$manh >= $minloss);
		
		my $d = substr($p,-1); # Remember where we came from
		my $k = join(",",$x,$y,$d&1);
		if (defined($seen{$k})) {
			$cached++;
			if	($seen{$k} <= $loss) {
				next;
			}
			$retries++;
		}
		$seen{$k} = $loss;
		
		if ($x == $tx && $y == $ty) { # Is this better?
			printf(STDERR "loss: %d, pushes: %d, pops: %d, cache hits: %d, retries: %d\n", 
				$loss, $pushes, $pops, $cached, $retries);
			if ($loss < $minloss) {
				$minloss = $loss;
			}
		}

		for (my $td = 1; $td < 4; $td += 2) { # Tries left and right from current dir
			my $nd = ($d+$td) & 3;
			my $l = $loss;
			my ($dx,$dy) = ($dx[$nd],$dy[$nd]);
			my ($nx,$ny) = ($x,$y);
			for (my $r = 1; $r <= $maxl; $r++) {
				$nx += $dx; $ny += $dy;
				last if ($nx < 0 || $ny < 0 || $nx > $tx || $ny > $ty);

				$l += substr($inp[$ny],$nx,1);
				next if ($r < $minl);

				my $mh = $tx+$ty-$nx-$ny;
#				my $s = sprintf("%d,%d,%d,%3,%s%d%s", $l,$mh,$nx,$ny,$p,$r-1,$nd);  # Full history!
				my $s = sprintf("%d,%d,%d,%d,%s", $l,$mh,$nx,$ny,$nd);
				$prio->insert($s,$l+$mh>>1);
				$pushes++;
			}
		}
	}
	return $minloss;
}

$part1 = solve(1,3);
$part2 = solve(4,10);
my $used = time-$t0;

printf("solve %s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);

#$t0 = time;
#$part2 = solve_star(4,10);

#$used = time-$t0;

#printf("star %s\n%s\n", $part1, $part2);
#printf("Used %1.3fs\n", $used);
