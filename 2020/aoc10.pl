#!perl -w 

use strict;
use Time::HiRes qw(time);

my $inp = 
q(70
102
148
9
99
63
40
52
91
39
55
28
54
22
95
61
118
35
14
21
129
82
137
45
7
87
81
25
3
108
41
11
145
18
65
80
115
29
136
42
97
104
117
141
62
121
23
96
24
128
48
1
112
8
34
144
134
116
58
147
51
84
17
126
64
68
135
10
77
105
127
73
111
90
16
103
109
98
146
123
130
69
133
110
30
122
15
74
33
38
83
92
2
53
140
4);

my $preamble = 25;

if (defined(shift)) {
	$inp = 
q(28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3);

$preamble = 5;
}

my $t = time();

my @ops = sort {$a <=> $b} split(/\n/,$inp);
push(@ops, $ops[-1]+3);
unshift(@ops, 0);

my @diff = ();

for (my $i = 1; $i < scalar(@ops); $i++) {
	my $step = $ops[$i]-$ops[$i-1];
	die("Bad step $step") if ($step <= 0 || $step > 3);
	$diff[$step]++;
}
	
printf("%d step 1, %d step 3, prod = %d\n", $diff[1], $diff[3], $diff[1] * $diff[3]);

my @reach = (0) x $ops[-1]; $reach[0] = 1;
for (my $a = 1; $a < scalar(@ops); $a++) {
	my $curr = $ops[$a];
	my $r = 0;
	for (my $p = $curr-1; ($p >= 0) && ($p >= $curr-3); $p--) {
		$r += $reach[$p];
	}
	$reach[$curr] = $r;
}

printf("Total paths = %1.0f\n", $reach[-1]);	
$t = time()-$t;
printf("Total time = %1.5fms\n", $t*1000);

exit();

