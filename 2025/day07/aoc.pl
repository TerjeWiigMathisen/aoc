#!perl -w
use strict;
use Time::HiRes qw (time);
use English;
use warnings;
no warnings 'recursion';

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my $START;
my %tach = ();
my $first = 1;
while (<>) {
	chomp;
	push(@lines, $_);
	if ($first) {
		my $s = index($_,"S");
		$tach{$s}++;
		$first = 0;
		$START = $s;
		#$part2 = 1;
	}
	else {
		my %ntach = ();
		my $line = $_;
#		printf("%s\n",join(',',keys %tach));
		my $splits = 0;
		foreach (keys %tach) {
			if (substr($line,$_,1) eq '^') {
				my ($p,$n) = ($_-1, $_+1);
				$ntach{$p}++;
				$ntach{$n}++;
				$splits++;
			}
			else {
				$ntach{$_}++;
			}
		}
		%tach = %ntach;
		$part1 += $splits;
		#$part2 *= $splits if ($splits);
	}
}

#part2

my %cache = ();
my $hits = 0;
sub ways
{
	my ($line, $pos) = @_;
	my $key = "$line;$pos";
	if (defined($cache{$key})) {
		$hits++;
		return $cache{$key};
	}
	return 1 if ($line >= scalar(@lines));

	if (substr($lines[$line],$pos,1) eq '^') {
		my $ways = ways($line+1,$pos-1)+ways($line+1,$pos+1);
		$cache{$key} = $ways;
		return $ways;
	}
	my $ways = ways($line+1,$pos);
	$cache{$key} = $ways;
	return $ways;
}

$part2 = ways(1,$START);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
printf("%d cache entries, %d hits\n",scalar(keys %cache), $hits);