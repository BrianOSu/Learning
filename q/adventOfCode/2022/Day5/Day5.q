// Preprocess input to work with it
l:read0 `:input5.txt;

// Find where to split and the number of cols
splitPos: first where l like "";
colCnt:"I"$last " " vs l[splitPos-1];

// Converts cols to lists of chars
initialPos:trim reverse'[flip (splitPos-1)#l][1+4*til splitPos];

// Extract the instructions
i:"I"$(" " vs/: (splitPos+1)_l)[;1 3 5];

// Part 1
pos:initialPos;
// Apply instructions
{pos[z-1]::pos[z-1],reverse neg[x]#pos[y-1]; pos[y-1]::neg[x]_pos[y-1]}./:i;

last each pos
// PTWLTDSJV

// Part 2
pos:initialPos;
// Apply instructions
{pos[z-1]::pos[z-1],neg[x]#pos[y-1]; pos[y-1]::neg[x]_pos[y-1]}./:i;

last each pos
// WZMFVGGZP