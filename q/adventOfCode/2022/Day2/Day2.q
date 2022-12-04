e:`A`B`C!`X`Y`Z; // Equality
d:`X`Y`Z!`C`A`B; // Beats
p:`X`Y`Z!1 2 3;  // Default points
l:flip `$" " vs/: read0 `:input2.txt

// Default points + points for draw or win
sum p[l 1] + (3 * e[l 0]=l[1]) + (6 * l[0] = d[l 1])
//12855

// Part 2
// loose/draw/win
r:`X`Y`Z!0 1 2;
// Map opponents choice to loose/draw/win options
f:()!();
f[`A]:(0 1 2!`Z`X`Y);
f[`B]:(0 1 2!`X`Y`Z);
f[`C]:(0 1 2!`Y`Z`X);

sum p[f'[l[0]; r[l 1]]] + (3 * l[1]=`Y) + (6 * l[1]=`Z)
//13726
