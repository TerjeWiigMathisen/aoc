[2024 Day 9] In Review (Disk Fragmenter)

Today an amphipod has a classic problem: it has a disk that is fragmented and it wants to defragment it.
The disk map is stored in a compressed format, alternating between a disk length and a free space length. If there is no free space between two files, the free space length is 0.

For Part1, the amphipod moves blocks, starting with the last file block which it then moves into the first free block, systematically moving all blocks as close to the front as possible, and as a side effect reversing the storage order for all moved files. We then produce a disk checksum consisting of summing up for every file block the product of the file number and the block position.

For Part2, because the amphipod ralized that the disk becomes too fragmented, it will instead only move full files as close to the front as it can find a large enough free space for them. The checksum is then calculated in the same way as Part1.

My Perl code 