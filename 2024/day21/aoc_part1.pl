#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use List::PriorityQueue;


my $part1 = 0;
my $part2 = 0;

my $start = time;

my %numpos = (
    '7','0,0','8','1,0','9','2,0',
	'4','0,1','5','1,1','6','2,1',
	'1','0,2','2','1,2','3','2,2',
	          '0','1,3','A','2,3');

my %pos2num = ();
foreach (keys %numpos) {
	$pos2num{$numpos{$_}} = $_;
}

my %num = ();
sub num
{
	my ($pos, $code) = @_;
	my $key = "$pos,$code";
	if (defined($num{$key})) {
		return $num{$key};
	}
	my ($sx,$sy) = split(/,/,$numpos{$pos});
	my ($bx,$by) = split(/,/,$numpos{$code});
	die("Unknown pos $pos") unless (defined($sy));
	die("Unknown code $code") unless (defined($by));
	
	my $p = '';
	my $xx = '';
	my $yy = '';
	my ($x,$y) = ($sx,$sy);
	while ($x < $bx) { $xx .= '>'; $x++; }
	while ($x > $bx) { $xx .= '<'; $x--; }
	while ($y > $by) { $yy .= '^'; $y--; }
	while ($y < $by) { $yy .= 'v'; $y++; }
	if ($bx == 0 && $sy == 3) {
		# y then x
		$p = $yy.$xx.'A';
	}
	elsif ($sx == 0 && $by == 3) {
		$p = $xx.$yy.'A';
	}
	elsif ($xx eq '' | $yy eq '') {
		$p = $yy.$xx.'A';
	}
	else  {
		$p = join(";", $xx.$yy.'A', $yy.$xx.'A');
	}
	$num{$key} = $p;
	return $p;
}

my %dirpos = (
	          '^','1,0','A','2,0',
	'<','0,1','v','1,1','>','2,1');

my %pos2dir = ();
foreach (keys %dirpos) {
	$pos2dir{$dirpos{$_}} = $_;
}


my %dir = ();
sub dir
{
	my ($pos, $code) = @_;
	my $key = "$pos,$code";
	if (defined($dir{$key})) {
		return $dir{$key};
	}
	my ($sx,$sy) = split(/,/,$dirpos{$pos});
	my ($bx,$by) = split(/,/,$dirpos{$code});
	die("Unknown pos $pos") unless (defined($sy));
	die("Unknown code $code") unless (defined($by));
	
	my $p = '';
	my $xx = '';
	my $yy = '';
	my ($x,$y) = ($sx,$sy);
	while ($x < $bx) { $xx .= '>'; $x++; }
	while ($x > $bx) { $xx .= '<'; $x--; }
	while ($y > $by) { $yy .= '^'; $y--; }
	while ($y < $by) { $yy .= 'v'; $y++; }
	if ($bx == 0) {
		# y then x
		$p = $yy.$xx.'A';
	}
	elsif ($sx == 0) {
		# x
		$p = $xx.$yy.'A';
	}
	elsif ($xx eq '' | $yy eq '') {
		$p = $yy.$xx.'A';
	}
	else  {
		$p = join(";", $xx.$yy.'A'); #, $yy.$xx.'A');
	}
	$dir{$key} = $p;
	return $p;
}

my @rpos = ('A','A','A','A'); # 3 robots + me

sub simulate_num
{
	my ($keys) = @_;
	my $press = '';
	my ($x,$y) = split(/,/,$numpos{'A'});
	foreach (split(//,$keys)) {
		if ($_ eq '<') { $x--; }
		elsif ($_ eq '^') { $y--; }
		elsif ($_ eq '>') {$x++; }
		elsif ($_ eq 'v') {$y++; }
		elsif ($_ eq 'A') { 
			my $k = $pos2num{"$x,$y"};
			#print("%s",$k);
			$press .= $k;
		}
		die($keys) if ($x == 0 && $y == 3);
	}
	#printf("$press\n");
	return $press;
}

sub simulate_dir
{
	my ($keys) = @_;
	my $press = '';
	my ($x,$y) = split(/,/,$dirpos{'A'});
	my $i = 0;
	foreach (split(//,$keys)) {
		if ($_ eq '<') { $x--; }
		elsif ($_ eq '^') { $y--; }
		elsif ($_ eq '>') {$x++; }
		elsif ($_ eq 'v') {$y++; }
		elsif ($_ eq 'A') { 
			my $k = $pos2dir{"$x,$y"};
			#print("%s",$k);
			$press .= $k;
		}
		die("dir keys[$i]: $keys") if ($x == 0 && $y == 0);
		$i++;
	}
	#printf("$press\n");
	return $press;
}

sub shortest_dir
{
	my ($keys, $level) = @_;
	my $last = 'A';
	my @keys = ();
	my $perms = 1;
	my @perms = ();
	foreach (split(//,$keys)) {
		my $curr = $_;
		my $n = dir($last,$curr);
		push(@keys,$n);
		my @p = split(/;/,$n);
		if (scalar(@perms)) {
			my @comb = ();
			foreach (@perms) {
				my $pre = $_;
				foreach (@p) {
					push(@comb, $pre.$_);
				}
			}
			@perms = @comb;
		}
		else {
			@perms = @p;
		}
		$last = $curr;
	}
	#printf("%s: found %d seqs\n", $keys, scalar(@perms));
	my $min = length($perms[0]);
	if ($level < 2) {
		#$min = 1e38;
		$min = undef;
		foreach (@perms) {
			my $len = shortest_dir($_, $level +1);
			$min = $len unless(defined($min));
			if ($len != $min) {
				printf("%s\n%s\n",$perms[0],$_);
				printf("Different length: $len, $min\n");
			}
			if ($len < $min) {
				#printf("Different length: $len, $min\n");
				#printf("%s\n%s\n",$perms[0],$_);
				$min = $len;
			}
		}
		return $min;
	}
	else {
		#$min = 1e38;
		foreach (@perms) {
			my $len = length($_);
			if ($len == 64) {
				my $sim = simulate_dir($_);
				if ($sim ne $keys) {
					die;
				}
				$sim = simulate_dir($sim);
				#printf("$sim\n");
				$sim = simulate_num($sim);
				#printf("$sim\n");
				#die() if ($sim ne '179A');
			}
			if ($len != $min) {
				printf("%s\n%s\n",$perms[0],$_);
				printf("Different length: $len, $min\n");
			}
			if ($len < $min) {
				#printf("Different length: $len, $min\n");
				#printf("%s\n%s\n",$perms[0],$_);
				$min = $len;
			}
		}
		return $min;
	}
}
	
sub shortest_num
{
	my ($keys) = @_;
	my $last = 'A';
	my @keys = ();
	my $perms = 1;
	my @perms = ();
	foreach (split(//,$keys)) {
		my $curr = $_;
		my $n = num($last,$curr);
		push(@keys,$n);
		my @p = split(/;/,$n);
		if (scalar(@perms)) {
			my @comb = ();
			foreach (@perms) {
				my $pre = $_;
				foreach (@p) {
					push(@comb, $pre.$_);
				}
			}
			@perms = @comb;
		}
		else {
			@perms = @p;
		}
		$last = $curr;
	}
	printf("%s: %d %s keys\n", $keys, scalar(@perms), join(" ",@perms));
	foreach (@perms) {
		my $sim = simulate_num($_);
		if ($sim ne $keys) {
			die;
		}
	}
	my $min = 1e38;
	foreach (@perms) {
		my $len = shortest_dir($_, 1);
		if ($len < $min) {
			$min = $len;
		}
	}
	return $min;
}


while (<>) {
	chomp;
	my $p = shortest_num($_);
	my $n = substr($_,0,-1);
	printf("%s: %d x %d = %d\n", $_, $p, $n, $p*$n);
	$part1 += $p * $n;
}

my $used = time - $start;
printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
