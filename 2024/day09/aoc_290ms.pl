#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

while (<>) {
	chomp;
	push(@lines, $_);
}
my @blocks = split(//,$lines[0]);

my $id = 0;
if (scalar(@blocks) & 1) {
	push(@blocks,0); # Tail free space
}
my @map = ();
$id = 0;
for (my $i = 0; $i < scalar(@blocks); $i+=2) {
	my $use = $blocks[$i];
	for (my $u = 0; $u < $use; $u++) {
		push(@map,$id);
	}
	$id++;
	my $free = $blocks[$i+1];
	for (my $u = 0; $u < $free; $u++) {
		push(@map,-1);
	}
}
#printf("%s\n",join("",@map));
my @p2map = @map;

for (my $i = 0; $i<scalar(@map);$i++) {
	if ($map[$i] == -1) {
		while (scalar(@map) > $i && $map[-1] == -1) {
			pop(@map);
		}
		if ($i+1 >= scalar(@map)) {
			last;
		}
		$map[$i] = pop(@map);
	}
}
#printf("%s\n",join("",@map));

for (my $i = 0; $i < scalar(@map); $i++) {
	$part1 += $i*$map[$i];
}

my $used;

#$used = time - $start;
#printf("%s\n", $part1);
#printf("Used %5.3fms\n",$used*1000);

# part2
@map = @p2map;

my @files = ();
my @free = ();
my $pos = 0;
$id = 0;
for (my $i = 1; $i < scalar(@blocks); $i+=2) {
	my $len = $blocks[$i-1];
	push(@files,$id,$len,$pos);
	$id++;
	$pos += $len;
	my $flen = $blocks[$i];
	if ($flen) {
		push(@free,$flen,$pos);
		$pos += $flen;
	}
}

#printf("%s\n",join("",@map));
#printf("Free: %s\n",join("",@free));
#printf("Used: %s\n",join("",@files));

for (my $i = scalar(@files)-3; $i > 0; $i-=3) {
	my $id = $files[$i];
	my $len = $files[$i+1];
	my $pos = $files[$i+2];
#	printf("%s\n",join("",@map));
#	printf("Free: %s\n",join(" ",@free));
#	printf("Used: %s\n",join(" ",@files));
#	printf("Trying to move file with id %d, len %d, pos %d\n",$id, $len, $pos);
	
	for (my $f = 0; $f < scalar(@free); $f+=2) {
		my $flen = $free[$f];
		my $fpos = $free[$f+1];
		last if ($fpos >= $pos);

		if ($flen >= $len) {
			$free[$f] -= $len;
			$free[$f+1] += $len;
			$files[$i+2] = $fpos;
			#printf("Found slot at pos %d\n",$fpos);
			do {
				$map[$fpos] = $id;
				$fpos++;
				$len--;
				$map[$pos+$len] = -1;
			} while ($len);
			
			if ($free[$f] == 0) {
				# Remove it from the free list!
				splice(@free,$f,2);
			}
			last;
		}
	}
}

#printf("%s\n",join(" ",@map));

for (my $i = 0; $i < scalar(@map); $i++) {
	$part2 += $i*$map[$i] if ($map[$i] > 0);
}

$used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);

my $p2 = 0;
for (my $f = 0; $f < scalar(@files); $f+=3) {
	my $id = $files[$f];
	my $len = $files[$f+1];
	my $pos = $files[$f+2];
#	printf("ID/len/pos: %d,%d,%d\n", $id, $len,$pos);
	for (my $p = $pos; $p < $pos+$len;$p++) {
		$p2 += $id * $p;
	}
}
printf("p2 = %d\n", $p2);
