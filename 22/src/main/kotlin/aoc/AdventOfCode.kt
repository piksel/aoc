package aoc

import java.net.URI

class AdventOfCode {
    companion object {
        val BaseURL = URI("https://adventofcode.com")
        val SessionID: String? = System.getenv("AOC_SESSION")
    }
}