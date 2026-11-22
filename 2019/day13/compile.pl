#!perl -w

for (my $mode3 = 0; $mode3 < 3; $mode3++ ) {
  for (my $mode2 = 0; $mode2 < 3; $mode2++ ) {
    for (my $mode1 = 0; $mode1 < 3; $mode1++ ) {
	  my $m = $mode1*9 + $mode2*27 + $mode3*81;
	  printf("%3d => { let s1 = self.opcodes[ip+1]; let src1 = %s;  ret = src1; ip += 2;}\n",
			$m, 
			$mode1 == 1 ? "s1" : $mode1 == 0 ? "self.opcodes[s1 as usize]" : "self.opcodes[(s1+self.relative_base) as usize]");
	  printf("%3d => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3];\n\t let src1 = %s; let src2 = %s; let dst = %s;\n\tself.opcodes[dst as usize] = src1 + src2; ip += 4; },\n",
			$m+1, 
			$mode1 == 1 ? "s1" : $mode1 == 0 ? "self.opcodes[s1 as usize]" : "self.opcodes[(s1+self.relative_base) as usize]",
			$mode2 == 1 ? "s2" : $mode2 == 0 ? "self.opcodes[s2 as usize]" : "self.opcodes[(s2+self.relative_base) as usize]",
			$mode3 == 1 ? "d"  : $mode3 == 0 ? "self.opcodes[d  as usize]" : "self.opcodes[(d +self.relative_base) as usize]");
	  printf("%3d => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = %s; let src2 = %s; let dst = %s; self.opcodes[dst as usize] = src1 * src2; ip += 4; },\n",
			$m+2, 
			$mode1 == 1 ? "s1" : $mode1 == 0 ? "self.opcodes[s1 as usize]" : "self.opcodes[(s1+self.relative_base) as usize]",
			$mode2 == 1 ? "s2" : $mode2 == 0 ? "self.opcodes[s2 as usize]" : "self.opcodes[(s2+self.relative_base) as usize]",
			$mode3 == 1 ? "d"  : $mode3 == 0 ? "self.opcodes[d  as usize]" : "self.opcodes[(d +self.relative_base) as usize]");
  	  printf("%3d => { if self.inputs.len() == 0 {break}; let s1 = self.opcodes[ip+1]; let addr = %s; self.opcodes[addr as usize] = self.inputs.pop_front().unwrap(); ip += 2; }\n", 
			$m+3, 
			$mode1 == 1 ? "s1" : $mode1 == 0 ? "self.opcodes[s1 as usize]" : "self.opcodes[(s1+self.relative_base) as usize]");
	  printf("%3d => { let s1 = self.opcodes[ip+1]; let src1 = %s; self.outputs.push(src1); ret = src1; ip += 2;}\n",
			$m+4, 
			$mode1 == 1 ? "s1" : $mode1 == 0 ? "self.opcodes[s1 as usize]" : "self.opcodes[(s1+self.relative_base) as usize]");
	  printf("%3d => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = %s; let src2 = %s; ip = if src1 != 0 {src2 as usize}else{ip + 3} }\n",
			$m+5,
			$mode1 == 1 ? "s1" : $mode1 == 0 ? "self.opcodes[s1 as usize]" : "self.opcodes[(s1+self.relative_base) as usize]",
			$mode2 == 1 ? "s2" : $mode2 == 0 ? "self.opcodes[s2 as usize]" : "self.opcodes[(s2+self.relative_base) as usize]");
	  printf("%3d => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let src1 = %s; let src2 = %s; ip = if src1 == 0 {src2 as usize}else{ip + 3} }\n",
			$m+6,
			$mode1 == 1 ? "s1" : $mode1 == 0 ? "self.opcodes[s1 as usize]" : "self.opcodes[(s1+self.relative_base) as usize]",
			$mode2 == 1 ? "s2" : $mode2 == 0 ? "self.opcodes[s2 as usize]" : "self.opcodes[(s2+self.relative_base) as usize]");
	  printf("%3d => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = %s; let src2 = %s; let dst = %s; self.opcodes[dst as usize] = (src1 < src2) as i32; ip += 4; },\n",	  
			$m+7,
			$mode1 == 1 ? "s1" : $mode1 == 0 ? "self.opcodes[s1 as usize]" : "self.opcodes[(s1+self.relative_base) as usize]",
			$mode2 == 1 ? "s2" : $mode2 == 0 ? "self.opcodes[s2 as usize]" : "self.opcodes[(s2+self.relative_base) as usize]",
			$mode3 == 1 ? "d"  : $mode3 == 0 ? "self.opcodes[d  as usize]" : "self.opcodes[(d +self.relative_base) as usize]");
	  printf("%3d => { let s1 = self.opcodes[ip+1]; let s2 = self.opcodes[ip+2]; let d = self.opcodes[ip+3]; let src1 = %s; let src2 = %s; let dst = %s; self.opcodes[dst as usize] = (src1 == src2) as i32; ip += 4; },\n",	  
			$m+8,
			$mode1 == 1 ? "s1" : $mode1 == 0 ? "self.opcodes[s1 as usize]" : "self.opcodes[(s1+self.relative_base) as usize]",
			$mode2 == 1 ? "s2" : $mode2 == 0 ? "self.opcodes[s2 as usize]" : "self.opcodes[(s2+self.relative_base) as usize]",
			$mode3 == 1 ? "d"  : $mode3 == 0 ? "self.opcodes[d  as usize]" : "self.opcodes[(d +self.relative_base) as usize]");
	}
  }
}
printf("243 => break;,\n"); 	  # 243 == 99 exit
