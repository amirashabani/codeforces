// http://codeforces.com/problemset/problem/50/A

import scala.io.StdIn.readLine

object DominoPiling {
    def main(args: Array[String]): Unit = {
        val Array(m, n): Array[Int] = readLine().split(" ").map(_.toInt)
        val maxDominos: Int = (m * n) / 2

        println(maxDominos)
    }
}