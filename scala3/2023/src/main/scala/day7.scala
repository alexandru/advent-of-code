package aoc2023.day7

import scala.annotation.nowarn

type Bid = Long

enum HandType(val value: Int):
    case FiveOfAKind  extends HandType(6)
    case FourOfAKind  extends HandType(5)
    case FullHouse    extends HandType(4)
    case ThreeOfAKind extends HandType(3)
    case TwoPair      extends HandType(2)
    case OnePair      extends HandType(1)
    case HighCard     extends HandType(0)

case class Hand(value: String, withJoker: Boolean):
    override def toString = s"$value{$handType}"

    val toNumList =
        val cards = if withJoker then Hand.cardsWithJoker else Hand.cardsWithoutJoker
        value.toList.map(ch => cards(ch.toString))

    val handType =
        def evalGroups(groups: List[Int]): HandType =
            if groups.exists(_ == 5) then
                HandType.FiveOfAKind
            else if groups.exists(_ == 4) then
                HandType.FourOfAKind
            else if groups.exists(_ == 3) && groups.exists(_ == 2) then
                HandType.FullHouse
            else if groups.exists(_ == 3) then
                HandType.ThreeOfAKind
            else if groups.count(_ == 2) == 2 then
                HandType.TwoPair
            else if groups.exists(_ == 2) then
                HandType.OnePair
            else
                HandType.HighCard

        val permutations =
            if withJoker then Hand.cardsWithJoker.keySet
            else Set("J")

        permutations.map: key =>
            evalGroups(value
                .replaceAll("J", key.toString)
                .view
                .groupBy(identity)
                .values
                .map(_.size)
                .toList)
        .maxBy(_.value)

object Hand:
    private def buildCards(str: String) = str
        .split(", ")
        .reverse
        .zipWithIndex
        .toMap

    val cardsWithoutJoker =
        buildCards("A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2")
    val cardsWithJoker =
        buildCards("A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J")

    given Ordering[Hand] with
        def compare(x: Hand, y: Hand): Int =
            import math.Ordered.orderingToOrdered
            import Ordering.Implicits.*
            x.handType.value.compare(y.handType.value) match
            case 0 => x.toNumList.compare(y.toNumList)
            case c => c

def calculate(hands: List[(Hand, Bid)]): Long =
    hands.sorted
        .zipWithIndex
        .map:
            case ((hand, bid), rank) =>
                bid * (rank + 1)
        .sum

@nowarn
@main def run: Unit =
    val part1Cards: List[(Hand, Bid)] = io.Source.fromResource("day7.txt")
        .getLines()
        .map(_.split("\\s+").toList)
        .map {
            case List(hand, bid) => (Hand(hand, withJoker = false), bid.toLong)
        }
        .toList

    val part1 = calculate(part1Cards)
    val part2 = calculate(part1Cards.map:
        case (hand, bid) => (hand.copy(withJoker = true), bid)
    )

    println("\nDay 7")
    println("--------------")
    println(s"Part 1: $part1")
    println(s"Part 2: $part2\n")
