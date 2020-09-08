// http://codeforces.com/problemset/problem/158/A

import scala.io.StdIn.readLine

object NextRound {
    def main(args: Array[String]): Unit = {
        val Array(n, k): Array[Int] = readLine().split(" ").map(_.toInt)
        val scores: Array[Int] = readLine().split(" ").map(_.toInt)

        var advances: Int = k
        val kth: Int = scores(k-1)

        for(i <- k until n) {
            var ith: Int = scores(i)
            if(ith >= kth && ith > 0) {
                advances += 1
            }
        }

        advances -= scores.slice(0, k).count(x => {x == 0})
        println(advances)
    }
}