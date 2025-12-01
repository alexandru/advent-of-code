package aoc2023.day3

/*
 * <https://adventofcode.com/2023/day/3>
 */

def isSpecialChar(ch: Char) =
  (ch < '0' || ch > '9') && ch != '.'

def adjacentCoordonates(i: Int, j: Int) =
  List(
    (i, j + 1),
    (i, j - 1),
    (i + 1, j),
    (i + 1, j - 1),
    (i + 1, j + 1),
    (i - 1, j),
    (i - 1, j - 1),
    (i - 1, j + 1)
  )

@main def run =
  val matrix = io.Source.fromResource("day3.txt")
    .getLines
    .map(_.trim)
    .filter(_.nonEmpty)
    .toVector
    .map(_.toVector)

  var allNumbers = Vector.empty[Int]
  var allGears = Map.empty[(Int, Int), List[Int]]

  for i <- 0 until matrix.size do
    var isInNumber = false
    var isAdjacentSpecial = false
    var detectedGears = Set.empty[(Int, Int)]
    var partialNumber = 0

    for j <- 0 to matrix(i).size do
      matrix(i).lift(j) match
        case Some(ch) if ch >= '0' && ch <= '9' =>
          if !isInNumber then isInNumber = true
          partialNumber = partialNumber * 10 + (ch - '0')

          for
            (x, y) <- adjacentCoordonates(i, j)
            ch2 <- matrix.lift(x).flatMap(_.lift(y))
          do
            if isSpecialChar(ch2) then
              isAdjacentSpecial = true
            if ch2 == '*' then
              detectedGears += ((x, y))

        case _ if isInNumber =>
          // Updating global state
          if isAdjacentSpecial then
            allNumbers = allNumbers :+ partialNumber.toInt
          for gear <- detectedGears do
            allGears = allGears.updated(
              gear,
              partialNumber.toInt :: allGears.getOrElse(gear, Nil)
            )
          // Reset local state, preparing for next number
          isInNumber = false
          detectedGears = Set.empty
          isAdjacentSpecial = false
          partialNumber = 0

        case _ => ()

  val part1 = allNumbers.sum
  val part2 = allGears.values
    .filter(_.size >= 2)
    .map(_.foldLeft(1L)(_ * _.toLong)).sum

  println("\nDay 3\n------------")
  println(s"Part 1: $part1")
  println(s"Part 2: $part2\n")
