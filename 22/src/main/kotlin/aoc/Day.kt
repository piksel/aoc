package aoc
import colors.cyan
import colors.green
import colors.red
import colors.yellow
import java.io.File
import java.net.CookieManager
import java.net.HttpCookie
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse.BodyHandlers
import java.nio.file.Path
import java.nio.file.Paths
import java.time.Instant
import kotlin.io.path.div

abstract class Day {
    open fun getInput(part: Part): File {
        val file = getInputFile(part, false)
        if (!file.exists()) {
            if(AdventOfCode.SessionID.isNullOrBlank())
                throw Exception("No solution input was found and no AOC_SESSION variable set")
            file.parentFile.mkdirs()
            val hc = HttpClient.newBuilder()
                .cookieHandler(CookieManager().apply {
                    cookieStore.add(
                        AdventOfCode.BaseURL,
                        HttpCookie("session", AdventOfCode.SessionID).apply { version = 0; path = "/" }
                    )
                })
                .build()
            val url = AdventOfCode.BaseURL.resolve("/2022/day/$day/input")
            log("Downloading input from $url...")
            hc.send(
                HttpRequest.newBuilder(url).GET().build(),
                BodyHandlers.ofFile(file.toPath())
            )
        }
        return file
    }

    open fun getTestInput(part: Part): File {
        val file = getInputFile(part, true)
        if (!file.exists()) {
            file.parentFile.mkdirs()
            // TODO: Prompt to add example
            file.createNewFile()
            log("Example input missing! Add it to ${yellow(file.absolutePath)}!")
        }
        return file
    }

    open val name: String get() = this.javaClass.simpleName.lowercase()
    open val day: Int get() = name.let { it[it.lastIndex] }.digitToInt()

    fun solveExample(part: Part): String =
        getTestInput(part).let { if(part == Part.First) solveFirst(it) else solveSecond(it) }

    open fun getInputFile(part: Part, example: Boolean): File =
        (getWorkRoot() / "inputs" / name / "${if (secondUsesSameInput || part == Part.First)"silver" else "gold"}${if (example) "_example" else ""}").toFile()

    fun solve(part: Part) {
        solveStart = Instant.now()
        val prettyPart = when(part){
            Part.First -> cyan("silver")
            Part.Second -> yellow("gold")
        }

        print("Solving ${green(name)} $prettyPart star... ")
        if (!quiet) println()

        day.runCatching { when(part) {
            Part.First -> solveFirst(getInput(part))
            Part.Second -> solveSecond(getInput(part))
        }}.fold(
            { println("Answer: ${cyan(it)}") },
            { println("${red("Failed")}: ${it.message}") }
        )
    }

    var quiet: Boolean = false

    private var solveStart: Instant = Instant.now()

    private val elapsed: String get() = "%03i".format(solveStart.epochSecond - Instant.now().epochSecond)

    fun log(m: String) {
        if (quiet) return
        println("[${cyan(name)}][$elapsed] $m")
    }

    abstract fun solveFirst(input: File): String
    abstract fun solveSecond(input: File): String

    abstract val secondUsesSameInput: Boolean

    companion object {
        fun getDay(day: Int): Result<Day> =
            ClassLoader.getSystemClassLoader()
                .runCatching { loadClass("days.Day${day}") }
                .map { it.getConstructor().newInstance() as Day }

        operator fun iterator(): Iterator<Day> =
            sequence {
                for (day in 1..24) {
                    val res = getDay(day)
                    if (res.isSuccess) yield(res.getOrThrow())
                    else break
                }
            }.iterator()

        fun getWorkRoot(): Path = Paths.get("").toAbsolutePath()
    }
}

enum class Part {
    First,
    Second;

    override fun toString(): String = super.toString().lowercase()
}

