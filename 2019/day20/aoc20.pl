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
			last if ($c eq '.');
		}
		if ($c ne '.') {
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

	my $ymid = $MY>>1;
	my $xmid = $MX>>1;
	my $xinner = $xmid - ($xmid>>2);
	my $yinner = $ymid - ($ymid>>2);
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
			
			my $port;
			
			my $k = "$x,$y";
			if ($walls < 2) {
				$vert{$k} = $c;
			}
			my $inner;
			if ($n[0] ge 'A' && $n[0] le 'Z') { # portal name above
				$port = substr($b[$y-2],$x,1).$n[0];
				$inner = $y > $ymid;
			}
			elsif ($n[1] ge 'A' && $n[1] le 'Z') { # portal name to left
				$port = substr($b[$y],$x-2,1).$n[1];
				$inner = $x > $xmid;
			}
			elsif ($n[2] ge 'A' && $n[2] le 'Z') { # portal name to right
				$port = $n[2].substr($b[$y],$x+2,1);
				$inner = $x < $xmid;
			}
			elsif ($n[3] ge 'A' && $n[3] le 'Z') { # portal name below
				$port = $n[3].substr($b[$y+2],$x,1);
				$inner = $y < $ymid;
			}
			if (defined($port)) {
				if (abs($xmid-$x) < $xinner && abs($ymid-$y) < $yinner) {
					$port = lc($port);
				}
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
			$portal{$_} = $_;
		}

		if (defined($seen{lc($p)})) {
			my $dst = $seen{lc($p)};
			my $src = $_;
#			$vert{$src} = $dst;
#			$vert{$dst} = $src;
			$portal{$src} = $dst;
			$portal{$dst} = $src;
		}
		else {
			$seen{lc($p)} = $_;
		}
	}
	# find all links
	foreach (keys %vert) {
		my $node = $_;
		my ($x,$y) = split(/,/,$_);
#		printf("Vertice: $_ = %s\n",substr($b[$y],$x,1));
#		if (defined($portal{$node})) {
#			my $tk = $portal{$node}.";1";
#			push(@{$links_from{$node}},$tk);
#			printf("Portal %s at $node points to %s\n",$vert{$node},$portal{$node});
#		}
		my $len = 0;
		for (my $dir = 0; $dir < 4; $dir++) {
			my ($tx,$ty,$len) = find_link($x+$dx[$dir],$y+$dy[$dir],$dir,\@b);
			next unless ($len);
			my $tk = "$tx,$ty;$len";
			push(@{$links_from{$node}},$tk);
		}
	}
#	foreach (sort keys %links_from) {
#		printf("$_ -> %s\n", join(" ",@{$links_from{$_}}));
#	}
#	foreach (sort keys %pos2
#	disp(@b);
}

sub disp
{
	my (@b) = @_;
	for (my $y = 0; $y < scalar(@b); $y++) {
		for (my $x = 0; $x < length($b[$y]); $x++) {
			my $c = substr($b[$y],$x,1);
			my $k = "$x,$y";
			if (defined($portal{$k})) {
				$c = 'P';
				if (lc($vert{$k}) eq $vert{$k}) {
					$c = 'p';
				}
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
#	$portal{$node} = undef;
	while (my $m = $pq->pop) {
		my ($node,$len,$path) = split(/;/,$m);
		if ($node eq $target) {
			printf("Found $target!\n");
			return $len;
		}
		next if ($seen{$node} && $seen{$node} <= $len);
		$seen{$node} = $len;

		if ($len && defined($portal{$node})) {
			$len++;
			$node = $portal{$node};
			$path = join("\t",$path,$vert{$node},$len);
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

my %path_cache = ();

sub spanning_tree
{
	my ($node) = @_;
	if (defined($path_cache{$node})) {
		return $path_cache{$node};
	}
	my $pq = new List::PriorityQueue;
	$pq->insert(join(";",$node,0),0);
	my %seen = ();
	my @paths = ();
	while (my $m = $pq->pop) {
#		printf(STDERR "%s\n",$m);
		my ($node,$len) = split(/;/,$m);
		next if ($seen{$node});
		$seen{$node}++;
		if ($portal{$node} && $len) {
			push(@paths,join("\t",$node,$vert{$node},$len));
			next;
		}
#		printf("Trying $m\n");

		foreach (@{$links_from{$node}}) {
			my ($ln, $ll) = split(/;/);
			$ll += $len;
			next if ($seen{$ln});
			$pq->insert("$ln;$ll",$ll);
		}
	}
	$path_cache{$node} = \@paths;
	return \@paths;
}

sub shortest_path2
{
	my ($node, $target) = @_;
	my $pq = new List::PriorityQueue;
	$pq->insert("$node;0;0;$node",0);
	my %seen = ();
	while (my $m = $pq->pop) {
#		printf("%s\n",$m);
		my ($node,$level,$len,$path) = split(/;/,$m);
		next if ($level < 0);
		if ($node eq $target) {
			if ($level == 0) {
				printf("Found $target!\n");
#				printf("%s\n",$path);
				return $len;
			}
			next;
		}
		next if ($seen{"$node;$level"} && $seen{"$node;$level"} <= $len);
		$seen{"$node;$level"} = $len;
#		next if ($len > 400);
#		next if ($level > 10);

		if ($len) {
			$level--;
			if ($vert{$node} eq lc($vert{$node})) {
				$level+=2;
			}
			next if ($level < 0);
			$len++;
			$node = $portal{$node};
#			$seen{"$node;$level"}++;
			$path = join("\t",$path,$vert{$node},$len);
		}
		
		my $paths = spanning_tree($node);

		foreach (@{$paths}) {
			my ($ln,$lv,$ll) = split(/\t/);
#			next if ($seen{"$ln;$level"});
			$ll += $len;
			my $p = join("\t",$path,$lv,$ll);
			$pq->insert("$ln;$level;$ll;$p",$ll);
		}
	}
	return -1;
}

sub p2
{
	my (@b) = @_;
	parse(@b);
	my ($aa,$zz) = ($AA,$ZZ);
	die unless (defined($aa) && defined($zz));
#	disp(@b);
	my $len = shortest_path2($aa,$zz);
}

$part1 = p1(@inp);
$part2 = p2(@inp);

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
