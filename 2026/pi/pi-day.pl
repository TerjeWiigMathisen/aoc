#!perl -w
use strict;

my $letters = ".-,-...,-.-.,-..,.,..-.,--.,....,..,.---,-.-,.-..,--,-.,---,.--.,--.-,.-.,...,-,..-,...-,.--,-..-,-.--,--..";
my $digits = ".----,..---,...--,....-,.....,-....,--...,---..,----.,-----";
my $symbols = "..--..,-.-.--,.-.-.-,---...,.-.-.,-....-,-..-.,-...-";

my %morse = ();
my %decode = ();

my $l = 'a';
foreach (split(/,/,$letters)) {
	$decode{$_} = $l;
	$morse{$l} = $_;
	printf(STDERR "%s -> %s\n",$l, $_);
	$l++;
}
my $d = '1';
foreach (split(/,/,$digits)) {
	$decode{$_} = $d;
	$morse{$d} = $_;
	printf(STDERR "%s -> %s\n",$d, $_);
	$d = $d < '9' ? ++$d : '0';
}

my $s = 0;
my $symboler = "?!.,;:+-/=";
foreach (split(/,/,$symbols)) {
	my $sym = substr($symboler,$s,1);
	$decode{$_} = $sym;
	$morse{$sym} = $_;
}

my $signal = "....  .  -.--  ,     -..  ..  -..      -.--  ---  ..-     -.-  -.  ---  .--      -  ....  .-  -     π  --.  ....  ---  ...  -      .-..  ---  ...-  .  ...     .--.  ..  .  ...  ?
";

sub decode
{
	my ($signal) = @_;
	my $res = '';
	while (index($signal, ' ') >= 0) { 
		my $s = index($signal, ' ');
		my $sym = substr($signal,0,$s);
		my $decoded = $decode{$sym};
		if (defined($decoded)) {
			#printf(STDERR "%s -> %s\n",$sym, $decoded);
		} else {
			#printf(STDERR "Not defined: %s\n",$sym);
			$decoded = $sym;
			#exit(1);
		}
		$signal = substr($signal,$s);
		my $l = 0;
		while (substr($signal,$l,1) eq ' ') { $l++; }
		$signal = substr($signal,$l);
		$res .= $decoded;
		if ($l != 2 && $l != 3) { $res .= ' '; }
	}
	my $decoded = $decode{$signal};
	$decoded = $signal unless (defined($decoded));
	$res .= $decoded;
	return $res;
}

#print decode($signal);

my $code = q(--.   .-.   ..   -..   : -.   -..-   -.
-.    = .----   -----   -----

 ...   .   .   -..   :    .--.   .. -..   ..   --.   ..   -   ...

 -.-.   ---   -.   ...   -   .-.   ..-   -.-.   -    -   ....   . --.   .-.   ..   -..    ..-   ...   ..   -.   --. -   ....   .    ..-.   ..   .-.   ...   - .----   -----   -----   -----   -----    -..   ..   --.   ..   -   ... ---   ..-.    .--.   ..
 -...   .   --.   ..   -. .-   -    (   -----   ,   -----   )
 .-.   .   .-   -.-.   .... (   ----.   ----.   ,   ----.   ----.   )

 .--.   .-   ...   ...   -.-.   ---   -..   .   :    .--.   ..   --.-   ..---   -----   ..---   -....

 --   ---   ...-   .   --   .   -.   - .-   .-..   .-..   ---   .--   .   -..    ..   -. ..-.   ---   ..-   .-.    -..   ..   .-.   .   -.-.   -   ..   ---   -.   ...
 -.-.   ---   ...   - -..   .   .--.   .   -.   -..   ...    ---   -. -   ....   .    --   ---   .-.   ...   . ---   ..-.    -   ....   . -..   ..   --.   ..   -);

foreach (split(/\n/,$code)) {
	printf("%s\n",decode($_));
}