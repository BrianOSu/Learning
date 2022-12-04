l:read0 `:input3.txt

// Part 1
// Split each input in 2
x:("i"$2,/:count'[l] % 2) #' l;
c:raze distinct each inter'[x[;0]; x[;1]];
sum 1+where each c =\: .Q.a,.Q.A
// 7908

// Part 2
n:raze distinct each inter/'[3 cut l]
sum 1+where each n =\: .Q.a,.Q.A
//2838
