// http://codeforces.com/problemset/problem/4/A

import scala.io.StdIn.readInt

object Watermelon {
    def main(args: Array[String]): Unit = {
        val w = readInt()
        if (w % 2 == 0 && w > 2) {
            println("YES")
        } else {
            println("NO")
        }
    }
}