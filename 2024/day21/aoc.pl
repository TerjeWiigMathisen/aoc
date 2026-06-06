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
		return split(/;/,$num{$key});
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
	return split(/;/,$p);
}

my %numlen = ();

foreach (split(//,"0123456789A")) {
	my $pre = $_;
	foreach (split(//,"0123456789A")) {
		my $cur = $_;
		my $a = num($pre,$cur);
		#$numlen{$pre.$cur} = length($a);
	}
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
		return split(/;/,$dir{$key});
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
		$p = join(";", $xx.$yy.'A', $yy.$xx.'A');
	}
	$dir{$key} = $p;
	return split(/;/,$p);;
}

my %dirlen = ();

foreach (split(//,"<^>vA")) {
	my $pre = $_;
	foreach (split(//,"<^>vA")) {
		my $cur = $_;
		my $a = dir($pre,$cur);
		#$dirlen{$pre.$cur} = length($a);
	}
}

my %diruse = ();

sub search
{
	my ($num, $depth) = @_;
	my $prev = 'A';
	my $len = 0;
	foreach (split(//,$num)) {
		my @perm = num($prev,$_);
		my $min = 1e38;
		foreach (@perm) {
			my $d = searchdir($_,$depth);
			if ($d < $min) { $min = $d; }
		}
		$len += $min;
		$prev = $_;
	}
	return $len;
}

sub searchdir
{
	my ($dir, $level) = @_;
	my $key = "$dir,$level";
	if (defined($dirlen{$key})) {
		$diruse{$key}++;
		return $dirlen{$key};
	}
	if ($level == 0) {
		$dirlen{$key} = length($dir);
		return length($dir);
	}
	my $prev = 'A';
	my $len = 0;
	foreach (split(//,$dir)) {
		my @perm = dir($prev,$_);
		my $min = 1e38;
		foreach (@perm) {
			my $d = searchdir($_, $level-1);
			if ($d < $min) { $min = $d; }
		}
		$len += $min;
		$prev = $_;
	}
	$dirlen{$key} = $len;
	return $len;
}

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
			$press .= $k;
		}
		die($keys) if ($x == 0 && $y == 3);
	}
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
			$press .= $k;
		}
		die("dir keys[$i]: $keys") if ($x == 0 && $y == 0);
		$i++;
	}
	return $press;
}


while (<>) {
	chomp;
	my $p = search($_,2);
	my $n = substr($_,0,-1);
	#printf("%s: %d x %d = %d\n", $_, $p, $n, $p*$n);
	$part1 += $p * $n;

	$p = search($_,25);
	#printf("%s: %d x %d = %d\n", $_, $p, $n, $p*$n);
	$part2 += $p * $n;
}

my $used = time - $start;
printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

printf("Total %d entries in the dirlen cache\n",scalar(keys %dirlen));
printf("Total %d entries in the diruse cache\n",scalar(keys %diruse));
foreach (sort {$diruse{$b} <=> $diruse{$a}} keys %diruse) {
	#printf("%d %s\n",$diruse{$_}, $_);
}
