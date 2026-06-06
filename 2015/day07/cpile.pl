#!perl -d 
use strict;
use warnings;

my %seen = ();

my %op = ("AND" => "&", "OR" => "|", "XOR" => "^", "LSHIFT" => "<<", "RSHIFT" => ">>");

my %resv = ("if" => 1, "in" => 1, "as" => 1, "do" => 1, "fn" => 1);

my @lines = (<>);

sub ready
{
	my ($v) = @_;
	return $v =~ /^\d+$/ || $seen{$v};
}

sub p 
{
	my ($v) = @_;
	return $v =~ /^\d+$/ ? $v : "_".$v;
}

while (@lines) {
	my @l = ();
	foreach (@lines) {
		chomp;
		my @p = split;
		my $tgt = $p[-1];
		if (scalar(@p) == 5) {
			if (ready($p[0]) && ready($p[2])) {
				printf("\tlet _%s:u16 = %s %s %s;\n", $tgt, p($p[0]), $op{$p[1]}, p($p[2]));
				$seen{$tgt}++;
			}
			else {
				push(@l,$_);
			}
		}
		elsif (scalar(@p) == 4) {
			#elsif ($st =~ /^NOT\s+(\w+)$/) {
			if (ready($p[1])) {
				printf("\tlet _%s:u16 = !%s;\n", $tgt, p($p[1]));
				$seen{$tgt}++;
			}
			else {
				push(@l,$_);
			}
		}
		elsif (scalar(@p) == 3) {
#		elsif ($p[0] =~ /^\d+$/) {
			if (ready($p[0])) {
				printf("\tlet _%s:u16 = %s;\n", $tgt, p($p[0]));
				$seen{$tgt}++;
			}
			else {
				push(@l,$_);
			}
		}
	}
	@lines = @l;
}
