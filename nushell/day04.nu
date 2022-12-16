let day04 = (open "../input/day04.txt" | lines | parse "{a}-{b},{c}-{d}" | into int a b c d)

let completely = (
    $day04 
    | where { |i|
        (($i.a <= $i.c && $i.c <= $i.b && 
          $i.a <= $i.d && $i.d <= $i.b) || 
         ($i.c <= $i.a && $i.a <= $i.d && 
          $i.c <= $i.b && $i.b <= $i.d))
    } 
    | length
)

let some = (
    $day04 
    | where { |i|
        (($i.a <= $i.c && $i.c <= $i.b) || 
        ($i.a <= $i.d && $i.d <= $i.b) || 
        ($i.c <= $i.a && $i.a <= $i.d) || 
        ($i.c <= $i.b && $i.b <= $i.d))
    } 
    | length
)

{
    day: 4,
    a: ($completely),
    b: ($some),
} | into df
