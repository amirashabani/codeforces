// http://codeforces.com/problemset/problem/71/A

import scala.io.StdIn.readInt
import scala.io.StdIn.readLine

object WayTooLongWords {
    def main(args: Array[String]): Unit = {
        val n: Int = readInt()
        for(i <- 1 to n) {
            var word = readLine()
            var len = word.length
            if(len > 10) {
                println(f"${word.substring(0, 1)}%s${len-2}%d${word.takeRight(1)}%s")
            } else {
                println(word)
            }
        }
    }
}
