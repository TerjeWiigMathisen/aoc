#!perl -w
use strict;
use Time::HiRes qw(time);
use List::PriorityQueue;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my $MAX_REACHABLE;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));
my $b = '#' x (length($inp[0])+2);
my @b = ($b);
my ($SX,$SY);
foreach (@inp) {
	my $s = index($_,'S');
	if ($s >= 0) {
		$SX = $s+1;
		$SY = scalar(@b);
		s/S/\./;
	}
	push(@b,'#'.$_.'#'); 
}
push(@b,$b);

sub reach1
{
	my ($limit) = @_;
	my %seen = ();

	my $prio = List::PriorityQueue->new;
	$prio->insert("$SX,$SY,0",0);
	while (my $m = $prio->pop) {
		my ($x,$y,$steps) = split(/,/,$m);
		next if (substr($b[$y],$x,1) eq '#');
		next if (defined($seen{"$x,$y"}));
		next if ($steps > $limit);
		$seen{"$x,$y"} = $steps;
		$steps++;
		$prio->insert(sprintf("%d,%d,%d",$x-1,$y,$steps),$steps);
		$prio->insert(sprintf("%d,%d,%d",$x+1,$y,$steps),$steps);
		$prio->insert(sprintf("%d,%d,%d",$x,$y-1,$steps),$steps);
		$prio->insert(sprintf("%d,%d,%d",$x,$y+1,$steps),$steps);
	}
	my $r = 0;
	foreach(keys %seen) {
		my $s = $seen{$_};
		if (($s & 1) == 0) {
			$r++;
		}
	}
	return ($r,scalar(keys %seen));
}

my $TOTAL_PER_PAGE;

sub reach2
{
	my ($limit) = @_;
	$limit++;
	my %seen = ();
	@b = @inp;
	my ($sx,$sy) = ($SX-1, $SY-1);;
	my ($xmax,$ymax) = (length($b[0])-1,scalar(@b)-1);
	
	my $prio = List::PriorityQueue->new;
	$prio->insert("$sx,$sy,0,0,1",0);
	my $r = 0;
	my $ps = 0;
	my $pr = 0;
	my $dr = 0;
	my $pdr = 0;
	my @r = ();
	my @delta = ();
	my %page = ();
	my %landing = ();
#	my $dots = join('',@b);
#	$dots =~ s/#//g;
	my $rounds;
	while (my $m = $prio->pop) {
		my ($x,$y,$mx,$my,$steps) = split(/,/,$m);
		if ($x < 0) {$x = $xmax; $mx--; }
		elsif ($x > $xmax) {$x = 0; $mx++; }
		if ($y < 0) {$y = $ymax; $my--; }
		elsif ($y > $ymax) {$y = 0; $my++; }
		
		next if (substr($b[$y],$x,1) eq '#');
		next if (defined($seen{"$x,$y,$mx,$my"}));
		
		$seen{"$x,$y,$mx,$my"} = $steps;
		$page{"$mx,$my"}++;
		if ($steps > $ps) {
			if ($ps % 262 == $limit % 262) {
				printf("%d steps -> $r reachable\n", $ps-1);
				for (my $y = -10; $y <= 10; $y++) {
					for (my $x = -10; $x <= 10; $x++) {
						my $p = $landing{"$x,$y"};
						if (defined($p)) {
							printf("%d",$p);
						}
						printf(",") if ($x < 10);
					}
					printf("\n");
				}
				printf("\n");
			}
			$ps = $steps;
		}
		last if ($steps > $limit);

		if (($steps^$limit)&1) {
			$landing{"$mx,$my"}++;
			$r++;
		}
		if ($page{"$mx,$my"} == $TOTAL_PER_PAGE) {
#			printf("Filled (%d,%d) in %d steps\n",$mx,$my,$steps);
			$page{"$mx,$my"} = -$steps;
			if ("$mx,$my" eq "1,1") {
				my $rem = $limit-$steps;
				printf("Remaining $rem step\n");
				my $period = -$page{"0,0"}*2;
				printf("Periode = %d\n", $period);
				my $skip_rounds = int($rem/$period);
				printf("Skipping %d rounds\n", $skip_rounds);
				$rounds = $skip_rounds;
				my $skip = $skip_rounds*$period;
				$limit -= $skip;
				printf("Skipping $skip steps, new limit $limit\n");
			}
		}

		$steps++;
		$prio->insert(sprintf("%d,%d,%d,%d,%d",$x-1,$y,$mx,$my,$steps),$steps);
		$prio->insert(sprintf("%d,%d,%d,%d,%d",$x+1,$y,$mx,$my,$steps),$steps);
		$prio->insert(sprintf("%d,%d,%d,%d,%d",$x,$y-1,$mx,$my,$steps),$steps);
		$prio->insert(sprintf("%d,%d,%d,%d,%d",$x,$y+1,$mx,$my,$steps),$steps);
	}
	my $xm = 0;
	for (my $y = -10; $y <= 10; $y++) {
		for (my $x = -10; $x <= 10; $x++) {
			my $p = $landing{"$x,$y"};
			if (defined($p)) {
				printf("%d",$p);
				if ($x > $xm) {
					$xm = $x;
				}
			}
			printf(",") if ($x < 10);
		}
		printf("\n");
	}
	printf("\n");
	printf("Visit $r plots, xmax=$xmax\n");
	if (defined($landing{"$xm,1"})) {
		$xm++;
		$landing{"$xm,0"} = 0;
		$landing{"-$xm,0"} = 0;
		$landing{"0,$xm"} = 0;
		$landing{"0,-$xm"} = 0;
	}

	my $sum = 0;
	my @center = ($landing{"0,0"},$landing{"0,1"});
	printf("%s\n",join(',',@center));
	my $xm1 = $xm-1;
	my $xm2 = $xm-2;
	my @l = (
		$landing{"$xm1,0"},$landing{"$xm2,1"},$landing{"0,$xm1"},$landing{"-1,$xm2"},
		$landing{"-$xm1,0"},$landing{"-$xm2,-1"},$landing{"0,-$xm1"},$landing{"1,-$xm2"},
		$landing{"$xmax,0"},$landing{"$xm1,1"},$landing{"0,$xmax"},$landing{"-1,$xm1"},
		$landing{"-$xmax,0"},$landing{"-$xm1,-1"},$landing{"0,-$xmax"},$landing{"1,-$xm1"});
	printf("%s\n",join(',',@l));

	# Full blocks 123235041266330
	#             492907943173707
	#852 steps -> 627408 reachable
	#590 steps -> 300518 reachable
	#328 steps -> 92596 reachable
	#
	$rounds = $rounds*2 + $xm2;
	$sum = $center[0];
	for (my $i = 1; $i <= $rounds; $i++) {
		my $loop = $l[$i&1] *$i*4;
		$sum += $loop;
	}
	#First boundary layer: W,NW,N,NE,E,SE,S,SW
	$sum += $l[0]+$l[2]+$l[4]+$l[6] + ($l[1]+$l[3]+$l[5]+$l[7])*($rounds-1);
	# Second boundary layer
	$sum += $l[8]+$l[10]+$l[12]+$l[14] + ($l[9]+$l[11]+$l[13]+$l[15])*$rounds;
	return $sum;
}

sub reach2brute
{
	my ($limit, @inp) = @_;
	my %seen = ();
	@b = @inp;
	my ($sx,$sy)=($SX-1,$SY-1);
	printf("sx=$sx, sy=$sy\n");
	my ($xmax,$ymax) = (length($b[0])-1,scalar(@b)-1);
	
	my $prio = List::PriorityQueue->new;
	$prio->insert("$sx,$sy,0,0,0",0);
	my $r = 0;
	my $ps=0;
	my %page = ();
	my %landing = ();
	while (my $m = $prio->pop) {
		my ($x,$y,$mx,$my,$steps) = split(/,/,$m);
		if ($x < 0) {$x = $xmax; $mx--; }
		elsif ($x > $xmax) {$x = 0; $mx++; }
		if ($y < 0) {$y = $ymax; $my--; }
		elsif ($y > $ymax) {$y = 0; $my++; }
		
		next if (substr($b[$y],$x,1) eq '#');
		next if (defined($seen{"$x,$y,$mx,$my"}));

		$seen{"$x,$y,$mx,$my"} = $steps;
		$page{"$mx,$my"}++;
		if ($steps > $ps) {
			if ($ps > 262*1 && ($limit-$ps) % 262 == 0) {
				my $xm = 0;
#				printf("%d steps -> $r reachable\n", $ps);
				for (my $y = -10; $y <= 10; $y++) {
					for (my $x = -10; $x <= 10; $x++) {
						my $p = $landing{"$x,$y"};
						if (defined($p)) {
#							printf("%d",$p);
							if ($x > $xm) {
								$xm = $x;
							}
						}
#						printf(",") if ($x < 10);
					}
#					printf("\n");
				}
#				printf("xm = $xm\n");
				my @center = ($landing{"0,0"},$landing{"0,1"});
				printf("%s\n",join(',',@center));
				
				if (defined($landing{"$xm,1"})) {
					$xm++;
					$landing{"$xm,0"} = 0;
					$landing{"-$xm,0"} = 0;
					$landing{"0,$xm"} = 0;
					$landing{"0,-$xm"} = 0;
				}
				my $xm1 = $xm-1;
				my $xm2 = $xm-2;
				my @l = (
					$landing{"$xm1,0"},$landing{"$xm2,1"},$landing{"0,$xm1"},$landing{"-1,$xm2"},
					$landing{"-$xm1,0"},$landing{"-$xm2,-1"},$landing{"0,-$xm1"},$landing{"1,-$xm2"},
					$landing{"$xm,0"},$landing{"$xm1,1"},$landing{"0,$xm"},$landing{"-1,$xm1"},
					$landing{"-$xm,0"},$landing{"-$xm1,-1"},$landing{"0,-$xm"},$landing{"1,-$xm1"});
#				printf("%s\n",join(',',@l));

				# 1637 steps -> 2325260 reachable
				
				my $s = 0; foreach (keys %landing) { $s += $landing{$_}; }
#				printf("Total marked spots = %d, r = %d\n",$s,$r);
				my $sum;
				# $ps
				for (my $lim = $limit; $lim <= $limit; $lim += 262) {
					my $rem = $lim-$ps;
					printf("Remaining $rem steps\n");
					my $period = 262;
					printf("Period = %d\n", $period);
					my $skip_rounds = $rem/$period;
					printf("Skipping %d double rounds\n", $skip_rounds);
					die unless ($skip_rounds == int($skip_rounds));
					
					my $rounds = $skip_rounds;
					my $skip = $skip_rounds*$period;
#					$limit -= $skip;
					printf("Skipping final $skip steps\n");
					$rounds = $rounds*2 + $xm2;
					$sum = $center[0];
#					printf("Core size; %d, center =%d\n", $rounds, $sum);
					for (my $i = 1; $i <= $rounds; $i++) {
						my $loop = $center[$i&1] *$i*4;
						$sum += $loop;
#						printf("loop $i = $loop, sum=$sum\n");
					}
#					printf("Core sum: %d\n", $sum);
					#First boundary layer: W,NW,N,NE,E,SE,S,SW
					my $b1 = $l[0]+$l[2]+$l[4]+$l[6] + ($l[1]+$l[3]+$l[5]+$l[7])*($rounds);
					$sum += $b1;
					# Second boundary layer
					my $b2 = $l[8]+$l[10]+$l[12]+$l[14] + ($l[9]+$l[11]+$l[13]+$l[15])*($rounds+1);
					$sum += $b2;
#					printf("lim: %d, b1: $b1, b2: $b2, sum: %d\n", $lim, $sum);
				}
				return $sum;
			}
			$ps = $steps;
		}
		last if ($steps > $limit);

		if ((($steps^$limit)&1)==0) {
			$landing{"$mx,$my"}++;
			$r++;
		}

		$steps++;
		$prio->insert(sprintf("%d,%d,%d,%d,%d",$x-1,$y,$mx,$my,$steps),$steps);
		$prio->insert(sprintf("%d,%d,%d,%d,%d",$x+1,$y,$mx,$my,$steps),$steps);
		$prio->insert(sprintf("%d,%d,%d,%d,%d",$x,$y-1,$mx,$my,$steps),$steps);
		$prio->insert(sprintf("%d,%d,%d,%d,%d",$x,$y+1,$mx,$my,$steps),$steps);
	}
	return $r;
}

my $dummy;
#($part1,$dummy) = reach1(6);
#printf("Reach $part1 in 6 steps\n");
($part1,$dummy) = reach1(64);

my ($r, $max) = reach1(scalar(@inp)*4);
#printf("Reached $max dots\n");
$TOTAL_PER_PAGE = $max;

$part2 = reach2brute(26501365,@inp);

my $used = time-$t0;

printf("Part1: %s\nPart2: %s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);
