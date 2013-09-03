extern mod std;
extern mod simplify;
use simplify::*;
use std::json::*;


fn dealList(l:~[Json])->~[Point]{
    io::println(fmt!("from %?",l.len()));
	l.map(|b|{
		match *b{
			List([Number(x),Number(y)])=>Point{x:x,y:y},
			_=>Point{x:0.0,y:0.0}
		}
	})
}
fn dealJson (s:~str)->~[Point]{
	match from_str(s){
		Ok(j)=> match j{
		    List(l)=>dealList(l),
		    _=>~[Point{x:0.0,y:0.0}]
		   },
	    _=>~[Point{x:0.0,y:0.0}]
	}
}
fn writeOut ( j:~[Point] , outPath:~path::Path) {
    io::println(fmt!("to %?",vec::len(j)));
	match io::buffered_file_writer(outPath) {
	    Ok(writer)=>to_writer(writer,~j.to_json()),
	    Err(e)=>io::println(fmt!("%?",e))
	}
	true;
}
fn main() {
    let args : ~[~str] = os::args();
	let reader = io::read_whole_file_str(~path::Path(args[1]));
	let outPath = ~path::Path(args[2]);
	let simp = match float::from_str(args[3]){
	    Some(s)=>s,
	    _=>1.0f
	};
	
	match reader{
		Ok(points)=>  writeOut(simplify(dealJson(points),simp,false),outPath),
		Err(e)=>io::println(fmt!("%?",e))
	}
}
