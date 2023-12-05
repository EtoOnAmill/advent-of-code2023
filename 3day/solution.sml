val dummydata = "467..114..\n\
\...*......\n\
\..35..633.\n\
\......#...\n\
\617*......\n\
\.....+.58.\n\
\..592.....\n\
\......755.\n\
\...$.*....\n\
\.664.598..";

val input = TextIO.openIn "/home/etonit/advent-of-code2023/3day/input";

val readLine = fn () => TextIO.inputLine input;

fun flines(SOME line)= line::(flines (readLine()))
  | flines NONE = [];

val lines = flines (readLine());

val matrix = List.map (fn s => String.explode s) (lines);

type NumberData = 
  { x:int
  , y:int
  , charsLenght:int };

(* 
* traverse line keeping x and y 
* when encountering digit see if curr x minus the x of the last number + the lenght
* is different from 0
*   if it is add new digit
*   if it isn't increase last number lenght by 1
*)

fun lineCrawler ((chr::rest):char list) ((lastNumber::other):(NumberData list)) ((xx, yy):(int*int)) =
let 
  val isN = Char.isDigit chr;
  val {x, charsLenght, y} = lastNumber;
  val isOfLastN = ((x + charsLenght - xx) = 0) andalso (y = yy);
  val nextCoords = (xx + 1, yy);
  val numberDataExpand = {x = x, charsLenght = charsLenght + 1, y = y}::other;
  val numberDataNew = {x = xx, charsLenght = 1, y = yy}::lastNumber::other;
  val numberDataSame = lastNumber::other;
in
  if isOfLastN andalso isN 
  then lineCrawler rest numberDataExpand nextCoords
  else 
    if isN 
    then lineCrawler rest numberDataNew nextCoords
    else lineCrawler rest numberDataSame nextCoords
end
  | lineCrawler ((chr::rest):char list) ([]:(NumberData list)) ((xx, yy):(int*int)) =
let 
  val isN = Char.isDigit chr;
  val nextCoords = (xx + 1, yy);
  val numberDataNew = {x = xx, charsLenght = 1, y = yy}::[];
in
  if isN 
  then lineCrawler rest numberDataNew nextCoords
  else lineCrawler rest [] nextCoords
end

  | lineCrawler [] nData coords = nData;

val lcTest = lineCrawler (hd matrix) [] (0, 0);

fun getNumberData (line::rest) currY nData = 
let 
  val currLineNdata = (lineCrawler line nData (0, currY));
in getNumberData rest (currY + 1) currLineNdata end
  | getNumberData [] currY nData = nData;


val numberData = getNumberData matrix 0 [];

type SymbolData = 
  { y:int
  , x:int };

fun symCrawler (c::rest) sData (x, y) = 
let 
  val isSym = (Char.isPunct c) andalso (c <> #".");
  val symData = {x = x, y = y};
  val coords = (x + 1, y);
in 
  if isSym 
  then symCrawler rest (symData::sData) coords
  else symCrawler rest sData coords
end
  | symCrawler [] sData _ = sData;


val scTest = symCrawler (hd matrix) [] (0, 0);

fun getSymData (line::rest) currY sData = 
let 
  val currLineSdata = (symCrawler line sData (0, currY));
in getSymData rest (currY + 1) currLineSdata end
  | getSymData [] currY sData = sData;


val symData = getSymData matrix 0 [];


val maxNumberLen = 
  List.foldr 
  (fn ({x, y, charsLenght}, acc) => if acc > charsLenght then acc else charsLenght)
  0
  numberData;

fun inrange n start eng = (n <= eng) andalso (n >= start);

fun checkForSymbols sList {x = nx, y = ny, charsLenght} =
let 
  val filtered = 
    List.filter 
    (fn {x, y} => 
      (inrange x (nx - 1) (nx + charsLenght) ) andalso
      (inrange y (ny - 1) (ny + 1)))
    sList;
in (List.length filtered) <> 0 end;



val validNumbers = 
  let 
    val validMetaN = 
      List.filter
      (fn n => checkForSymbols symData n)
      numberData;
    val validN =
      List.map
      (*slice from #y lines a slice nx + charsLenght and turn to int*)
      (fn {y, x, charsLenght} => 
        valOf(Int.fromString (String.substring(List.nth(lines, y), x, charsLenght))))
      validMetaN;
  in validN end;

val res = List.foldr (fn (e, acc) => e + acc) 0 validNumbers;

fun checkForGears nList {x = sx, y = sy} =
let 
  val filtered = 
    List.filter 
    (fn {x, y, charsLenght} => 
      (inrange sx (x - 1) (x + charsLenght) ) andalso
      (inrange sy (y - 1) (y + 1)))
    nList;
  val symbol = String.sub(List.nth(lines, sy), sx);
in if symbol = #"*" then filtered else [] end;

val res1 = List.map (fn sym => checkForGears numberData sym) symData;
val res2 = List.filter (fn ll => (List.length ll) = 2) res1;
val res3 = List.map 
      (fn {y, x, charsLenght}::{y = yy, x = xx, charsLenght = cL}::_ => 
        valOf(Int.fromString (String.substring(List.nth(lines, y), x, charsLenght))) * 
        valOf(Int.fromString (String.substring(List.nth(lines, yy), xx, cL)))
        )
      res2;
val res4 = List.foldr (fn (e, acc) => e + acc) 0 res3;
