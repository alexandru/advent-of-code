package aoc2025.day2

def repeatsPartA(n: Long): Boolean = {
  val s = n.toString
  val p = s.substring(0, s.length() / 2)
  s == p * 2
}

def repeatsPartB(n: Long): Boolean = {
  val s = n.toString
  (1 to (s.length / 2))
    .view
    .map(s.take)
    .exists { p =>
      s == p * (s.length() / p.length)
    }
}

def sumOfInvalidIDs(p: Long => Boolean): Long = {
  io.Source.fromResource("day2.txt")
    .getLines()
    .map(_.trim)
    .filter(_.nonEmpty)
    .flatMap(_.split("\\s*,\\s*"))
    .map(_.split("-").map(_.toLong).toList)
    .flatMap {
      case start :: end :: Nil =>
        start.to(end).filter(p)
      case _ =>
        Nil
    }
    .foldLeft(0L)(_ + _)
}

@main def run(): Unit = {
  println(s"Part A: ${sumOfInvalidIDs(repeatsPartA)}")
  println(s"Part B: ${sumOfInvalidIDs(repeatsPartB)}")
}
