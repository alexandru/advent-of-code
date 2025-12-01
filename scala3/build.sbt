val scala3Version = "3.7.4"

Global / onChangedBuildSource := ReloadOnSourceChanges

ThisBuild / scalaVersion := scala3Version
ThisBuild / version := "0.1.0"
ThisBuild / scalafmtOnCompile := true

lazy val y2022 = project
  .in(file("2022"))
  .settings(
    name := "Year 2022"
  )

lazy val y2023 = project
  .in(file("2023"))
  .settings(
    name := "Year 2023",
    Compile / run / mainClass := Some("aoc2023.day9.run"),
    libraryDependencies ++= Seq(
      "org.scalameta" %% "munit" % "1.2.1" % Test
    )
  )

lazy val y2025 = project
  .in(file("2025"))
  .settings(
    name := "Year 2025",
    libraryDependencies ++= Seq(
      "org.scalameta" %% "munit" % "1.2.1" % Test
    ),
    scalacOptions ++= Seq(
      "-no-indent",
      "-rewrite"
    )
  )

lazy val root = project
  .in(file("."))
  .aggregate(y2022, y2023, y2025)
