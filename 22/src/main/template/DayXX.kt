package days
import aoc.*
import org.testng.Assert.assertEquals
import org.testng.annotations.Test
import java.io.File
import colors.*

fun main() {
    DayXX().solve(Part.First)
    println()
    // DayXX().solve(Part.Second)
}

class DayXXTests {
    @Test fun solveFirst() = assertEquals("??", DayXX().solveExample(Part.First))
    @Test fun solveSecond() = assertEquals("??", DayXX().solveExample(Part.Second))
}

class DayXX: Day() {

    override val secondUsesSameInput: Boolean get() = true

    override fun solveFirst(input: File): String =
        input.useLines { lines ->
            TODO("day XX silver solution is not implemented")
        }.toString()

    override fun solveSecond(input: File): String =
        input.useLines { lines ->
            TODO("day XX gold solution is not implemented")
        }.toString()
}
