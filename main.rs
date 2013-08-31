extern mod std;
extern mod simplify;
use simplify::*;
use std::json::*;
fn dealList(l:~[Json])->~[Point]{
	l.map(|b|{
		match *b{
			List([Number(x),Number(y)])=>Point{x:x,y:y},
			_=>::simplify::Point{x:0.0,y:0.0}
		}
	})
}

fn dealJson (j:Json)->~[Point]{
	match j{
		List(l)=> dealList(l),
	_=>~[Point{x:0.0,y:0.0}]
	}
}

fn main() {
	let reader = io::stdin();
	match from_reader(reader){
		Ok(points)=> io::println(fmt!("to %?",vec::len(simplifyDouglasPeucker(dealJson(points),0.8f,false)))),
		Err(e)=>io::println(fmt!("%?",e))
	}
}
