package aoc2025.day6

def part1(input: String) = {
  input
    .split("\\s*\\r?\\n\\s*")
    .map(_.split("\\s+"))
    .transpose
    .map(_.reverse.toList)
    .map {
      case "+" :: rest =>
        rest.map(_.toLong).sum
      case "*" :: rest =>
        rest.map(_.toLong).product
      case _ => 0L
    }
    .sum
}

def part2(input: String) = {
  val lines = input.split("\\r?\\n").filterNot(_.isEmpty).toList
  // Operators position tells us how to split each line
  // (we need to preserve alignment, so spaces are important)
  val regex = "[*+]\\s+".r
  val operators = regex.findAllIn(lines.last).toList
  val indices = operators.scanLeft(0) { case (acc, op) => acc + op.length }.toList
  lines.dropRight(1)
    .map { line =>
      // Split the line according to operator positions
      // (to preserve alignment)
      indices.zip(indices.tail).map {
        case (start, end) =>
          line.substring(start, if end < line.length then end - 1 else end)
      }
    }
    .toList
    .transpose
    .zip(operators.map(_.trim))
    .map { case (cols, op) =>
      val nums = cols
        .map(_.toList)
        .transpose
        .map(_.filterNot(_ == ' ').mkString.toLong)
      op match {
        case "+" => nums.sum
        case "*" => nums.product
      }
    }
    .sum
}

@main def run(): Unit = {
  val input = io.Source.fromResource("day6.txt").mkString
  println("=-----------------------------=")
  println(s"Part 1: ${part1(input)}")
  println(s"Part 2: ${part2(input)}")
}
