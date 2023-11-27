def part1: Unit =
    val regex = """\d""".r
    val sum = io.Source
        .fromResource("day1.txt")
        .getLines()
        .flatMap: line =>
            val all = regex.findAllIn(line).toVector.map(_.toInt)
            for
                f <- all.headOption
                l <- all.lastOption
            yield f * 10 + l
        .sum
    println(s"Sum of calibration values: $sum")

@main def part2: Unit =
    val regex = """\d|one|two|three|four|five|six|seven|eight|nine""".r
    val digitsMap = Map(
      "one" -> 1,
      "two" -> 2,
      "three" -> 3,
      "four" -> 4,
      "five" -> 5,
      "six" -> 6,
      "seven" -> 7,
      "eight" -> 8,
      "nine" -> 9
    )

    def findDigits(line: String, acc: Vector[String] = Vector.empty): Vector[Int] =
        regex.findFirstMatchIn(line) match
        case None =>
            acc.map(v => digitsMap.getOrElse(v, v.toInt))
        case Some(m) =>
            findDigits(line.substring(m.start + 1), acc :+ m.matched)

    val sum = io.Source
        .fromResource("day1.txt")
        .getLines()
        .flatMap: line =>
            val all = findDigits(line)
            for
                f <- all.headOption
                l <- all.lastOption
            yield f * 10 + l
        .sum

    println(s"Sum of calibration values: $sum")
