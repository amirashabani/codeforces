// http://codeforces.com/problemset/problem/339/A

import scala.io.StdIn.readLine

object HelpfulMaths {
    def main(args: Array[String]): Unit = {
        val line: String = readLine()
        val lineSorted: String = line.replace("+", "").toSeq.sorted.mkString("+")

        println(lineSorted)
    }
}