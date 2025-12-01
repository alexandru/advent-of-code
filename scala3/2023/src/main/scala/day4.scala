package aoc2023.day4

/*
 * <https://adventofcode.com/2023/day/4>
 */

type CardNr = Int

case class Card(
    id: CardNr,
    winning: List[Int],
    guesses: List[Int]
)

@main def run =
  val CardRe = """Card\s+(\d+)\s*:([^|]+)[|]([^$]+)$""".r

  val input = io.Source.fromResource("day4.txt")
    .getLines
    .map(_.trim)
    .filter(_.nonEmpty)
    .to(LazyList)

  val cards = input.map:
    case CardRe(nrStr, winningStr, guessesStr) =>
      val winning = winningStr.trim().split("\\s+").map(_.toInt).toList
      val guesses = guessesStr.trim().split("\\s+").map(_.toInt).toList
      Card(nrStr.toInt, winning, guesses)

  val part1 = cards
    .map: card =>
      card.guesses.foldLeft(0): (acc, e) =>
        if card.winning.contains(e) then
          if acc == 0 then 1
          else acc * 2
        else
          acc
    .sum

  val part2 = cards
    .foldLeft(Map.empty[CardNr, Int]): (map, card) =>
      val count = card.guesses.intersect(card.winning).size
      val wonThis = map.getOrElse(card.id, 0) + 1
      val newMap0 = map.updated(card.id, wonThis)
      cards
        .drop(card.id)
        .take(count)
        .foldLeft(newMap0): (map, c) =>
          map.updated(c.id, map.getOrElse(c.id, 0) + wonThis)
    .values
    .sum

  println("\nDay 4\n------------")
  println(s"Part 1: $part1")
  println(s"Part 2: $part2\n")
