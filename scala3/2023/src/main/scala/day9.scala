package aoc2023.day9

val input = io.Source.fromResource("day9.txt")
    .getLines
    .to(LazyList)
    .map(_.split("\\s+").map(_.toLong).toList)

def extrapolate(history: List[Long]): (Long, Long) =
    val all =
        var step = history
        var all  = List.empty[List[Long]]
        while step.exists(_ != 0) do
            all = step :: all
            step = step.lazyZip(step.tail).map((x, y) => y - x)
        all

    all.foldLeft((0L, 0L)):
        case ((exFirst, exLast), line) =>
            val first = line.head - exFirst
            val last  = line.last + exLast
            (first, last)

@main def run =
    val (part2, part1) = input.map(extrapolate).foldLeft((0L, 0L)):
        case ((fs, ls), (first, last)) =>
            (fs + first, ls + last)

    println("\nDay 9")
    println("---------------")
    println(s"Part 1: $part1")
    println(s"Part 2: $part2\n")
