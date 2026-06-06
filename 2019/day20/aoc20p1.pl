#!perl -w
use strict;
use Time::HiRes qw(time);
use List::PriorityQueue;
use Math::Polygon::Calc;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);

#printf("Input:\n%s\n\n", join("\n",@inp));

my @dx = (1,0,-1,0);
my @dy = (0,1,0,-1);

my %vert = ();
my %portal = ();
my %links_from = ();

my $AA;
my $ZZ;

sub skey
{
	my ($a,$b) = @_;
	if ($a gt $b) {
		($a,$b) = ($b,$a);
	}
	return "$a,$b";
}

sub find_link
{
	my ($x,$y,$dir,$b) = @_;
	return (0,0,0) if (substr($b->[$y],$x,1) ne '.');
	my $len = 0;
	my $pdir = $dir;
	if ($x == 31 && $y == 17) {
				printf("debug find_link");
	}
	while (1) {
		$len++;
		my $k = "$x,$y";
		if (defined($vert{$k})) {
			return ($x,$y,$len);
		}
		my ($nx,$ny);
		my $c;
		for (my $dd = $pdir+3; $dd < $pdir+6; $dd++) {
			$dir = $dd & 3;
			($nx,$ny) = ($x+$dx[$dir],$y+$dy[$dir]);
			$c = substr($b->[$ny],$nx,1);
			last if ($c ne '#');
		}
		if ($c eq '#') {
			return (0,0,0);
		}
		($x,$y) = ($nx,$ny);
		$pdir = $dir;
	}
	die;
}

sub parse
{
	my (@b) = @_;
	my $MX = length($b[0])-2;
	my $MY = scalar(@b)-2;
	%vert = ();
	%links_from = ();
	my %port = ();

	# Find all junctions, as well as special nodes
	for (my $y = 2; $y < $MY; $y++) {
		for (my $x = 2; $x < $MX; $x++) {
			my $c = substr($b[$y],$x,1);
			next if ($c ne '.');
			
			my @n = (substr($b[$y-1],$x,1),substr($b[$y],$x-1,1),
				substr($b[$y],$x+1,1),substr($b[$y+1],$x,1));
			my $walls = join('',@n); 
			$walls =~ s/[^#]//g; 
			$walls = length($walls);
			
			my $k = "$x,$y";
			if ($walls < 2) {
				$vert{$k} = $c;
			}
			if ($n[0] ge 'A' && $n[0] le 'Z') { # portal name above
				my $port = substr($b[$y-2],$x,1).$n[0];
				$port{$k} = $port;
				$vert{$k} = $port;
			}
			elsif ($n[1] ge 'A' && $n[1] le 'Z') { # portal name to left
				my $port = substr($b[$y],$x-2,1).$n[1];
				$port{$k} = $port;
				$vert{$k} = $port;
			}
			elsif ($n[2] ge 'A' && $n[2] le 'Z') { # portal name to right
				my $port = $n[2].substr($b[$y],$x+2,1);
				$port{$k} = $port;
				$vert{$k} = $port;
			}
			elsif ($n[3] ge 'A' && $n[3] le 'Z') { # portal name below
				my $port = $n[3].substr($b[$y+2],$x,1);
				$port{$k} = $port;
				$vert{$k} = $port;
			}
		}
	}
	#link all portals:
	my %seen = ();
	foreach (keys %port) {  # Coords with portals
		my $p = $port{$_};
		if ($vert{$_} eq 'AA') {
			$AA = $_;
		}
		elsif ($vert{$_} eq 'ZZ') {
			$ZZ = $_;
		}

		if (defined($seen{$p})) {
			my $dst = $seen{$p};
			my $src = $_;
#			$vert{$src} = $dst;
#			$vert{$dst} = $src;
			$portal{$src} = $dst;
			$portal{$dst} = $src;
		}
		else {
			$seen{$p} = $_;
		}
	}
	# find all links
	foreach (keys %vert) {
		my $node = $_;
		if ($_ eq '32,17') {
			$_ = $_;
		}
		my ($x,$y) = split(/,/,$_);
#		printf("Vertice: $_ = %s\n",substr($b[$y],$x,1));
		if (defined($portal{$node})) {
			my $tk = $portal{$node}.";0";
#			push(@{$links_from{$node}},$tk);
			printf("Portal %s at $node points to %s\n",$vert{$node},$portal{$node});
		}
		my $len = 0;
		for (my $dir = 0; $dir < 4; $dir++) {
			my ($tx,$ty,$len) = find_link($x+$dx[$dir],$y+$dy[$dir],$dir,\@b);
			next unless ($len);
			my $tk = "$tx,$ty;$len";
			push(@{$links_from{$node}},$tk);
		}
	}
	foreach (sort keys %links_from) {
		printf("$_ -> %s\n", join(" ",@{$links_from{$_}}));
	}
#	foreach (sort keys %pos2
	disp(@b);
}

sub disp
{
	my (@b) = @_;
	for (my $y = 0; $y < scalar(@b); $y++) {
		for (my $x = 0; $x < length($b[$y]); $x++) {
			my $c = substr($b[$y],$x,1);
			if (defined($portal{"$x,$y"})) {
				$c = 'p';
			}
			elsif (defined($vert{"$x,$y"})) {
				$c = 'x';
			}
			printf("%s",$c);
		}
		printf("\n");
	}
}

sub shortest_path
{
	my ($node, $target) = @_;
	my $pq = new List::PriorityQueue;
	$pq->insert("$node;0;$node",0);
	my %seen = ();
	$portal{$node} = undef;
	while (my $m = $pq->pop) {
		my ($node,$len,$path) = split(/;/,$m);
		if ($node eq $target) {
			printf("Found $target!\n");
			return $len;
		}
		next if ($seen{$node});
		$seen{$node}++;
		printf("Trying $m\n");

		if (defined($portal{$node})) {
			printf("Hit portal %s @ $node, jumping to %s\n",$vert{$node}, $portal{$node});
			$node = $portal{$node};
			$seen{$node}++;
			$len++;
			$path .= join("\t",'',$vert{$node},$node,$len);
		}

		
		foreach (@{$links_from{$node}}) {
			my ($ln, $ll) = split(/;/);
			$ll += $len;
			$pq->insert("$ln;$ll;$path\t$ln\t$ll",$ll) if ($ln =~ /^\d/);
		}
	}
	return -1;
}

sub p1{
	my (@b) = @_;
	parse(@b);
	my ($aa,$zz) = ($AA,$ZZ);
	die unless (defined($aa) && defined($zz));
		
	my $len = shortest_path($aa,$zz);
}

sub p2
{
}

$part1 = p1(@inp);
#$part2 = p2(@inp);

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
