package aoc2025.day4

import scala.collection.View

def process(grid: Array[Array[Char]]): Int = {
  var count = 0
  for i <- grid.indices do {
    for j <- grid(i).indices do {
      val neighbors = {
        val nearby =
          for {
            di <- List(-1, 0, 1)
            dj <- List(-1, 0, 1)
            if di != 0 || dj != 0
            ch = grid.lift(i + di).flatMap(_.lift(j + dj)).getOrElse('.')
          } yield ch
        nearby.count { ch =>
          ch == '@' || ch == 'x'
        }
      }
      if neighbors < 4 && grid(i)(j) == '@' then {
        grid(i)(j) = 'x'
        count += 1
      }
    }
  }
  count
}

def part1(input: View[String]): Int = {
  val grid = input.map(_.toCharArray).toArray
  process(grid)
}

def part2(input: View[String]): Int = {
  val grid = input.map(_.toCharArray).toArray
  var count = 0
  var isDone = false

  while !isDone do {
    isDone = true
    count += process(grid)

    for i <- grid.indices do
      for j <- grid(i).indices do
        if grid(i)(j) == 'x' then {
          grid(i)(j) = '.'
          isDone = false
        }
  }

  count
}

@main def run(): Unit = {
  val input = io.Source.fromResource("day4.txt").mkString
  val lines = input
    .split("\\s*\\n\\s*")
    .view
    .map(_.trim)
    .filter(_.nonEmpty)

  println("=-----------------------------=")
  println(s"Part 1: ${part1(lines)}")
  println(s"Part 2: ${part2(lines)}")
}
