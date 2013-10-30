extern mod extra;
use simplify::{Point,simplify};
use extra::json;
use extra::json::ToJson;
use extra::time::precise_time_s;
use std::path;
use std::os::args;
use std::io::{buffered_file_writer,read_whole_file_str};

mod simplify;
fn dealList(l:~[json::Json])->~[Point]{
    println(fmt!("from %?",l.len()));
	l.map(|b|{
		match *b{
			json::List([json::Number(x),json::Number(y)])=>Point{x:x,y:y},
			_=>Point{x:0.0,y:0.0}
		}
	})
}
fn dealJson (s:~str)->~[Point]{
	match json::from_str(s){
		Ok(j)=> match j{
		    json::List(l)=>dealList(l),
		    _=>~[Point{x:0.0,y:0.0}]
		   },
	    _=>~[Point{x:0.0,y:0.0}]
	}
}
fn writeOut ( j:~[Point] , outPath:~path::Path) {
    println(fmt!("to %?",j.len()));
	match buffered_file_writer(outPath) {
	    Ok(writer)=>j.to_json().to_writer(writer),
	    Err(e)=>println(fmt!("%?",e))
	}
	true;
}
fn main() {
    let args : ~[~str] = args();
	let reader = read_whole_file_str(~path::Path(args[1]));
	let outPath = ~path::Path(args[2]);
	let simp =from_str::<float>(args[3]).unwrap_or(1.0f);
	match reader{
		Ok(points)=> {
		let p :~[Point] = dealJson(points);
		let startT :float = precise_time_s();
		let out = simplify(p,simp,false);
		let endT : float =  precise_time_s();
		println!("time {} ms",(endT-startT)*1000f);
		 writeOut(out,outPath)
		 }
		Err(e)=>println(fmt!("%?",e))
	}
}
