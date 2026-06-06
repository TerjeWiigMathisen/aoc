#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

my $fname = shift;
open(F,$fname) || die("Open");
my @code = (<F>);
close(F);

@code = split(/,/, $code[0]);
my $ip = 0;

my $part1 = 0;
my $part2 = 0;

my @inputs;

my @outputs;

sub runcode
{
	my ($ip, @code) = @_;
	while (1) {
		# printf(STDERR "%d: %s\n", $ip, join(',',@code));
		my $op = '000'.$code[$ip];
		my $mode = substr($op,0,length($op)-2);
		$op = substr($op,-2,2);
		last if ($op == 99);

		my $s1 = $code[$ip+1];
		my $s2 = $code[$ip+2];
		my $d = $code[$ip+3];
		
#		printf("IP=%3d, OP=%2d, MODE=%02d, S1=%d, S2=%d, D=%d\n", $ip, $op, $mode, $s1, $s2, $d);
		if ($op == 1) {
			$code[$d] = (substr($mode,-1,1) == 0 ? $code[$s1] : $s1) + 
						(substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			$ip += 4;
		}
		elsif ($op == 2) {
			$code[$d] = (substr($mode,-1,1) == 0 ? $code[$s1] : $s1) * 
						(substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			$ip += 4;
		}
		elsif ($op == 3) {
			$code[$s1] = shift @inputs;
			$ip += 2;
		}
		elsif ($op == 4) {
			push(@outputs, $code[$s1]);
#			printf("Output: %d\n", $outputs[-1]);
			$ip += 2;
			return($code[$s1],$ip,@code);
		}
		elsif ($op == 5) { # JT
			if ((substr($mode,-1,1) == 0 ? $code[$s1] : $s1)) {
				$ip = (substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			}
			else {
				$ip += 3;
			}
		}
		elsif ($op == 6) { # JF
			if ((substr($mode,-1,1) == 0 ? $code[$s1] : $s1)==0) {
				$ip = (substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			}
			else {
				$ip += 3;
			}
		}
		elsif ($op == 7) { # LT
			$code[$d] = (substr($mode,-1,1) == 0 ? $code[$s1] : $s1) < 
						(substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			$ip += 4;
		}
		elsif ($op == 8) { # EQ
			$code[$d] = (substr($mode,-1,1) == 0 ? $code[$s1] : $s1) ==
						(substr($mode,-2,1) == 0 ? $code[$s2] : $s2);
			$ip += 4;
		}
		else {
			die("Bad opcode $op at IP=$ip");
		}
	}
	return ($outputs[-1],$ip);
}

my @acode =  @code;
my @bcode =  @code;
my @ccode =  @code;
my @dcode =  @code;
my @ecode = @code;

my ($aip,$bip,$cip,$dip,$eip) = (0,0,0,0,0);

@outputs = (0);

my @phases = (0,1,2,3,4);
#Algorithm::Permute::permute { print "@phases\n"; } @phases;
#die;

@phases = (5,6,7,8,9);
my $p = Algorithm::Permute->new([@phases]);
while (@phases = $p->next()) {
#	printf(STDERR "Testing %s\n", join(',',@phases));
	($aip,$bip,$cip,$dip,$eip) = (0,0,0,0,0);
	@acode = @code; @bcode = @code; @ccode = @code; @dcode = @code; @ecode = @code;
	@inputs = ($phases[0],0); @outputs = ();
	my $out;
	($out, $aip,@acode) = runcode($aip,@acode);
	@inputs = ($phases[1],$out); @outputs = ();
	($out, $bip,@bcode) = runcode($bip,@bcode);
	@inputs = ($phases[2],$out); @outputs = ();
	($out, $cip,@ccode) = runcode($cip,@ccode);
	@inputs = ($phases[3],$out); @outputs = ();
	($out, $dip,@dcode) = runcode($dip,@dcode);
	@inputs = ($phases[4],$out); @outputs = ();
	($out, $eip,@ecode) = runcode($eip,@ecode);
	my $lastout;
	while (1) {
		@inputs = ($out); @outputs = ();
		($out, $aip,@acode) = runcode($aip,@acode);
		@inputs = ($out); @outputs = ();
		($out, $bip,@bcode) = runcode($bip,@bcode);
		@inputs = ($out); @outputs = ();
		($out, $cip,@ccode) = runcode($cip,@ccode);
		@inputs = ($out); @outputs = ();
		($out, $dip,@dcode) = runcode($dip,@dcode);
		@inputs = ($out); @outputs = ();
		($out, $eip,@ecode) = runcode($eip,@ecode);
		if (scalar(@ecode)) {
			$lastout = $out;
		}
		else {
			last;
		}
	}
	if ($lastout > $part2) {
		$part2 = $lastout;
		printf("Found new max: $part2 from %s\n",join(',',@phases));
	}
} 

printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
