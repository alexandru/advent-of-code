package aoc2023.day1

/*
 * <https://adventofcode.com/2023/day/1>
 */

def part1 =
    val regex = """\d""".r
    io.Source
        .fromResource("day1.txt")
        .getLines
        .flatMap: line =>
            val all = regex.findAllIn(line).map(_.toInt).toVector
            for
                f <- all.headOption
                l <- all.lastOption
            yield f * 10 + l
        .sum

def part2 =
    val regex = """\d|one|two|three|four|five|six|seven|eight|nine""".r
    val digitsMap = Map(
      "one"   -> 1,
      "two"   -> 2,
      "three" -> 3,
      "four"  -> 4,
      "five"  -> 5,
      "six"   -> 6,
      "seven" -> 7,
      "eight" -> 8,
      "nine"  -> 9
    )
    io.Source
        .fromResource("day1.txt")
        .getLines
        .flatMap: line =>
            val all = Vector.unfold(line): line =>
                for
                    m <- regex.findFirstMatchIn(line)
                    ds = m.matched
                    di = digitsMap.getOrElse(ds, ds.toInt)
                yield (di, line.substring(m.start + 1))
            for
                f <- all.headOption
                l <- all.lastOption
            yield f * 10 + l
        .sum

@main def run: Unit =
    println("\nDay 1\n------------")
    println(s"Part 1: $part1")
    println(s"Part 2: $part2\n")
