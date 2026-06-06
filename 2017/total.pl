#!perl -w 
use strict;

opendir(D,'.'); my @dirs = readdir(D); closedir(D);

my @times = ();
for (my $i=1; $i <= 25; $i++) {
	my $dir = "day".$i;
	chdir($dir);
	my $cmd = sprintf(q(target\release\%s.exe),$dir);
	printf(STDERR "%s\n", $cmd);
	my $res = `$cmd` if $i != 17;
	printf(STDERR "%s\n", $res);
	if ($res =~ /(Total time \d+.+s)$/) {
		push(@times,"$dir: $1");
	}
	chdir("..");
}

printf("%s\n", join("\n", @times));