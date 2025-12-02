package aoc2025.day2

import scala.util.matching.Regex

lazy val invalidA = """^(\d+)\1$""".r
lazy val invalidB = """^(\d+)\1+$""".r

def sumOfInvalidIDs(p: Regex)(input: String): Long =
  input
    .split("\n")
    .map(_.trim)
    .filter(_.nonEmpty)
    .flatMap(_.split("\\s*,\\s*"))
    .map(_.split("-").map(_.toLong).toList)
    .flatMap {
      case start :: end :: Nil =>
        start.to(end).filter(n => p.matches(n.toString))
      case _ =>
        Nil
    }
    .foldLeft(0L)(_ + _)

@main def run(): Unit = {
  val input = io.Source.fromResource("day2.txt").mkString
  println(s"Part A: ${sumOfInvalidIDs(invalidA)(input)}")
  println(s"Part B: ${sumOfInvalidIDs(invalidB)(input)}")
}
