package aoc2023.day8

lazy val input = io.Source.fromResource("day8.txt")
    .getLines()
    .toList

lazy val path =
    input.head.split("").toList

lazy val nodes =
    input.drop(2).foldLeft(Map.empty[String, (String, String)]):
        case (acc, s"$key = ($left, $right)") =>
            acc + (key -> (left, right))
        case (acc, _) =>
            acc

def findPath(start: String, exit: String => Boolean): Long =
    var cursor      = start
    var count       = 0L
    var currentPath = path

    while !exit(cursor) do
        val (left, right) = nodes(cursor)
        currentPath.head match
        case "L" =>
            cursor = left
        case "R" =>
            cursor = right

        currentPath = currentPath.tail
        if currentPath.isEmpty then
            currentPath = path
        count += 1
    count

def lcm(a: BigInt, b: BigInt): BigInt =
    (a * b).abs / a.gcd(b)

@main def run: Unit =
    val part1 = findPath("AAA", _ == "ZZZ")

    val part2 = nodes.keys
        .filter(_.endsWith("A"))
        .map: key =>
            BigInt(findPath(key, _.endsWith("Z")))
        .toList
        .reduce(lcm(_, _))

    println("\nDay 8")
    println("--------------")
    println(s"Part 1: $part1")
    println(s"Part 2: $part2\n")
