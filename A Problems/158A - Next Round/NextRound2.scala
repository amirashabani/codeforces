// http://codeforces.com/problemset/problem/158/A

import scala.io.StdIn.readLine

object NextRound2 {
    def main(args: Array[String]): Unit = {
        val Array(n, k): Array[Int] = readLine().split(" ").map(_.toInt)
        val scores: Array[Int] = readLine().split(" ").map(_.toInt)

        var advances: Int = 0
        val kth: Int = scores(k-1)

        for(score <- scores) {
            if(score >= kth && score > 0) {
                advances += 1
            }
        }

        println(advances)
    }
}