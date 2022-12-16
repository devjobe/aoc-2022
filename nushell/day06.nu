let day06 = (open "../input/day06.txt" | split chars)

let a = ($day06 | window 4 | each -n { |i| { end: ($i.index + 4), n: ($i.item | uniq | length ) } } | where $it.n == 4 | first | $in.end)
let b = ($day06 | window 14 | each -n { |i| { end: ($i.index + 14), n: ($i.item | uniq | length ) } } | where $it.n == 14 | first | $in.end)

{
    day: 6,
    a: ($a),
    b: ($b)
} | into df
