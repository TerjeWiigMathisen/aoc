#!perl -w
use strict;
use warnings;
no warnings 'recursion';
use Time::HiRes qw(time);
use List::PriorityQueue;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

my @dx = (1,0,-1,0);
my @dy = (0,1,0,-1);

sub shortest_path_starting_with
{
	my ($x,$y,$tx,$ty,$path,@inp) = @_;
	my %seen = ();
	my $prio = List::PriorityQueue->new;
	my $steps = 0;
	my $dir = 0;
	foreach(split(//,$path)) {
		return (-1,'',) if ($seen{"$x,$y"});
		$seen{"$x,$y"}=$steps++;
		$x += $dx[$_];
		$y += $dy[$_];
		$dir=$_;
	}
	$prio->insert("$x,$y,$steps,$dir,$path",0);
		
	# Find shortest!
	my ($shortest,$lp,@longer) = (0,0,());
	while(my $m = $prio->pop) {
		my ($x,$y,$steps,$dir,$path) = split(/,/,$m);
		if ($x==$tx && $y==$ty){
			$shortest = $steps;
			$lp = $path;
			next;
		}
		die if ($y > $ty);
		my $c = substr($inp[$y],$x,1);

		next if ($c eq '#');
#		printf("Try $m -> $c\n");

		my $dd = index('>v<^',$c);
		next if ($dd >= 0 && $dir ne $dd);

		my $se = $seen{"$x,$y"};
		if (defined($se)) {
			if ($se < $steps) {
				push(@longer,$path);
#				printf("Longer: $se < $steps\r");
#				return (-1,$path);
			}
			next;
		}
		$seen{"$x,$y"} = $steps;
		my $ns = $steps+1;
		for (my $d = 0; $d < 4; $d++) {
			my $dd = ($dir+$d)&3;
			next if (($dir ^ $dd) == 2); # Don't backtrack!
			my ($nx,$ny) = ($x+$dx[$dd],$y+$dy[$dd]);
			$prio->insert("$nx,$ny,$ns,$dd,$path$dd",$ns);
		}
	}
	return ($shortest, $lp,@longer);
}

sub p1
{
	my (@inp) = @_;
	unshift (@inp, '#' x length($inp[0]));
	my ($x,$y) = (index($inp[1],'.'),1);
	my $ty = scalar(@inp)-1;
	my $tx = index($inp[$ty],'.');
	
	#my ($x,$y,$tx,$ty,$path,@inp) = @_;
	my ($maxlen,$path,@longer) = shortest_path_starting_with($x,$y,$tx,$ty,'',@inp);
#	printf("First path: %d, %s\n",$maxlen,$path);

	while (scalar(@longer)) {
		my $pa = shift @longer;
		my ($len, $p, @lo) = shortest_path_starting_with($x,$y,$tx,$ty,$pa,@inp);
		if ($len > $maxlen) {
#			printf("Longer path: %d, %s\n",$len,$p);
			$maxlen = $len;
		}
		push(@longer,@lo);
	}
	return $maxlen;
}

my $maxlen = 0;

my %paths = ();
my %junc = ();

sub navigate
{
	my ($sx,$sy,$tx,$ty) = @_;
	my $sq = List::PriorityQueue->new;
	$sq->insert("$sx\t$sy\t0\t",0);
#	printf("%s\n",join(' ',sort keys %paths));
	my $qlen = 1;
	while (my $m = $sq->pop) {
		$qlen--;
		my ($x,$y,$len,$seen) = split(/\t/,$m);
#		printf("$qlen: $m\n");

		for (my $dir = 0; $dir < 4; $dir++) {
			my $p = "$x,$y,$dir";
			if (defined($paths{$p})) {
				my ($px,$py,$plen) = split(/,/,$paths{$p});
				my $k = ";$px,$py;";
				next if (index($seen,$k) >= 0); # Don't backtrack!
#				printf("Trying $p\n");

				my $pl = $len+$plen;
				if ($px==$tx && $py==$ty) {
					if ($maxlen < $pl) {
						$maxlen = $pl;
						printf("%1.3f Found path with len = %d\n", time-$t0, $maxlen); 
					}
					next;
				}
				$sq->insert("$px\t$py\t$pl\t$seen;$x,$y;",1e6-$pl);
				$qlen++;
			}
		}
	}
	return $maxlen;
}

sub p2
{
	my (@inp) = @_;
	unshift (@inp, '#' x length($inp[0]));
	push(@inp,$inp[0]);
	
	my ($sx,$sy) = (index($inp[1],'.'),1);
	my $ty = scalar(@inp)-2;
	my $tx = index($inp[$ty],'.');

	# Check all cells, locate junctions:
	$junc{"$sx,$sy"}++; $junc{"$tx,$ty"}++;
	for (my $y = 1; $y <= $ty; $y++){
		for (my $x = 1; $x < length($inp[0])-1; $x++) {
			my $c = substr($inp[$y],$x,1);
			next if ($c eq '#');
			my $bord = substr($inp[$y],$x+1,1).substr($inp[$y+1],$x,1).
			           substr($inp[$y],$x-1,1).substr($inp[$y-1],$x,1);
			$bord =~ s/[^#]//g;
			my $paths = 4-length($bord);
			if ($paths>2) {
#				printf("Junction at ($x,$y), %d paths meet\n",$paths);
				$junc{"$x,$y"}++;
			}
		}
	}
	foreach (sort keys %junc) {
		my ($sx,$sy) = split(/,/);
		if ($_ eq "3,6") {
			printf("debug");
		}
		for (my $dir = 0; $dir < 4; $dir++) {
			my ($x,$y) = ($sx+$dx[$dir],$sy+$dy[$dir]);
			next if (substr($inp[$y],$x,1) eq '#');
			my $pd = $dir;
#			printf("Trying ($sx,$sy)($x,$y,$dir)\n");
			my $plen = 1;
			#my %seen = ("$sx,$sy"=>1,"$x,$y"=>1);
			my $found;
			while (!defined($junc{"$x,$y"})) {
				$found = 0;
				for (my $d = 0; $d < 4; $d++) {
					next if (($pd^$d) == 2); # No U-turn
					my ($nx,$ny) = ($x+$dx[$d],$y+$dy[$d]);
					next if (substr($inp[$ny],$nx,1) eq '#');
#					next if (defined($seen{"$nx,$ny"}));
#					$seen{"$nx,$ny"} = 1;
					($x,$y) = ($nx,$ny);
					$found = 1;
					$pd = $d;
					last;
				}
				last unless($found);
				$plen++;
			}
			next if ($found == 0 || ($x == $sx && $y == $sy));
			$paths{"$sx,$sy,$dir"} = "$x,$y,$plen";
			printf("Found path ($plen) from ($sx,$sy) to ($x,$y) in dir $dir\n");
		}
	}
	printf("Found %d junctions and %d uni-directional paths\n",scalar(keys %junc), scalar(keys %paths));
	
	my %seen = ();
	my $len = navigate($sx,$sy,$tx,$ty);
	return $len;
}

$part1 = p1(@inp);
$part2 = p2(@inp);

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);

