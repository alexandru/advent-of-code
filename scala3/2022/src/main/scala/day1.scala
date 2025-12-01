package adventofcode.y2022

@main def day1: Unit =
  val elves = input.split("(?m)^\\s*$")
    .view
    .map(_.trim())
    .map: elfLines =>
      elfLines.split("(?m)$").toList.map(_.trim().toLong).sum

  val maxCalories = elves.maxOption
  println(s"\nMax calories: ${maxCalories.orNull}")

  val top3Sum = elves.sortBy(-_).take(3).sum
  println(s"\nCalories of top 3: ${top3Sum}\n")

private lazy val input =
  readResourceAsString("/day1.txt")
