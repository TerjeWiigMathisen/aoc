#!perl -w
use strict;
use warnings;
use Time::HiRes qw(time);
use List::PriorityQueue;
#use Math::Polygon::Calc;

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);

#printf("Input:\n%s\n\n", join("\n",@inp));

my @dx = (1,0,-1,0);
my @dy = (0,1,0,-1);

my %vert = ();
my %keypos = ();
my %pos2key = ();
my %lockpos = ();
my %pos2lock = ();
my %links_from = ();
my @START = ();


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
	return (0,0,0) if (substr($b->[$y],$x,1) eq '#');
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
	my $MX = length($b[0])-1;
	my $MY = scalar(@b)-1;
	%vert = ();
	%links_from = ();
	@START = ();

	# Find all junctions, as well as special nodes
	for (my $y = 1; $y < $MY; $y++) {
		for (my $x = 1; $x < $MX; $x++) {
			my $c = substr($b[$y],$x,1);
			next if ($c eq '#');
			my $paths = (substr($b[$y-1],$x,1) ne '#') + (substr($b[$y],$x-1,1) ne '#') +
						(substr($b[$y],$x+1,1) ne '#') + (substr($b[$y+1],$x,1) ne '#');
			my $k = "$x,$y";
			if ($paths > 2) {
				$vert{$k} = $c;
			}
			if ($c ge 'a' && $c le 'z') {
				$keypos{$c} = $k;
				$pos2key{$k} = $c;
				$vert{$k} = $c;
			}
			elsif ($c ge 'A' && $c le 'Z') {
				$lockpos{$c} = $k;
				$pos2lock{$k} = $c;
				$vert{$k} = $c;
			}
			elsif ($c eq '@') {
				push(@START,$k);
				$vert{$k} = $c;
			}
		}
	}
	# find all links
	foreach (keys %vert) {
		my $node = $_;
		my ($x,$y) = split(/,/,$_);
#		printf("Vertice: $_ = %s\n",substr($b[$y],$x,1));
		my $len = 0;
		for (my $dir = 0; $dir < 4; $dir++) {
			my ($tx,$ty,$len) = find_link($x+$dx[$dir],$y+$dy[$dir],$dir,\@b);
			next unless ($len);
			my $tk = "$tx,$ty;$len";
			push(@{$links_from{$node}},$tk);
		}
	}
	foreach (sort keys %links_from) {
#		printf("$_ -> %s\n", join(" ",@{$links_from{$_}}));
	}
#	foreach (sort keys %pos2
}

sub spanning_tree
{
	my ($node,$keys) = @_;
	my $pq = new List::PriorityQueue;
	my @paths = ();
	my %seen = ();
	if ($node eq "7,5") {
		printf("bottom right @");
	}
	$pq->insert("$node;0",0);
	while (my $m = $pq->pop) {
		my ($node,$len) = split(/;/,$m);
		next if ($seen{$node});
		$seen{$node}++;
		
		my $k = $pos2key{$node};
		if (defined($k)) {		# Is this a key?
			if (index($keys,$k) < 0) { # which we haven't already collected?
				push(@paths,$m);
				next;
			}
		}
		my $l = $pos2lock{$node};
		if (defined($l)) {	# This is a lock
			if (index($keys,lc($l)) < 0) { # Do we have the needed key?
#				push(@paths,$m); # We need to pick up this key at some point!
				next; # No, so abandon this path!
			}
		}
		foreach (@{$links_from{$node}}) {
			my ($ln, $ll) = split(/;/);
			$ll += $len;
			$pq->insert("$ln;$ll",$ll);
		}
	}
	return @paths;
}

sub p1{
	my (@b) = @_;
	
	my $prio = new List::PriorityQueue;
	my %seen = ();
	my $total_keys = scalar(keys %keypos);
	my ($SX,$SY) = split(/,/,$START[0]);
	my $start = "$SX,$SY;0;";
	$prio->insert($start,);
	my $minlen = 1e38;
	my $maxlen = 0;
	while (my $m = $prio->pop) {
		my ($node,$len,$keys) = split(/;/,$m);
		if (length($keys) > $maxlen) {
			$maxlen = length($keys);
			printf("Found %d keys in %d: %s\n",$maxlen, $len, $keys);
			if (length($keys) >= $total_keys) {
				if ($len < $minlen) {
					printf("%4d: %s\n",$len,$keys);
					$minlen = $len;
				}
				next;
			}
		}
		my @keys = split(//,$keys);
		next if ($len > 10000);
		my ($x,$y) = split(/,/,$node);
		my $se = join("\t",$node,join('',sort @keys));
		next if (defined($seen{$se}) && $seen{$se} <= $len);
		$seen{$se} = $len;

#		printf(STDERR "%-30s (%s)\n", $m, substr($b[$y],$x,1));

		my @paths = spanning_tree($node,$keys);
		foreach (@paths) {
			my ($target,$tl) = split(/;/);
			$tl += $len;

			my $k = $pos2key{$target};
			$keys = join("", (@keys));
			if (defined($k)) {
				if ($k eq 'h') {
					printf("FOund the h key\n");
				}
				$keys .= $k;
			}
			$prio->insert(join(";",$target,$tl,$keys),($total_keys-length($keys))*1+$tl*100);
		}
	}
	return $minlen;
}

sub p2
{
	my (@b) = @_;

	my $prio = new List::PriorityQueue;
	my %seen = ();
	my $total_keys = scalar(keys %keypos);
	
	printf("%s\n\n",join("\n",@b));
	
	$prio->insert(join(";",0,'',@START),0);

	my $minlen = 1e38;
	my $maxlen = 0;
	my $qlen = 1;
	my $cachehits = 0;
	my $high_pri = 0;
	while (my $m = $prio->pop) {
		$qlen--;
		my ($len,$keys,@nodes) = split(/;/,$m);
#		printf("%s\n",$m);
#		if (substr($keys,0,3) eq'abi') {
#			printf("Should not be possible...");
#		}
		if (length($keys) >= $maxlen) {
			if (length($keys) > $maxlen) {
				$maxlen = length($keys);
				$minlen = $len+1;
			}
			if ($len < $minlen) {
				printf("Found %d keys in %d: (%d hits, q=%d) %s\n",
					$maxlen, $len, $cachehits, $qlen, $keys);
				#printf("%4d: %s\n",$len,$keys);
				$minlen = $len;
			}
#			next;
#				return $minlen;
		}
		my @keys = split(//,$keys);
		my $se = join("\t",@nodes,join('',sort @keys));
		if (defined($seen{$se}) && $seen{$se} <= $len) {
			$cachehits++;
			next;
		}
		$seen{$se} = $len;
		
		next if ($len > 10000);
		
		for (my $n = 0; $n < scalar(@nodes); $n++) {
			my $node = $nodes[$n];
			my ($x,$y) = split(/,/,$node);
			
#			printf(STDERR "%-30s (%s)\n", $m, substr($b[$y],$x,1));

			my @paths = spanning_tree($node,$keys);
##			if (scalar(@paths) == 0) {
#				$high_pri++;
#				$prio->insert($m,$high_pri+1);
#				$qlen++;
#				next;
#			}
			foreach (@paths) {
				my ($target,$tl) = split(/;/);
				$tl += $len;
				my @n = @nodes;
				$n[$n] = $target;

				my $k = $pos2key{$target};
				my $pri = ($total_keys-length($keys))*4000+$tl;
				if ($pri > $high_pri) {
					$high_pri = $pri;
				}
				$prio->insert(join(";",$tl,$keys.$k,@n),$pri);
				$qlen++;
			}
		}
	}
	return $minlen;
}

parse(@inp); # Will set @START
if (scalar(@START) < 4) {
	my ($SX,$SY) = split(/,/,$START[0]);
	substr($inp[$SY-1],$SX-1,3) = "@#@";
	substr($inp[$SY],$SX-1,3) = "###";
	substr($inp[$SY+1],$SX-1,3) = "@#@";
	parse(@inp);
}
$part2 = p2(@inp);

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
