#!perl -w
use strict;
use warnings;
no warnings 'recursion';
use Time::HiRes qw(time);
use List::PriorityQueue;

my $t0 = time;
my $part1 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

my %link = ();
my %links_to_nodes = ();
my %node = ();

my %used = ();
my %cuts = ();

my %route = ();
my %used_by = ();

sub okey
{
	my ($a,$b) = @_;
	if ($a gt $b) {
		($a,$b) = ($b,$a);
	}
	return join(',',$a,$b);
}

my @nodes;

sub spanning_tree
{
	my ($start) = @_;
	my $prio = List::PriorityQueue->new;
	my %seen = ();
#	my $ns = $nodes[$start];
    my $ns = $start;
	$prio->insert(sprintf("%s,0",$ns));
	while (my $m = $prio->pop) {
		my ($n,$len) = split(/,/,$m);

		next if ($seen{$n});
		$seen{$n} = $len++;
		foreach (@{$links_to_nodes{$n}}) {
			next if ($seen{$_});
			my $key = okey($n,$_);
			next if ($cuts{$key});
			$used{$key}++;
			$prio->insert(join(",",$_,$len),$len);
		}
	}
	return %seen;
}

my $RANDOM_STARTS = 10;

sub cut3
{
	my (@cuts) = @_;
	printf("starting cut set (%s)\n", join(' ',@cuts)) if (scalar(@cuts) <= 2);
	%cuts = (); foreach (@cuts) {$cuts{$_}++; }
	@nodes = sort keys %node;
	my $target_nodes = scalar(@nodes);
	%used = ();
	for (my $i = 0; $i < $RANDOM_STARTS; $i++) {
		my $start = int(rand()*$target_nodes);
		my $nodes_reached = spanning_tree($start);
		my $other_set = $target_nodes-$nodes_reached;
		my $prod = $nodes_reached*$other_set;
		if ($prod > $target_nodes) {
			printf("Bisected by (%s), into %d and %d parts\n",join(" ",@cuts),$nodes_reached, $other_set);
			return scalar(@cuts)>3 ? 0:$prod;
		}
	}
	my @top_links = sort {$used{$b} <=> $used{$a}} keys %used;
	foreach(@top_links[0..9]) {
#		printf("%s %s\n",$_, $used{$_}) if (scalar(@cuts) < 2);
	}
	return 0 if (scalar(@cuts) > 3);
	foreach(@top_links[0..24]) {
		my $res = cut3(@cuts,$_);
		return $res if ($res);
	}
	return 0;
}

sub shortest_route
{
	my ($s,$d) = @_;
	my $prio = List::PriorityQueue->new;
	my %seen = ();
	$prio->insert("$s,0,",0);
	while (my $m = $prio->pop) {
		my ($n,$len,$path) = split(/,/,$m);
		if ($n eq $d) {
#			printf("$s->$d = $path\n");
			return split(/;/,$path);
		}

		next if (defined($seen{$n}) && $seen{$n} <= $len);
		$seen{$n} = $len++;
		my $r = $route{join(";",$n,$d)};
		if ($r) {
			my @path = split(/;/,$path);
			pop(@path);
			push(@path,split(/;/,$r));
			return @path;
		}
		my $pt = $path;
		$pt .= ';' unless ($pt eq '');
		foreach (@{$links_to_nodes{$n}}) {
			my $key = join(",",sort ($n,$_));
			next if ($cuts{$key});
			$prio->insert(join(",",$_,$len,$pt.$_),$len);
		}
	}
	printf("No path found!");
	return ();
}

sub cut_link
{
	my ($k) = @_;
	foreach (@{$used_by{$k}}) {
		$route{$_} = undef;
	}
}

sub most_used
{
	my (@cuts) = @_;
	%cuts = (); foreach (@cuts) {$cuts{$_}++; }
	%used_by = ();
	%used = ();
	my %routes = ();
	my $cnt = 0;
	foreach (@nodes) {
		my $a = $_;
		$cnt++;
		printf("%3d $a %d used, %d routes\n",$cnt, scalar(keys %used),scalar(keys %route));
		foreach (@nodes) {
			$b = $_;
			next if ($a ge $b);
			next if (defined($route{"$a,$b"}));

			my $t0 = time;
			my @path = shortest_route($a,$b);
			$t0 = time-$t0;
			printf("%3d $a $b %d used, %d routes %1.6f (%d steps)\n",
				$cnt, scalar(keys %used),scalar(keys %route),$t0, scalar(@path));

			unless(scalar(@path)) {
				printf("Graph is cut between %s and %s\n",$a,$b);
				return ();
			}
#			$route{"$a,$b"} = join(';',@path);
			$route{"$a,$b"}++;
			for (my $i = 1; $i < scalar(@path); $i++) {
				my $d = $path[$i];
				my $link = join(",",sort($d,$path[$i-1]));
				$used{$link}++;
				for (my $j = 0; $j < $i; $j++) {
					my $s = $path[$j];
					my $r = join(",",sort ($s,$d));
#					$route{$r} = join(";",@path[$j..$i]);
					$route{$r}++;
				}
#				push(@{$used_by{$link}},join(",",$nodes[$a],$nodes[$b]));
			}
		}
	}
	my @used = sort {$used{$b} <=> $used{$a}} keys %used;
	foreach(@used[0..9]) {
		printf("%3d %s\n",$used{$_},$_);
	}
	return @used;
}

sub p1
{
	my (@inp) = @_;
	foreach(@inp) {
		die unless(/^(\S+): (\S.*)$/);
		my $src = $1;
		my @dests = split(/ /,$2);
		foreach(@dests) {
			$link{okey($src,$_)}++;
			push(@{$links_to_nodes{$src}},$_);
			push(@{$links_to_nodes{$_}},$src);
			$node{$_}++;
			$node{$src}++;
		}
	}
	my @nodes = sort keys %node;
	$|=1;
	printf("Found %d nodes and %d links_to_nodes\n",scalar(keys %node), scalar(keys %link));
	my @sorted_nodes = sort {scalar(@{$links_to_nodes{$a}}) <=> scalar(@{$links_to_nodes{$b}})} keys %links_to_nodes;
	foreach(@sorted_nodes[0..9],@sorted_nodes[-10..-1]) {
		printf("Node %s has %d links_to_nodes\n",$_,scalar(@{$links_to_nodes{$_}}));
	}
	# Pick a low-connected node as source, then find a node far away
	
	my $source = $sorted_nodes[0];

	my $dest;
	my @path = ();
	for (my $d = -2; $d >= -6; $d--) {
		my $dst = $sorted_nodes[$d];
		my @pth = shortest_route($source,$dst);
		printf("$source -> $dst (%s)\n",join(" ",@pth));
		next unless (scalar(@pth) > scalar(@path));
		$dest = $dst;
		@path = @pth;
	}
	while (1) {
		my $p = $source;
		foreach(@path) {
			$cuts{okey($p,$_)}++;
			$p = $_;
		}
		@path = shortest_route($source,$dest);
		last unless(@path);
		printf("$source -> $dest (%s)\n",join(" ",@path));
	}
	my %seen = spanning_tree($source);
	my $p1 = scalar(keys %seen);
	my $p2 = scalar(@nodes)-$p1;
	printf("$p1 x $p2 = %d\n",$p1*$p2);
	
	die;
	my %fan = ();
	foreach (keys %link) {
		my ($a,$b) = split(/,/);
		$fan{$_} = scalar(@{$links_to_nodes{$a}})+scalar(@{$links_to_nodes{$b}});
	}
	my @fan = sort {$fan{$a}<=>$fan{$b}} keys %fan;
	foreach(@fan[0..9],@fan[-10..-1]) {
		printf("Link %s has %d fanout\n",$_,$fan{$_});
	}
	my $res;
	for (my $a = 0; $a < 8; $a++) {
		for (my $b = $a+1; $b<9; $b++) {
			for (my $c = $b+1; $c < 10; $c++) {
				$res = cut3($fan[$a],$fan[$b],$fan[$c]);
				last if $res;
			}
		}
	}
	die("No solution found!") unless ($res);
	return $res;
}

$part1 = p1(@inp);

my $used = time-$t0;

printf("%s\n", $part1);
printf("Used %1.3fs\n", $used);

