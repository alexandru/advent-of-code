package aoc2025.day5

def parseInput(input: String) = {
  val parts = input.trim.split("\\r?\\n\\s*\\r?\\n").toList
  parts match {
    case ranges :: ingredients :: Nil =>
      val rangeList = ranges
        .split("\\s*\\n\\s*")
        .toList
        .map { line =>
          val Array(start, end) = line.split("-").map(BigInt(_))
          (start, end)
        }

      val ingredientList = ingredients
        .split("\\s*\\n\\s*")
        .toList
        .map(s => BigInt(s.trim))

      (rangeList, ingredientList)
    case _ =>
      throw new IllegalArgumentException("Invalid input format")
  }
}

def part1(input: String): Int = {
  val (rangeList, ingredientList) = parseInput(input)

  ingredientList.count { ingredient =>
    rangeList.exists { case (start, end) =>
      ingredient >= start && ingredient <= end
    }
  }
}

def part2(input: String) = {
  val (rangeList, _) = parseInput(input)
  rangeList
    // Merge ranges
    .sortBy(_._1)
    .foldLeft(List.empty[(BigInt, BigInt)]) {
      case (Nil, curr) => List(curr)
      case (acc @ (lastStart, lastEnd) :: rest, (currStart, currEnd)) =>
        if currStart <= lastEnd + 1 then
          (lastStart, lastEnd.max(currEnd)) :: rest
        else
          (currStart, currEnd) :: acc
    }
    .reverse
    .foldLeft(BigInt(0)) {
      case (sum, (start, end)) =>
        sum + (end - start + 1)
    }
}

@main def run(): Unit = {
  val input = io.Source.fromResource("day5.txt").mkString
  println("=-----------------------------=")
  println(s"Part 1: ${part1(input)}")
  println(s"Part 2: ${part2(input)}")
}
