package aoc2023.day6

case class Race(time: Long, distance: Long):
    def possibilities: Int =
        // H/t: Will Billingsley, as my solution was more imperative :)
        Range.Long(0, time, 1).count: speed =>
            (time - speed) * speed > distance

lazy val input =
    io.Source.fromResource("day6.txt").getLines().to(LazyList)

lazy val inputPart1 =
    val time = input.collectFirst:
        case s"Time: $numbers" => numbers.trim().split("\\s+").map(_.toLong).toList
    val distance = input.collectFirst:
        case s"Distance: $numbers" => numbers.trim().split("\\s+").map(_.toLong).toList
    time.getOrElse(Nil)
        .zip(distance.getOrElse(Nil))
        .map(Race.apply.tupled)

lazy val inputPart2 =
    val time = inputPart1.map(_.time.toString).mkString.toLong
    val dist = inputPart1.map(_.distance.toString).mkString.toLong
    Race(time, dist)

@main def run(): Unit =
    val part1 = inputPart1.map(_.possibilities).product
    val part2 = inputPart2.possibilities

    println("\nDay 6")
    println("------------------")
    println(s"Part 1: ${part1}")
    println(s"Part 2: ${part2}\n")
