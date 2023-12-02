package aoc2023.day2

case class GameSet(red: Int, green: Int, blue: Int):
    def <=(other: GameSet): Boolean =
        red <= other.red && green <= other.green && blue <= other.blue

    def power =
        red * green * blue

case class Game(id: Int, sets: List[GameSet]):
    def minimum = GameSet(
      red = sets.map(_.red).max,
      green = sets.map(_.green).max,
      blue = sets.map(_.blue).max
    )

@main def run: Unit =
    val expected = GameSet(red = 12, green = 13, blue = 14)

    val ExtractGame  = """^Game\s+(\d+)[:]([^$]+)$""".r
    val ExtractCount = """^\s*(\d+)\s+(red|green|blue)\s*$""".r

    val games = io.Source.fromResource("day2.txt").getLines.collect:
        case ExtractGame(id, setsStr) =>
            val sets = setsStr
                .split("\\s*;\\s*")
                .map: gameSet =>
                    val map = gameSet.split("\\s*,\\s*")
                        .map:
                            case ExtractCount(nr, color) => (color, nr.toInt)
                        .toMap
                    GameSet(
                      red = map.getOrElse("red", 0),
                      green = map.getOrElse("green", 0),
                      blue = map.getOrElse("blue", 0)
                    )
                .toList
            Game(id.toInt, sets)

    val part1 = games
        .map: game =>
            if game.sets.forall(_ <= expected)
            then game.id
            else 0
        .sum

    val part2 =
        games.map(_.minimum.power).sum

    println("\nDay 2\n------------")
    println(s"Part 1: $part1")
    println(s"Part 2: $part2\n")
