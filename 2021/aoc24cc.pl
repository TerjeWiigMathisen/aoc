#!perl -w

use strict;

my $fname = shift;
open(F ,'<',$fname);
my @inp = (<F>);
chomp(@inp);

my @progs = split(/inp w\n/, join("\n",@inp));

for (my $p = 1; $p <= 14; $p++) {
	compile($p, $progs[$p]);
}

sub compile
{
	my ($pnr, $prog) = @_;
	
	my @inp = split(/\n/, $prog);
	my @ops = ();
	foreach (@inp) {
		next if (/#/);
		my ($op, $a, $b) = split;
		next unless defined ($a);
		
		if ($op eq 'add') {
			push(@ops, sprintf("%s+=%s;",$a, $b));
		}
		elsif ($op eq 'mul') {
			push(@ops, $b eq '0' ? sprintf("%s=0;",$a) : 
					sprintf("%s*=%s;",$a, $b));
		}
		elsif ($op eq 'div') {
			push(@ops, sprintf("%s/=%s;", $a, $b));
		}
		elsif ($op eq 'mod') {
			push(@ops, sprintf("%s%%=%s;",$a, $b));
		}
		elsif ($op eq 'eql') {
			push(@ops, sprintf("%s=%s==%s;",$a, $a, $b));
		}
	}
	printf("#define f%d(iw) {w=iw;%s}\n", $pnr, join(" ",@ops));
	printf("u64 func%d(char cw, rgs &r) {",$pnr);
	printf(" u64 w, x=r.x, y=r.y, z=r.z;");
	#printf(" assert(w >= 1 && w <= 9);"); 
	printf(" f%d(cw);", $pnr);
	printf(" r.z = z; return x;}\n\n");
}

