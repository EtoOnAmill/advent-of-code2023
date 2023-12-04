use "/home/etonit/advent-of-code2023/util.sml";

val input = TextIO.openIn "/home/etonit/advent-of-code2023/day2/input";
val filter = {red = 12, green = 13, blue = 14};


fun nextLine () = TextIO.inputLine input;

fun splitLine(line) = String.tokens 
  (fn #" " => true 
    | #":" => true 
    | #"," => true 
    | #";" => true 
    | #"\n" => true
    | _ => false) line;

fun idAndPulls ("Game"::id::pulls) = {id = intfromstr id, pulls = pulls}
  | idAndPulls _ = {id = 0, pulls = []};


fun maxColors (pulls) = let
  val rec maxForColors = 
    (fn {red, blue, green} => 
    (fn (n::"red"::rest) =>
    if red > (intfromstr n)
    then maxForColors {red = red, blue = blue, green = green} rest 
    else maxForColors {red = (intfromstr n), blue = blue, green = green} rest
  | (n::"blue"::rest) =>
      if blue > (intfromstr n)
      then maxForColors {red = red, blue = blue, green = green} rest
      else maxForColors {red = red, blue = (intfromstr n), green = green} rest
  | (n::"green"::rest) =>
      if green > (intfromstr n) 
      then maxForColors {red = red, blue = blue, green = green} rest
      else maxForColors {red = red, blue = blue, green = (intfromstr n)} rest
  | _ => {red = red, blue = blue, green = green}));
in
  maxForColors {red = 0, blue = 0, green = 0} pulls
end

fun checkValidity {red, blue, green} = (red <= #red filter) 
  andalso (blue <= #blue filter)
  andalso (green <= #green filter);



fun sumPowers (SOME line) tot = 
      let
        val {id, pulls} = idAndPulls (splitLine line);
        val maxPulls = maxColors pulls;
        val {red, green, blue} = maxPulls;
        val power = red * green * blue;
      in 
        sumPowers (nextLine()) (tot + power)
      end
  | sumPowers (NONE) tot = tot;
  
fun addValidId (SOME line) tot = 
      let
        val {id, pulls} = idAndPulls (splitLine line);
        val maxPulls = maxColors pulls;
        val isValid = checkValidity maxPulls;
      in 
        if isValid then addValidId (nextLine()) (tot + id) else addValidId (nextLine()) tot 
      end

  | addValidId (NONE) tot = tot;


val inputTest = TextIO.openIn "/home/etonit/advent-of-code2023/day2/input";
val d = valOf(TextIO.inputLine inputTest);
val dd = splitLine d;
val ddd = idAndPulls dd;
val ddd1 = maxColors (#pulls ddd);





(*val res = addValidId (nextLine()) 0;*)
val res2 = sumPowers (nextLine()) 0;









(*

fun gameId (line) = let
  val woutgame = if (String.size line) < 5 then "0: 0 red\n" else String.extract (line, 5, NONE);
  val rec getIdbuilder = 
    fn acc => (fn str => 
      case hd(String.explode str) of
           #":" => valOf(Int.fromString acc)
         | c => (getIdbuilder (acc ^ String.str c)) (String.extract (str, 1, NONE)));
in 
  getIdbuilder "" woutgame
end;


fun pulls (line) = 
  let val ret =case String.tokens (fn c => c = #":") line of
                          _::rr::_ => rr
                        | _ => "0 red\n";
  in ret end;
fun colors (line) = 
    (fn str => 
    String.tokens 
      (fn #"," => true 
        | #";" => true
        | #" " => true
        | #"\n" => true
        | _ => false) str) (pulls line);

(*parse n, check with corrseponding color and recurse with biggest*)

val rec sumIds:int -> string option -> int = fn tot => 
(fn (SOME line) => 
    let 
      val id = gameId line;
      val colors = maxColors line;
      val valid = checkValidity colors;
    in 
      if valid then sumIds (tot + id) (nextLine()) else sumIds tot (nextLine())
    end
  | NONE => tot);
  *)
