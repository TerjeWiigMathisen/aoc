#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
use List::PriorityQueue;

#use bigint;

#use JSON::Parse;
no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $part1;
my $part2;

my %regs = ('a' => 0, 'b' => 0, 'c' => 0, 'd' => 0);

sub val
{
	my ($r) = @_;
	if (index('abcd',$r) >= 0) {
		return $regs{$r};
	}
	return $r;
}

my @output = ();

sub run
{
	my @r = @_;
	@output = ();
	$regs{a} = $r[0]; $regs{b} = $r[1]; $regs{c} = $r[2]; $regs{d} = $r[3]; 
	for (my $ip = 0; $ip < scalar(@inp); $ip++) {
		$_ = $inp[$ip];
	#	printf("%3d %-10s a=%d, b=%d, c=%d, d=%d\n",$ip, $_, $regs{a},$regs{b},$regs{c},$regs{d});
		if (/cpy (\S+) (\S+)/) {
			$regs{$2} = val($1);
		}
		elsif (/inc (\S+)/) {
			$regs{$1}++;
		}
		elsif (/dec (\S+)/) {
			$regs{$1}--;
		}
		elsif (/jnz (\S+) (\S+)/) {
			if (val($1)) {
				$ip += $2-1;
			}
		}
		elsif (/out (\S+)/) {
			push(@output, val($1));
			if (scalar(@output) > 1) {
				if (($output[-1] ^ $output[-2]) != 1) {
					return 0;
				}
			}
			if (scalar(@output) == 6) {
				printf("%3d: %s\r", $r[0], join(",",@output));
			}
			if (scalar(@output) >= 15) {
				printf("%3d: %s\n", $r[0], join(",",@output));
				return $r[0];
			}
		}
	}
	return $regs{a};
}

for (my $a = 1; $a < 10000; $a++) {
	$part1 = run($a,0,0,0);
}
$part2 = run(0,0,1,0);

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
