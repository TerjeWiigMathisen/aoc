#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
#use List::Util qw (reduce);

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $DEBUG = 0;

my $fname = shift;
$fname = 'input.txt' unless (defined($fname));
open(F ,'<',$fname);

my $start = time;

my $part1 = 0;
my $part2 = 0;

my $inp = '';
foreach (<F>) {
	$inp .= $_;
}
close(F);

my %v = ();
my @active_nodes = ();
sub parse
{
	foreach (split(/\n/,$inp)) {
		#Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
		my ($name, $flow, $paths);
		die("Valve error: $_") unless (/Valve (\S+) /);
		$name = $1;
		die("Flow error: $_") unless (/has flow rate.(\d+)./);
		$flow = $1;
		die("Tunnels error: $_") unless (/tunnel.? lead.? to valve.?\s(.*)$/);
		push(@active_nodes, $name) if ($flow);

		$paths = $1;
		$paths =~ s/\s//g;
		my @paths = split(/,/, $paths);
		my %room = ();
		$room{"name"} = $name;
		$room{"flow"} = $flow;
		$room{"paths"} = \@paths;
		$room{"open"} = 0;
		$v{$name} = \%room;
	}
	@active_nodes = sort {$v{$b}->{"flow"} <=> $v{$a}->{"flow"}} @active_nodes; # In max flow order!
}

my ($dcache, $ddist, $dreserve, $dsymmetry, $dprune, $dsearch) = (0,0,0,0,0,0);

my %dcache = ();
sub distances
{
	my ($curr) = @_;
	my $dist = 0;
	if (defined($dcache{$curr})) {
		$dcache++;
		return $dcache{$curr};
	}
	$ddist++;
	my %visited = ();
	my @list = ($curr);
	while (scalar(@list)) {
		my @next = ();
		foreach (@list) {
			my $c = $_;
			next if (defined(($visited{$c})));
			$visited{$c} = $dist;
			foreach (@{$v{$c}->{"paths"}}) {
				push(@next, $_);
			}
		}
		@list = @next;
		$dist++;
	}
	$dcache{$curr} = \%visited;
	return \%visited;
}

sub show
{
	my ($curr) = @_;
	my $d = distances($curr);
	foreach (sort keys %v) {
		printf("%s",$_ eq $curr ? "->":"  ");
		printf("%3d %s %s %2d(%2d) %s\n", $d->{$_}, $_, $v{$_}->{"name"}, $v{$_}->{"flow"}, 
			$v{$_}->{"open"}, join(',',@{$v{$_}->{"paths"}}));
	}
	printf("There are %d active valves: %s\n", scalar(@active_nodes), join(",", @active_nodes));
}

parse();
#show('AA');

sub search
{
	my ($timeleft, $total, @active) = @_;
	if ($total >= $part1) {
		#printf(STDERR "left: %2d flow = %d from %s\n", $timeleft, $total, join(',', reverse @active));
		$part1 = $total;
	}
	my $curr = $active[0];
	my $d = distances($curr);
	my %active = (); foreach (@active) { $active{$_}++; }
	
	# Is it possible to beat the current best?
	my $pot = $total; # Current total
	my @next = ();
	my $tm = $timeleft-2; # One step to get there + plus one to open
	foreach (@active_nodes) { # Only look at nodes with positive flow
		next if ($active{$_}); # Skip those already opened
		my $f = $v{$_}->{"flow"};
		
		push(@next, $_);
		$pot += $tm * $f;
		$tm -= 2; # Step + open
		last if ($tm <= 0);
	}
	if ($pot <= $part1) {
		return;	 # No way to reach the current max!
	}

	foreach (@next) {
		my $f = $v{$_}->{"flow"};
		my $dist = $d->{$_};
		my $dt = $timeleft - $dist - 1; # -1 to open
		next if ($dt <= 0);
		search($dt, $total + $dt * $f, $_, @active);
	}
}

search(30, 0, 'AA');

my $reserved = '';

my %symmetry = ();

sub search2
{
	my ($timeleft, $timeleft2, $total, @active) = @_;
	$dsearch++;
	if ($total >= $part2) {
		#printf(STDERR "%f left: %2d,%2d flow = %d from %s\n", time - $start,
		#	$timeleft, $timeleft2, $total, join(',', reverse @active));
		$part2 = $total;
	}
	my %active = (); foreach (@active) { $active{$_}++; } # open valves hash

	my ($curr, $ele) = splice(@active,0,2); # Mine/Elephant position

	if ($timeleft == $timeleft2) { # Are we in a potentially symmetrical setup?
		my $list = join(',',sort($curr, $ele), sort(@active));
		if ($symmetry{$list} && $symmetry{$list} >= $total) { # Curr pos, visited and total
			#printf("Skipping due to symmetry: %s\n", $list);
			$dsymmetry++;
			return;
		}
		$symmetry{$list} = $total; # Best score seen here
	}

# Is it even possible to beat the current best?
	my $pot = $total; # Current total
	my @next = (); # List of nodes to try next
	my $tm = $timeleft;
	my $te = $timeleft2;
	foreach (@active_nodes) {
		next if ($active{$_});
		my $f = $v{$_}->{"flow"};
		if ($tm >= $te) {
			$tm -= 2; #At least once cycle to reach + one cycle to open
			$pot += $tm * $f if ($tm > 0);
		}
		else {
			$te-=2;
			$pot += $te*$f if ($te > 0);
		}
		push(@next, $_);
	}
	if ($pot <= $part2) {
		$dprune++;
		return;	 # No way to reach the current max!
	}

	if ($timeleft >= $timeleft2) { # Pick the actor with the most time left

		my $d = distances($curr);
		foreach (@next) {
			#$reserved = $_ unless ($reserved); # Break symmetry!

			my $f = $v{$_}->{"flow"};
			#next unless ($f);

			my $dist = $d->{$_};
			my $dt = $timeleft - $dist - 1; # -1 to open
			next if ($dt <= 0);

			search2($dt, $timeleft2, $total + $dt * $f, $_, $ele, $curr, @active);
		}
	}
	else {
		my $de = distances($ele);
		foreach (@next) {
			my $fe = $v{$_}->{"flow"};

			my $diste = $de->{$_};
			my $dte = $timeleft2 - $diste - 1; # -1 to open
			next if ($dte <= 0);
		
			search2($timeleft, $dte, $total + $dte * $fe, $curr, $_, $ele, @active);
		}
	}
}

printf(STDERR "Part1 time = %f\n", time - $start);
printf("Part1: %s\n", $part1);

search2(26,26,0,'AA','AA');

printf(STDERR "Total time = %f\n", time - $start);

printf(STDERR "search: %d, distance: %d (cache %d), reserved: %d, symmetry: %d, prune: %d\n",
	$dsearch, $ddist, $dcache, $dreserve, $dsymmetry, $dprune);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
