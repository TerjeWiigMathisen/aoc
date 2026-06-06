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

my $DEBUG = 1;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my $d = shift;
if (defined($d)) {
	$DEBUG = $d;
}

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

sub run
{
	my @r = @_;
	$regs{a} = $r[0]; $regs{b} = $r[1]; $regs{c} = $r[2]; $regs{d} = $r[3]; 
	for (my $ip = 0; $ip < scalar(@inp); $ip++) {
		$_ = $inp[$ip];
		printf("%3d %-10s a=%d, b=%d, c=%d, d=%d\n",$ip, $_, $regs{a},$regs{b},$regs{c},$regs{d}) if ($DEBUG);
		if (/cpy (\S+) (\S+)/) {
			$regs{$2} = val($1) if defined($regs{$2});
		}
		elsif (/mul (\S+) (\S+)/) {
			die("mul error!") unless defined($regs{$2});
			$regs{$2} *= val($1);
		}
		elsif (/inc (\S+)/) {
			$regs{$1}++;
		}
		elsif (/dec (\S+)/) {
			$regs{$1}--;
		}
		elsif (/jnz (\S+) (\S+)/) {
			if (val($1)) {
				$ip = $ip + val($2)-1;
			}
		}
		elsif (/tgl (\S+)/) {
			my $target = $ip+val($1);
			next if ($target < 0 || $target >= scalar(@inp));
			if ($inp[$target] =~ /inc (\S+)/) {
				$inp[$target] = "dec $1";
			}
			elsif ($inp[$target] =~ /(dec|tgl) (\S+)/) {
				$inp[$target] = "inc $2";
			}
			elsif ($inp[$target] =~ /jnz (\S+) (\S+)/) {
				$inp[$target] = "cpy $1 $2";
			}
			elsif ($inp[$target] =~ /cpy (\S+) (\S+)/) {
				$inp[$target] = "jnz $1 $2";
			}
		}

	}
	return $regs{a};
}

my @prog = @inp;
$part1 = run(7,0,0,0);
@inp = @prog;
$part2 = run(12,0,0,0);

printf(STDERR "Total time = %f\n", time - $start);

printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
