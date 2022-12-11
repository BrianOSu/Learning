l:read0 `:input7.txt;

// Part 1

// Reduce input to folder/files
reduced:l where not any l like/: ("dir*"; "* cd .."; "*ls");
// Find the size of each folder ignoring nesting sizes
size:sum each "J"$1_/:where[reduced like "* cd *"] cut first each " " vs/: reduced;

// Extract directory names
dir:raze `$-1#/:" " vs/: l where l like "* cd *";

// Identify full path for each folder
paths:` sv/: (),/:{if[y~`..; :-1_x]; x,y}\[dir] where not dir=`..;

// Create a table mapping path to size, also including nested folder size
dirSizes:select sum size by paths from (ungroup update ` sv/:/: paths from (update (,\) each `$"." vs/: string paths from ([]paths;size)))

select sum size from dirSizes where size<=100000
// 1444896


// part 2
exec first size from `size xasc select from (update required:30000000-70000000-max size from dirSizes) where required<size
//404395
