let day01 = (open "../input/day01.txt" | split row "\r\n\r\n" | each { |entry| $entry | lines | into int | math sum } | sort | last 3)
{ 
 day: 1,
 a: ($day01 | last), 
 b: ($day01 | math sum) 
} | into df

