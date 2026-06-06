#!perl -w

use strict;
use Time::HiRes qw (time);
use warnings;
use English;
use Data::Dumper;
use Algorithm::Permute qw (permute);
use Digest::MD5;

#use bigint;

#use JSON::Parse;
#no warnings 'recursion';

my $start = time;

my $DEBUG = 0;

#my $fname = shift;
#open(F ,'<',$fname);
#my @inp = (<F>);
#chomp(@inp);

my $part1 = '';
my $part2 = '________';

my $salt = "uqwqemis";

my $p2found = 0;



for (my $i = 0; $p2found < 8; $i++) {
	my $hash = Digest::MD5::md5_hex(sprintf("%s%d",$salt, $i));
	if (substr($hash,0,5) eq "00000") {
		my $p2pos = substr($hash,5,1);
		$part1 .= $p2pos;
		next if ($p2pos ge '8');
		next if (substr($part2,$p2pos,1) ne '_');
		substr($part2,$p2pos,1) = substr($hash,6,1);
		printf(STDERR "%s\r",$part2);
		$p2found++;
	}
}

		
printf(STDERR "Total time = %f\n", time - $start);
printf("Part1: %s\n", $part1);
printf("Part2: %s\n", $part2);
