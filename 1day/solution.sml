use "/home/etonit/advent-of-code2023/util.sml";

val data = TextIO.openIn "/home/etonit/advent-of-code2023/day1/input";
val data2 = TextIO.openIn "/home/etonit/advent-of-code2023/day1/input";


fun getFirstN ("") = 0
  | getFirstN (str) = case String.explode str of
                            #"0"::_ => 0
                          | #"1"::_ => 1
                          | #"2"::_ => 2
                          | #"3"::_ => 3
                          | #"4"::_ => 4
                          | #"5"::_ => 5
                          | #"6"::_ => 6
                          | #"7"::_ => 7
                          | #"8"::_ => 8
                          | #"9"::_ => 9
                          | #"z" :: #"e":: #"r":: #"o"::_ => 0 | #"o":: #"n":: #"e"::_ => 1
                          | #"t":: #"w":: #"o"::_ => 2
                          | #"t":: #"h":: #"r":: #"e":: #"e"::_ => 3
                          | #"f":: #"o":: #"u":: #"r"::_ => 4
                          | #"f":: #"i":: #"v":: #"e"::_ => 5
                          | #"s":: #"i":: #"x"::_ => 6
                          | #"s":: #"e":: #"v":: #"e":: #"n"::_ => 7
                          | #"e":: #"i":: #"g":: #"h":: #"t"::_ => 8
                          | #"n":: #"i":: #"n":: #"e"::_ => 9
                          | _ => getFirstN (String.extract (str, 1, NONE));
fun getLastN ("", last) = last
  | getLastN (str, last) = case String.explode str of
                            #"0"::_ => getLastN(String.extract(str, 1, NONE), 0)
                          | #"1"::_ => getLastN(String.extract(str, 1, NONE), 1)
                          | #"2"::_ => getLastN(String.extract(str, 1, NONE), 2)
                          | #"3"::_ => getLastN(String.extract(str, 1, NONE), 3)
                          | #"4"::_ => getLastN(String.extract(str, 1, NONE), 4)
                          | #"5"::_ => getLastN(String.extract(str, 1, NONE), 5)
                          | #"6"::_ => getLastN(String.extract(str, 1, NONE), 6)
                          | #"7"::_ => getLastN(String.extract(str, 1, NONE), 7)
                          | #"8"::_ => getLastN(String.extract(str, 1, NONE), 8)
                          | #"9"::_ => getLastN(String.extract(str, 1, NONE), 9)
                          | #"z":: #"e":: #"r":: #"o"::_ => getLastN(String.extract(str, 1, NONE), 0)
                          | #"o":: #"n":: #"e"::_ => getLastN(String.extract(str, 1, NONE), 1)
                          | #"t":: #"w":: #"o"::_ => getLastN(String.extract(str, 1, NONE), 2)
                          | #"t":: #"h":: #"r":: #"e":: #"e"::_ => getLastN(String.extract(str, 1, NONE), 3)
                          | #"f":: #"o":: #"u":: #"r"::_ => getLastN(String.extract(str, 1, NONE), 4)
                          | #"f":: #"i":: #"v":: #"e"::_ => getLastN(String.extract(str, 1, NONE), 5)
                          | #"s":: #"i":: #"x"::_ => getLastN(String.extract(str, 1, NONE), 6)
                          | #"s":: #"e":: #"v":: #"e":: #"n"::_ => getLastN(String.extract(str, 1, NONE), 7)
                          | #"e":: #"i":: #"g":: #"h":: #"t"::_ => getLastN(String.extract(str, 1, NONE), 8)
                          | #"n":: #"i":: #"n":: #"e"::_ => getLastN(String.extract(str, 1, NONE), 9)
                          | _ => getLastN (String.extract (str, 1, NONE), last)


fun recurser (SOME ll, total) = recurser(TextIO.inputLine data, total + (getFirstN ll) * 10 + (getLastN (ll, 0)))
  | recurser (NONE, total) = total;

fun test (SOME s, f) = f s
  | test (NONE, f) = f "asd2gqweq1";

val res = recurser (TextIO.inputLine data, 0);
