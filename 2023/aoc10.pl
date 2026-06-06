#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);

my $YMAX=scalar(@inp);
my $XMAX = length($inp[0]);

my $guard = '.' x ($XMAX+2);

my $SX;
my $SY;
for (my $y=0;$y<$YMAX;$y++) {
	$_='.'.$inp[$y].'.';
	if (index($_,'S') >= 0) {
		$SX = index($_,'S');
		$SY = $y+1;
	}
	$inp[$y] = $_;
}

@inp = ($guard,@inp,$guard);

sub walk
{
	my ($sx,$sy,$dx,$dy,$maxs) = @_;
	$maxs = 1e9 unless (defined($maxs) && $maxs > 0);

	my ($x,$y) = ($sx,$sy);
	my $steps = 0;
	my %path = ();
	while ($steps < $maxs) {
		$steps++;
		$x += $dx; $y += $dy;
		$path{"$x;$y"}++;
		my $c = substr($inp[$y],$x,1);
		if ($c eq 'S') { return ($steps, \%path); }
		if ($c eq '.') { return -$steps; }

		if ($c eq '|') {
			return -$steps if ($dy == 0);
		}
		
		elsif ($c eq '-') {
			return -$steps if ($dx == 0);
		}
		
		elsif ($c eq 'F') {
			return -1-$steps unless (($dy == 0 && $dx== -1) || ($dy == -1 && $dx== 0)) ;
			if ($dy==0) {
				$dx=0;
				$dy=1;
			}
			else {
				$dx=1;
				$dy=0;
			}
		}
		elsif ($c eq 'J') {
			return -1-$steps unless (($dy == 0 && $dx== 1) || ($dy == 1 && $dx== 0)) ;
			if ($dy==0) {
				$dx=0;
				$dy=-1;
			}
			else {
				$dx=-1;
				$dy=0;
			}
		}
		elsif ($c eq 'L') {
			return -1-$steps unless (($dy == 0 && $dx== -1) || ($dy == 1 && $dx== 0)) ;
			if ($dy==0) {
				$dx=0;
				$dy=-1;
			}
			else {
				$dx=1;
				$dy=0;
			}
		}
		elsif ($c eq '7') {
			return -1-$steps unless (($dy == 0 && $dx== 1) || ($dy == -1 && $dx== 0)) ;
			if ($dy==0) {
				$dx=0;
				$dy=1;
			}
			else {
				$dx=-1;
				$dy=0;
			}
		}
	}
}

my ($dist_up,$up) = walk($SX,$SY,0,-1);
my ($dist_rt,$rt) = walk($SX,$SY,1,0);
my ($dist_lt,$lt) = walk($SX,$SY,-1,0);
my ($dist_dn,$dn) = walk($SX,$SY,0,1);

my $S = '.';
my $path;
if ($dist_up > 0) {
	$path = $up;
	if ($dist_lt > 0) {
		$S = 'J';
	}
	elsif ($dist_rt > 0) {
		$S = 'L';
	}
	elsif ($dist_dn > 0) {
		$S = '|';
	}
	else {die;}
}
elsif ($dist_dn > 0) {
	$path = $dn;
	if ($dist_lt > 0) {
		$S = '7';
	}
	elsif ($dist_rt > 0) {
		$S = 'F';
	}
	else {die;}
}
elsif ($dist_lt > 0) {
	$path = $lt;
	if ($dist_rt > 0) {
		$S = '-';
	}
	else {die;}
}

substr($inp[$SY],$SX,1) = $S;

my @d = sort {$b<=>$a} ($dist_up,$dist_dn,$dist_lt,$dist_rt);
$part1 = $d[0]>>1;

# Fill non-path cells with '.'
for (my $y = 1; $y <= $YMAX; $y++) {
	for (my $x = 1; $x <= $XMAX; $x++) {
		if (!defined($path->{"$x;$y"})) {
			substr($inp[$y],$x,1) = '.';
		}
	}
}
printf("%s\n\n",join("\n",@inp));
# Count transitions

my $v = join("\n",@inp);
$v =~ tr/.FJL7\-\|/0211221/;
printf("%s\n\n",$v);

my @v = split(/\n/,$v);
my %inner = ();

for (my $y = 1; $y <= $YMAX; $y++) {
	my $i = 0;
	for (my $x = 1; $x <= $XMAX; $x++) {
		my $c = substr($v[$y],$x,1);
		if ($c == 0 && ($i & 1)) {
			$inner{"$x;$y"} = 'i';
			$part2++;
#			substr($v[$y],$x,1) = 'i';
		}
		$i += $c;
	}
}
printf("%s\n\n",join("\n",@v));

#my $v = join("\n",@inp);
#$v =~ tr/.FJL7\-\|/0366112/;
#my @v = split(/\n/,$v);

#for (my $x = 1; $x <= $XMAX; $x++) {
#	my $i = 0;#
#	for (my $y = 1; $y <= $YMAX; $y++) {
#		my $c = substr($v[$y],$x,1);
#		if ($c == 0 && ($i & 1)) {
#			if (defined($inner{"$x;$y"})) {
#				$part2++;
#				substr($v[$y],$x,1) = 'I';
#			}
#		}
#		$i += $c;
#	}
#}
#printf("%s\n\n",join("\n",@v));

my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.6fs\n", $used);
