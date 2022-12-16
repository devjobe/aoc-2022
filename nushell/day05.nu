let initial = (open "../input/day05.txt" | lines | take until { |line| $line == "" } | each { |line| $line | split chars | skip 1 | every 4 } );
let moves = (open "../input/day05.txt" | lines | skip until { |line| $line == "" } | skip 1 | parse "move {n} from {from} to {to}" | into int n from to | update from {|f| $f.from - 1} | update to {|f| $f.to - 1})

let rows = ($initial | reverse | skip 1);
let stacks = (for $i in 0..($rows | length) { $rows | each {|j| $j | get $i } | where {|a| $a != " "}})

let first = (
    $moves 
    | reduce -f $stacks { 
        |i,acc| 
        $acc 
        | update $i.to ($acc | get $i.to | append ($acc | get $i.from | last $i.n | reverse)) 
        | update $i.from ($acc | get $i.from | range ..(-1 - $i.n)) 
    } 
    | each {|i| $i | last } | where $it != " " | str collect
)

let second = (
    $moves 
    | reduce -f $stacks { 
        |i,acc| $acc 
            | update $i.to ($acc | get $i.to | append ($acc | get $i.from | last $i.n)) 
            | update $i.from ($acc | get $i.from | range ..(-1 - $i.n))
        }
    | each {|i| $i | last } | where $it != " " | str collect    
)

{
    day: 5,
    a: ($first),
    b: ($second),
} | into df
