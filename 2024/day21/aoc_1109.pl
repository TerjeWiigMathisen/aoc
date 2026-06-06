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

my %num = ();
sub numeric
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
	if ($bx == 0 && $sy == 3) {
		# y then x
		while ($sy > $by) {
			$p .= '^';
			$sy--;
		}
		while ($sx > $bx) {
			$p .= '<';
			$sx--;
		}
	}
	else {
		# First x then y
		while ($sx < $bx) {
			$p .= '>';
			$sx++;
		}
		while ($sx > $bx) {
			$p .= '<';
			$sx--;
		}
		while ($sy > $by) {
			$p .= '^';
			$sy--;
		}
		while ($sy < $by) {
			$p .= 'v';
			$sy++;
		}
	}
	$p .= 'A';
	$num{$key} = $p;
	return $p;
}

my %num1 = ();
sub numeric1
{
	my ($pos, $code) = @_;
	my $key = "$pos,$code";
	if (defined($num1{$key})) {
		return $num1{$key};
	}
	my ($sx,$sy) = split(/,/,$numpos{$pos});
	my ($bx,$by) = split(/,/,$numpos{$code});
	die("Unknown pos $pos") unless (defined($sy));
	die("Unknown code $code") unless (defined($by));
	
	my $p = '';
	if ($bx == 0 && $sy == 3) {
		# y then x
		while ($sy > $by) {
			$p .= '^';
			$sy--;
		}
		while ($sx > $bx) {
			$p .= '<';
			$sx--;
		}
	}
	else {
		# First y then x
		while ($sy > $by) {
			$p .= '^';
			$sy--;
		}
		while ($sy < $by) {
			$p .= 'v';
			$sy++;
		}
		while ($sx < $bx) {
			$p .= '>';
			$sx++;
		}
		while ($sx > $bx) {
			$p .= '<';
			$sx--;
		}
	}
	$p .= 'A';
	$num1{$key} = $p;
	return $p;
}

my %dirpos = (
	          '^','1,0','A','2,0',
	'<','0,1','v','1,1','>','2,1');

my %dir = ();
sub direction
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
	if ($bx == 0 && $sy == 0) {
		# y first
		while ($sy < $by) {
			$p .= 'v';
			$sy++;
		}
		while ($sx > $bx) {
			$p .= '<';
			$sx--;
		}
	}
	else {
		# x then y
		while ($sx < $bx) {
			$p .= '>';
			$sx++;
		}
		while ($sx > $bx) {
			$p .= '<';
			$sx--;
		}
		while ($sy > $by) {
			$p .= '^';
			$sy--;
		}
		while ($sy < $by) {
			$p .= 'v';
			$sy++;
		}
	}
	$p .= 'A';
	$dir{$key} = $p;
	return $p;
}

my %dir1 = ();
sub direction1
{
	my ($pos, $code) = @_;
	my $key = "$pos,$code";
	if (defined($dir1{$key})) {
		return $dir1{$key};
	}
	my ($sx,$sy) = split(/,/,$dirpos{$pos});
	my ($bx,$by) = split(/,/,$dirpos{$code});
	die("Unknown pos $pos") unless (defined($sy));
	die("Unknown code $code") unless (defined($by));
	
	my $p = '';
	if ($bx == 0 && $sy == 0) {
		# y first
		while ($sy < $by) {
			$p .= 'v';
			$sy++;
		}
		while ($sx > $bx) {
			$p .= '<';
			$sx--;
		}
	}
	else {
		while ($sy > $by) {
			$p .= '^';
			$sy--;
		}
		while ($sy < $by) {
			$p .= 'v';
			$sy++;
		}
		# x
		while ($sx < $bx) {
			$p .= '>';
			$sx++;
		}
		while ($sx > $bx) {
			$p .= '<';
			$sx--;
		}
	}
	$p .= 'A';
	$dir1{$key} = $p;
	return $p;
}

my @rpos = ('A','A','A','A'); # 3 robots + me

while (<>) {
	chomp;
	my $code = $_;
	printf("%s\n",$_);
	my $curr = $rpos[0];
	my $keys = '';
	my $keys1 = '';
	foreach (split(//)) {
		$keys .= numeric($curr,$_);
		$keys1 .= numeric1($curr,$_);
		$curr = $_;
	}
	$rpos[0] = $curr;
	die("Not A?") unless ($curr eq 'A');
	die("Diff len $keys, $keys1") if (length($keys) != length($keys1));
	my @keys = ($keys, $keys1);

	my @k = ();
	foreach(@keys) {
		printf("%s\n",$_);
		$curr = $rpos[1];
		$keys = '';
		$keys1 = '';
		foreach (split(//)) {
			$keys .= direction($curr,$_);
			$keys1 .= direction1($curr,$_);
			$curr = $_;
		}
		$rpos[1] = $curr;
		die("Not A?") unless ($curr eq 'A');
		push(@k,$keys,$keys1);
	}
	my $len = length($k[0]);
	@keys = ();
	foreach (@k) {
		my $l = length($_);
		if ($l != $len) {
			printf("Different lengths:\n%s\n", join("\n",@k));
			next if ($l > $len);
			@keys = ();
			$len = $l;
		}
		push(@keys,$_);
	}

	@k = ();
	foreach(@keys) {
		printf("%s\n",$_);
		$curr = $rpos[2];
		$keys = '';
		$keys1 = '';
		foreach (split(//)) {
			$keys .= direction($curr,$_);
			$keys1 .= direction1($curr,$_);
			$curr = $_;
		}
		$rpos[2] = $curr;
		die("Not A?") unless ($curr eq 'A');
		push(@k,$keys,$keys1);
	}

	$len = length($k[0]);
	@keys = ();
	foreach (@k) {
		my $l = length($_);
		if ($l != $len) {
			printf("Different lengths:\n%s\n", join("\n",@keys,$_));
			next if ($l > $len);
			@keys = ();
			$len = $l;
		}
		push(@keys,$_);
	}

	my $p = $len*substr($code,0,-1);
	printf("%s\n%d x %d = %d\n",$keys[0], $len, substr($code,0,-1), $p);
	
	$part1 += $p;
}

my $used = time - $start;
printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
