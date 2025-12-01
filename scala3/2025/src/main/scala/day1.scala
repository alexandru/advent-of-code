package aoc2025.day1

import scala.annotation.tailrec

enum Rotation {
  case Left(steps: Int)
  case Right(steps: Int)

  override def toString: String = this match {
    case Left(steps) => s"L$steps"
    case Right(steps) => s"R$steps"
  }
}

@tailrec
def mod(steps: Int, inc: Int, current: Int, pz: Int = 0): (Int, Int) =
  if steps == 0 then (current, pz)
  else {
    val next = current + inc
    if next == 0 then mod(steps - 1, inc, next, pz + 1)
    else if next < 0 then mod(steps - 1, inc, 99, pz)
    else if next == 100 then mod(steps - 1, inc, 0, pz + 1)
    else
      mod(steps - 1, inc, next, pz)
  }

case class State(
    current: Int,
    atZero: Int = 0,
    passesByZero: Int = 0
) {

  def evolve(rotation: Rotation): State = {
    val (next, isPassingByZero) = rotation match {
      case Rotation.Left(steps) => mod(steps, -1, current)
      case Rotation.Right(steps) => mod(steps, 1, current)
    }

    println(
      s"The dial is rotated $rotation to point at $next${
          if isPassingByZero > 0 then s", passing by zero $isPassingByZero time(s)" else ""
        }"
    )
    copy(
      current = next,
      atZero = if next == 0 then atZero + 1 else atZero,
      passesByZero = passesByZero + isPassingByZero
    )
  }
}

lazy val input = io.Source.fromResource("day1.txt")
  .getLines
  .map(_.trim)
  .filter(_.nonEmpty)
  .flatMap { line =>
    line.charAt(0) match {
      case 'L' => Some(Rotation.Left(line.drop(1).toInt))
      case 'R' => Some(Rotation.Right(line.drop(1).toInt))
      case _ => None
    }
  }
  .toVector

@main def run = {
  val state = input.foldLeft(State(50)) { (state, rotation) =>
    state.evolve(rotation)
  }
  println(s"A: ${state.atZero}")
  println(s"B: ${state.passesByZero}")
}
