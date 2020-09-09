// http://codeforces.com/problemset/problem/118/A

import scala.io.StdIn.readLine

object StringTask {
    def main(args: Array[String]): Unit = {
        val vowels: Set[Char] = "aeiouy".toSet

        var line: String = readLine().toLowerCase()
        line = line.filterNot(vowels.contains(_))
        val dotted: String = line.mkString(".")
        println(f".${dotted}%s")
    }
}