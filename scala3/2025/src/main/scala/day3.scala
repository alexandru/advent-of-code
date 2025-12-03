package aoc2025.day3

def process(charsCount: Int)(input: String): Long = {
  def max(line: String, remaining: Int): String = {
    val maxIdx = (0 until line.length - remaining + 1)
      .maxBy(i => line.charAt(i))
    val rest =
      if remaining > 1 then
        // recursive call, unsafe
        max(line.substring(maxIdx + 1), remaining - 1)
      else
        ""
    s"${line.charAt(maxIdx)}$rest"
  }

  input.split("\\s*\\n\\s*")
    .view
    .map(_.trim)
    .filter(_.nonEmpty)
    .map { line =>
      max(line, charsCount).toLong
    }
    .map(_.toLong)
    .sum
}

def part1(input: String): Long = {
  input.split("\\s*\\n\\s*")
    .view
    .map(_.trim)
    .filter(_.nonEmpty)
    .map { line =>
      (0 until line.length - 1)
        .view
        .map { i =>
          (i + 1 until line.length)
            .map(j => s"${line.charAt(i)}${line.charAt(j)}")
            .map(_.toLong)
            .max
        }
        .max
    }
    .sum
}

@main def run(): Unit = {
  val input = io.Source.fromResource("day3.txt").mkString
  println("=-----------------------------=")
  println(s"Part 1: ${part1(input)}")
  println(s"Part 1: ${process(2)(input)}")
  println(s"Part 2: ${process(12)(input)}")
}
