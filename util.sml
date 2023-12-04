datatype 'a iter = As of 'a * (unit -> 'a iter);
datatype 'a list = Ll of 'a * 'a list | Nil;

fun nats (n:int) = As(n, fn () => nats(n + 1));
val ns = nats(0);

fun take (As(st, _): 'a iter, 0:int) = Ll(st, Nil)
  | take (As(st, nd), n:int) = 
	  if n >= 0 
	  then Ll(st, take (nd(), n - 1)) 
	  else Ll(st, take (nd(), n + 1));
fun next (As(_, nd): 'a iter) = nd();

fun nth (As(st, _), 0:int) = st
  | nth (As(_, nd), n) = nth(nd(), n - 1);

fun map (As(st, nd): 'a iter, f: 'a -> 'b) = 
  As(f(st), fn () => map(nd(), f));

fun filter (As(st, nd), f) = 
  if f(st) 
  then As(st, fn () => filter(nd(), f)) 
  else filter(nd(), f);

fun intfromstr(str) = valOf(Int.fromString str);
