my $w,$x,$y,$z=(0,0,0,0);
$w=shift @data;
$z = $w+3;

$w=shift @data;
$z *= 26;
$z += $w+12;

$w=shift @data;
$z *= 26;
$z += $w+21;

$w=shift @data; # z = w0
$x = $z % 26;
$x += -6;
$x = $x != $w;
$y = 25*$x+1;
$z *= $y;
$y = $w+12;
$y *= $x;
$z += $y;

$w=shift @data;
$z *= 26;
$z += $w+2;

$w=shift @data;
$x = $z % 26;
$z = int($z / 26);
$x += -8;
$x = $x != $w;
$y = 25*$x+1;
$z *= $y;
$y = $w+1;
$y *= $x;
$z += $y;

$w=shift @data;
$x = $z % 26;
$z = int($z / 26);
$x += -4;
$x = $x != $w;
$y = 25*$x+1;
$z *= $y;
$y = $w+1;
$y *= $x;
$z += $y;

$w=shift @data;
$z *= 26;
$z += $w+13;

$w=shift @data;
$z *= 26
$z += $w+1;

$w=shift @data;
$z *= 26;
$z += $w+6;

$w=shift @data;
$x = $z % 26;
$z = int($z / 26);
$x += -11;
$x = $x != $w;
$y = 25*$x+1;
$z *= $y;
$y = $w+2;
$y *= $x;
$z += $y;

$w=shift @data;
$x = $z % 26;
$z = int($z / 26);
$x = $x != $w;
$y = 25*$x+1;
$z *= $y;
$z *= $y;
$y = $w+11;
$y *= $x;
$z += $y;

$w=shift @data;
$x = $z % 26;
$z = int($z / 26);
$x += -8;
$x = $x != $w;
$y = 25*$x+1;
$z *= $y;
$y += $w+10;
$y *= $x;
$z += $y;

$w=shift @data;
$x = $z % 26;
$z = int($z / 26);
$x += -7;
$x = $x != $w;
$y = 25*$x+1;
$z *= $y;
$y = $w+3;
$y *= $x;
$z += $y;