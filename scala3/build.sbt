val scala3Version = "3.3.1"

Global / onChangedBuildSource := ReloadOnSourceChanges

ThisBuild / scalaVersion := scala3Version
ThisBuild / version := "0.1.0"
ThisBuild / scalafmtOnCompile := true

lazy val y2022 = project
  .in(file("2022"))
  .settings(
    name := "Year 2022",
  )

lazy val y2023 = project
  .in(file("2023"))
  .settings(
    name := "Year 2023",
    Compile / run / mainClass := Some("aoc2023.day8.run"),
    libraryDependencies ++= Seq(
        "org.scalameta" %% "munit" % "0.7.29" % Test,
    )
  )

lazy val root = project
  .in(file("."))
  .aggregate(y2022, y2023)
