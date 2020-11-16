// http://codeforces.com/problemset/problem/282/A

import scala.io.StdIn.readInt
import scala.io.StdIn.readLine

object BitPlusPlus {
    def main(args: Array[String]): Unit = {
        val n: Int = readInt()
        var x: Int = 0
        for(i <- 1 to n) {
            var line: String = readLine()
            if(line.contains("+")) {
                x += 1
            } else if(line.contains("-")) {
                x -= 1
            }
        }

        println(x)
    }
}