#!perl -w
use strict;
use Time::HiRes qw (time);
use English;

my $part1 = 0;
my $part2 = 0;

my $start = time;

my @lines = ();

my ($x0,$y0,$x1,$y1) = (1e38,1e38,-1e38,-1e38);

while (<>) {
	chomp;
	push(@lines, $_);
	my ($x,$y) = split(/,/);
	$x0 = $x if ($x < $x0);
	$x1 = $x if ($x > $x1);
	$y0 = $y if ($y < $y0);
	$y1 = $y if ($y > $y1);
}

#part1
#BL to TR
my @offs = ();
for (my $i = 0; $i < scalar(@lines); $i++) {
	my ($x,$y) = split(/,/, $lines[$i]);
	push(@offs, sprintf("%14d,%d",$x+$y1-$y, $i));
}
@offs = sort(@offs);

sub fmax
{
	my $max = 0;
	my $len = scalar(@offs)>>1;
	for (my $l = 0; $l < $len; $l++) {
		my ($a,$b) = split(/,/,$lines[(split(/,/,$offs[$l]))[1]]);
		for (my $r = scalar(@offs)-1; $r > scalar(@offs)-$len; $r--) { 
			my ($c,$d) = split(/,/,$lines[(split(/,/,$offs[$r]))[1]]);
			my $sz = (abs($c-$a)+1) * (abs($b-$d)+1);
			$max = $sz if ($sz > $max);
		}
	}
	return $max;
}

$part1 = fmax();
@offs = ();
for (my $i = 0; $i < scalar(@lines); $i++) {
	my ($x,$y) = split(/,/, $lines[$i]);
	push(@offs, sprintf("%14d,%d",$x1-$x+$y, $i));
}
@offs = sort(@offs);
my $m = fmax();
$part1 = $m if ($m > $part1);

my @br = split(/,/,$lines[248]);
my @tr = split(/,/,$lines[249]);
my $right = $br[0];

sub intersectLines 
{
#working subroutine. thanks to the original poster.
    my( $ax, $ay, $bx, $by, $cx, $cy, $dx, $dy )= @_;
    my @rval=0;
    
    my $d1=($ax-$bx)*($cy-$dy);
    my $d2=($ay-$by)*($cx-$dx);
    
    my $dp = $d1 - $d2;
    my $dq = $d2 - $d1;
    
    if($dp!=0 && $dq!=0) {    
       my $p = ( ($by-$dy)*($cx-$dx) - ($bx-$dx)*($cy-$dy) ) / $dp;   
+ 
       my $q = ( ($dy-$by)*($ax-$bx) - ($dx-$bx)*($ay-$by) ) / $dq;
      if($p>0 && $p<=1 && $q>0 && $q<=1) {    
         my $px= $p*$ax + (1-$p)*$bx;
         my $py= $p*$ay + (1-$p)*$by;
         @rval=($px, $py);
      }
   }
    
   return(@rval);
}		

sub insideRect
{
	my ($x,$y, @rect) = @_;
	if ($x > $rect[0] && $x < $rect[2]) {
		if ($y > $rect[1] && $y < $rect[3]) {
			return 1;
		}
#		if ($y < $rect[1] && $y > $rect[3]) {
#			return 1;
#		}
	}
	return 0;
}		

use GD;

my $im = GD::Image->new($x1 /10, $y1 / 10);
my $white = $im->colorAllocate(255,255,255);
my $black = $im->colorAllocate(0,0,0);
my $red = $im->colorAllocate(255,0,0);

my $poly = GD::Polygon->new();
foreach (@lines) {
	my ($x,$y) = split(/,/);
	$poly->addPt(int($x/10),int($y/10));
}
$im->polygon($poly,$red);

my $max = 0;
for (my $i = 0; $i < scalar(@lines); $i++) {
	my ($x,$y) = split(/,/,$lines[$i]);
	next if ($x >= $right);
	my @rect = (0,0,0,0);
	next if ($x >= $br[0]);
	if ($y > 50000) { # Bottom half, use @br as top right corner
		@rect = ($x,$br[1],$right,$y);
	}
	else {
		@rect = ($x,$y,$right,$tr[1]);
	}
	my $sz = ($rect[2]-$rect[0]+1)*($rect[3]-$rect[1]+1);
	if ($sz > $max) {
		for (my $j = 0; $j < scalar(@lines); $j++) {
			#next if ($i == $j);
			#next if ($j == 248 || $j == 249);
			my ($a,$b) = split(/,/,$lines[$j]);
			if (insideRect($a,$b,@rect)) {
				$sz = 0;
				last;
			}
		}

		if ($sz > $max) {
#			$poly = GD::Polygon->new();
#			$poly->addPt($rect[0]/10,$rect[1]/10);
#			$poly->addPt($rect[1],$rect[2]);
#			$poly->addPt($rect[2],$rect[3]);
			$im->rectangle($rect[0]/10,$rect[1]/10,$rect[2]/10,$rect[3]/10,$black);
			#$im->polygon($poly,$black);
			
			printf("Found new max $i (%s) -> %d\n", join(",",@rect), $sz);
			$max = $sz;
		}
	}
}
my $pngdata = $im->png;
open(my $p100,'>:raw',"p100.png");
print $p100 $pngdata;
close($p100);

$part2 = $max;
printf("1405718496 is too low\n");
printf("1600371528 is too high\n");

printf("1425773088 is not the correct answer\n");
printf("1470693630 is not the correct answer\n");
printf("1570961824 is not the correct answer\n");
printf("1577741808 is not the correct answer\n");
printf("1577920934 is not the correct answer\n");
printf("1592986320 is not the correct answer\n");
# 

# my ($px,$py) = @br;
# my @topr = (0,100000);
# my @botr;
# my @topl;
# my @botl;

# printf("br 248: %d,%d\n",@br);
# printf("tr 249: %d,%d\n",@tr);
# my $right = $br[0];
# for (my $i = 0; $i < scalar(@lines); $i++) {
	# my ($x,$y) = split(/,/,$lines[$i]);
	# next if ($x < 50000);
	# if ($y <= 50000) { # top half
		# if ($py == $y && (($px <= $right) != ($x <= $right))) {
#			next if ($y > $topr[1]);
			# @topr = ($right,$y);
			# printf("top right corner $i at %d,%d\n",@topr);
		# }
	# }
	# else {
		# if ($py == $y && (($px <= $right) != ($x <= $right))) {
			# @botr = ($right,$y);
			# printf("bottom right corner $i at %d,%d\n",@botr);
		# }
	# }
	# ($px,$py) = ($x,$y);
# }
# my @rect = ();
# for (my $i = 0; $i < scalar(@lines); $i++) {
	# my ($x,$y) = split(/,/,$lines[$i]);
	# next if ($x >= 50000);
	# if ($y <= 50000) { # Find vertical line corresponding to @topr
		# if ($px == $x && (($py <= $topr[1]) != ($y <= $topr[1]))) {
			# printf("top left:$i %d,%d -> %d,%d size:\n%d\n", $x,$y, @tr, ($right-$x+1)*($tr[1]-$y+1));
			# @rect = ($x/10,$y/10,$right/10, $tr[1]/10);
		# }
	# }
	# else {
		# if ($px == $x && (($py <= $botr[1]) != ($y <= $botr[1]))) {
			# @rect = ($x/10,$y/10,$right/10, $tr[1]/10);
			# printf("bottom left:$i $x,$y -> %d,%d size:\n%d\n", @br, ($right-$x+1)*($y-$br[1]+1));
		# }
	# }
	# ($px,$py) = ($x,$y);
# }

# $part2 = (96926-2643+1)*(46276-41698+1);

my $used = time - $start;

printf("%s\n%s\n", $part1, $part2);
printf("Used %5.3fms\n",$used*1000);
