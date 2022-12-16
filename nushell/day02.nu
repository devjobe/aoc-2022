let day02 = (open "../input/day02.txt" | lines | split column " " a b | update a {|f| { A:0, B:1, C:2 } | get $f.a } | update b {|f| { X:0, Y:1, Z:2 } | get $f.b })
{
    day: 2,
    a: ($day02 | each { |it| $it.b + 1 + ($it.a * 2 + 1 + $it.b) mod 3 * 3 } | math sum)
    b: ($day02 | each { |it| ($it.a + 2 + $it.b) mod 3 + 1 + $it.b * 3 } | math sum)
} | into df