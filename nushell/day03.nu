let day03 = (open "../input/day03.txt" | lines)

let first = (
    $day03
    | each {
        |i| {
          a: ($i | str substring [0 (($i | str length) / 2)]), 
          b: ($i | str substring [(($i | str length) / 2) ($i | str length)]) 
        }
    }
    | each {
        |i| $i.a | split chars | where { |j| $i.b | str contains $j } | get 0
    }
    | each { 
        |i| $i | if $i <= 'Z' { ($i | into binary | get 0) - ('A' | into binary | get 0) + 27 } else { ($i | into binary | get 0) - ('a' | into binary | get 0) + 1 }
    }
    | math sum
)

let second = (
    $day03 | group 3
    | each {
        |i| $i.0 | split chars | where { |j| ($i.1 | str contains $j) && ($i.2 | str contains $j) } | get 0
    }    
    | each { 
        |i| $i | if $i <= 'Z' { ($i | into binary | get 0) - ('A' | into binary | get 0) + 27 } else { ($i | into binary | get 0) - ('a' | into binary | get 0) + 1 }
    }
    | math sum
)

{
    day: 3,
    a: ($first),
    b: ($second),
} | into df
