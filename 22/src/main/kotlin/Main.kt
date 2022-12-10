@file:Suppress("UnusedImport")

import aoc.*
import colors.*
import days.*

fun main(args: Array<String>) {
    println("${red("Advent")} of ${green("Code")} ${cyan("2022")}")
    println()

    when (val action = args.elementAtOrElse(0) { "list" }) {
        "list" -> {
            println("Days:")

            for (day in Day) {
                day.quiet = true
                println("</> ${yellow(day.name.replaceFirstChar { it.uppercaseChar() })}")
                day.solve(Part.First)
                day.solve(Part.Second)
                println()

            }
        }
        "solve" -> {
            val day = Day.getDay(args.elementAt(1).toInt()).getOrThrow()

            day.solve(Part.First)
            println()
            day.solve(Part.Second)
        }
        else -> {
            println("Invalid action $action. Possible actions: list, solve")
        }
    }

}