#!perl -w
use strict;
use Time::HiRes qw(time);

my $t0 = time;
my $part1 = 0;
my $part2 = 0;

my @inp = (<>); chomp(@inp);
#printf("Input:\n%s\n\n", join("\n",@inp));

# Helper lookup tables:
my @dx = (1,0,-1,0); # rt,dn,lt,up
my @dy = (0,1,0,-1);
my @sf = (3,2,1,0); # /
my @sb = (1,0,3,2); # \

sub energize
{
	my (@b) = @_;
	my %en = (); # beams(d,x,y)
	my %e = ();  # energized (x,y)
	my @beams = splice(@b,0,3); # initial beam
	while (scalar(@beams)) {
		my ($d,$x,$y) = splice(@beams,0,3);
		next if ($x < 0 || $y < 0 || $x >= length($b[0]) || $y >= scalar(@b));

		my $k = "$d;$x;$y";
		next if (defined($en{$k}));
		$en{$k}++;
		$e{"$x;$y"}++;

		my $c = substr($b[$y],$x,1);
		if ($c eq '.') {
			push(@beams,$d,$x+$dx[$d],$y+$dy[$d]);
		}
		elsif ($c eq '/') {
			$d = $sf[$d];
			push(@beams,$d,$x+$dx[$d],$y+$dy[$d]);
		}
		elsif ($c eq '\\') {
			$d = $sb[$d];
			push(@beams,$d,$x+$dx[$d],$y+$dy[$d]);
		}
		elsif ($c eq '|') {
			if ($d & 1) { # up/dn
				push(@beams,$d,$x+$dx[$d],$y+$dy[$d]); # Pass through
			}
			else { #split!
				$d = $sb[$d];
				push(@beams,$d,$x+$dx[$d],$y+$dy[$d]);
				$d ^= 2; # opposite dir
				push(@beams,$d,$x+$dx[$d],$y+$dy[$d]);
			}
		}
		elsif ($c eq '-') {
			if (($d & 1)==0) { # lt/rt
				push(@beams,$d,$x+$dx[$d],$y+$dy[$d]); # Pass through
			}
			else { #split!
				$d = $sb[$d];
				push(@beams,$d,$x+$dx[$d],$y+$dy[$d]);
				$d ^= 2;
				push(@beams,$d,$x+$dx[$d],$y+$dy[$d]);
			}
		}
	}
	return scalar(keys %e);
}

sub maxenergy
{
	my (@b) = @_;
	my $max = 0;
	# Top & bottom;
	my $e;
	for (my $x = 0; $x < length($b[0]); $x++) {
		$e = energize(1,$x,0,@b);
		$max = $e if ($e > $max);
		$e = energize(3,$x,scalar(@b)-1,@b);
		$max = $e if ($e > $max);
	}
	# left & right
	for (my $y = 0; $y < scalar(@b); $y++) {
		$e = energize(0,0,$y,@b);
		$max = $e if ($e > $max);
		$e = energize(2,length($b[0])-1,$y,@b);
		$max = $e if ($e > $max);
	}
	return $max;
}

$part1 = energize(0,0,0,@inp);            
$part2 = maxenergy(@inp);
	
	
my $used = time-$t0;

printf("%s\n%s\n", $part1, $part2);
printf("Used %1.3fs\n", $used);
