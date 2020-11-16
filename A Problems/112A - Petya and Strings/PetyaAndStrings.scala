// http://codeforces.com/problemset/problem/112/A

import scala.io.StdIn.readLine

object PetyaAndStrings {
    def main(args: Array[String]): Unit = {
        val first: String = readLine().toLowerCase()
        val second: String = readLine().toLowerCase()

        var comparison: Int = 0

        if(first < second) {
            comparison = -1
        } else if(first > second) {
            comparison = 1
        }

        println(comparison)
    }
}