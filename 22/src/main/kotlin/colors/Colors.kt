package colors

import java.util.Formattable

// Console colors! 🎉
fun cyan(v: Any, fmt: String = "%s") = "\u001B[1;36m${fmt}\u001B[0m".format(v)
// fun cyan(n: Int, radix: Int = 10) = cyan(n.toString(radix))
fun yellow(s: String) = "\u001B[1;33m${s}\u001B[0m"
fun green(s: String) = "\u001B[1;32m${s}\u001B[0m"
fun red(s: String) = "\u001B[1;31m${s}\u001B[0m"

fun String.rgb(hexColor: String?, background: Boolean = false): String =
    hexColor?.take(6)?.toIntOrNull(16)?.let {
        "\u001B[${if (background) 4 else 3}8;2;${it shr 16 and 0xff};${it shr 8 and 0xff};${it and 0xff}m"
    } ?: ""
