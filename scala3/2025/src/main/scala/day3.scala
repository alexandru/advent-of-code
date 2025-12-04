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

@main def run(): Unit = {
  val input = io.Source.fromResource("day3-test.txt").mkString
  println("=-----------------------------=")
  println(s"Part 1: ${process(2)(input)}")
  println(s"Part 2: ${process(12)(input)}")
}
