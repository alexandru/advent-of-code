package aoc2023.day5

import scala.collection.immutable.SortedSet

case class Range(startInclusive: Long, endExclusive: Long):
  assert(startInclusive < endExclusive, s"Invalid range: $toString")

  override def toString =
    s"[$startInclusive, $endExclusive)"

  def cutAndTranslate(mask: Range, delta: Long): (Option[Range], SortedSet[Range]) =
    val s = math.max(startInclusive, mask.startInclusive)
    val e = math.min(mask.endExclusive, endExclusive)
    Range.validated(s, e) match
      case Some(i) =>
        val newr = Range(i.startInclusive + delta, i.endExclusive + delta)
        val rest = List(
          Range.validated(mask.startInclusive, startInclusive),
          Range.validated(i.endExclusive, mask.endExclusive)
        )
        (Some(newr), SortedSet(rest.flatten*))
      case None =>
        (None, SortedSet(mask))

object Range:
  def validated(startI: Long, endE: Long): Option[Range] =
    Option.when(startI < endE)(Range(startI, endE))

  given Ordering[Range] with
    def compare(x: Range, y: Range): Int =
      x.startInclusive.compare(y.startInclusive) match
        case 0 => x.endExclusive.compare(y.endExclusive)
        case c => c

case class RangeMapping(range: Range, mapToStartInclusive: Long):
  val delta = mapToStartInclusive - range.startInclusive

  override def toString =
    s"$range{delta:$delta}"

  def cutAndTranslate(mask: Range): (Option[Range], SortedSet[Range]) =
    this.range.cutAndTranslate(mask, delta)

case class Layer(ranges: List[RangeMapping]):
  def translate(range: Range): SortedSet[Range] =
    var remainingSource = SortedSet(range)
    var translated = SortedSet.empty[Range]

    while remainingSource.nonEmpty do
      val currentSource = remainingSource.firstKey
      remainingSource = remainingSource - currentSource

      val found = this.ranges.view
        .map(_.cutAndTranslate(currentSource))
        .collectFirst:
          case (Some(r), rest) => (r, rest)

      found match
        case None =>
          translated = translated + currentSource
        case Some((tr, rest)) =>
          translated = translated + tr
          remainingSource = remainingSource ++ rest

    translated

def calculate(layers: List[Layer], seeds: SortedSet[Range]): Long =
  layers
    .foldLeft(seeds): (seeds, layer) =>
      seeds.flatMap(layer.translate)
    .map(_.startInclusive)
    .min

@main def run =
  val input = io.Source.fromResource("day5.txt")
    .mkString
    .split("\r?\n\\s*\r?\n")
    .toList

  val (seedsPart1, seedsPart2) =
    val seedsRaw = input.headOption match
      case Some(s"seeds: $seeds") =>
        seeds.split("\\s+").toList.map(_.toLong)
      case _ =>
        throw RuntimeException(s"Invalid seeds line: ${input.headOption}")

    val p1 = SortedSet.from(seedsRaw.map: point =>
      Range(point, point + 1))
    val p2 = SortedSet.from(seedsRaw.sliding(2, 2).collect:
      case List(a, len) => Range(a, a + len))
    (p1, p2)

  val layers = input.tail.map: layerStr =>
    val lines = layerStr.split("\r?\n").toList

    Layer(lines.tail.map: rangeStr =>
      rangeStr.split("\\s+").map(_.toLong).toList match
        case List(dest, src, len) =>
          RangeMapping(Range(src, src + len), dest)
        case _ =>
          throw RuntimeException(s"Invalid line: $rangeStr"))

  println("\nDay 5")
  println("----------------")
  println(s"Part 1: ${calculate(layers, seedsPart1)}")
  println(s"Part 2: ${calculate(layers, seedsPart2)}\n")
