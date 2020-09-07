// http://codeforces.com/problemset/problem/231/A

import scala.io.StdIn.readInt
import scala.io.StdIn.readLine

object Team {
    def main(args: Array[String]): Unit = {
        val n: Int = readInt()
        var count: Int = 0
        for(i <- 1 to n) {
            if (readLine().split(" ").map(_.toInt).sum >= 2) {
                count += 1
            }
        }
        println(count)
    }
}