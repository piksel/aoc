import java.nio.file.Paths
import kotlin.io.path.div
import colors.*

fun main() {
    val day = 4
    val rootPath = Paths.get("") / "src" / "main"
    val srcFile = (rootPath / "template" / "DayXX.kt").toFile()
    val dstFile = (rootPath / "kotlin" / "days" / "Day${day}.kt").toFile()
    if (dstFile.exists()) {
        println(red("File already exists: ") + dstFile.absolutePath)
        return
    }

    println("Using template ${yellow(srcFile.absolutePath)}")

    val dd = day.toString()
    dstFile.createNewFile()
    dstFile.writer().apply {
        srcFile.useLines { lines ->
            lines.forEach {
                append(it.replace("XX", dd))
                appendLine()
            }
        }
        close()
    }

    println("Generated new file ${yellow(dstFile.absolutePath)} from template!")

}
