// http://codeforces.com/problemset/problem/1/A

import scala.io.StdIn.readLine
import scala.math.ceil

object TheatreSquare {
    def main(args: Array[String]): Unit = {
        val Array(n, m, a): Array[Double] = readLine().split(" ").map(_.toDouble)
        val result: Long = ceil(n / a).asInstanceOf[Long] * ceil(m / a).asInstanceOf[Long]
        println(result)
    }
}