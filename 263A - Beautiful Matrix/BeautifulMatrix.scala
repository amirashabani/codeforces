// http://codeforces.com/problemset/problem/263/A

import scala.io.StdIn.readLine

object BeautifulMatrix {
    def main(args: Array[String]): Unit = {
        var one_row: Int = 0
        var one_column: Int = 0
        var minimum_moves: Int = 0

        var i: Int = 1
        var foundOne: Boolean = false
        while(i <= 5 && !foundOne) {
            var row: String = readLine().replace(" ", "")
            if(row.contains("1")) {
                one_row = i
                one_column = row.indexOf("1") + 1
                foundOne = true
            }
            i += 1
        }

        minimum_moves = (one_row - 3).abs + (one_column - 3).abs
        println(minimum_moves)
    }
}