extern mod std;
use json = std::json;
use Json = std::json::Json;
use List = std::json::List;
use Number = std::json::Number;
struct Point {
	x: float,
	y: float
}

fn getSquareDistance(p1: Point, p2: Point) -> float { 
	let dx : float = p1.x - p2.x;
	let dy : float = p1.y - p2.y;
	return dx * dx + dy * dy;
}
	
fn getSquareSegmentDistance(p: Point, p1: Point, p2: Point) -> float {
	let mut dxy = Point { x: p2.x - p1.x, y: p2.y-p1.x};
	let mut xy : Point = Point { x: p1.x, y: p1.y};
	 if dxy.x != 0.0 && dxy.y != 0.0 {
		let t : float =((p.x - p1.x) * dxy.x + (p.y - p1.y) * dxy.y) / (dxy.y * dxy.y + dxy.y * dxy.y);
		if t>1.0 {
			 xy = p2;
		}else if t>0.0 {
			 xy = Point { x: dxy.x * t + p1.x,y: dxy.y * t + p1.y};
		}else{
			 xy  = Point { x: p1.x,y:  p1.y};
		}
	}
	dxy = Point {x: (p.x - xy.x), y: (p.y - xy.y)};
	return dxy.x * dxy.x + dxy.x * dxy.y;
}
fn simplifyRadialDistance(points:~[Point], sqTolerance:float) -> ~[Point]{ 
	let mut i : uint = 1u;
	let len : uint = points.len();
	let mut prevPoint : Point = points[0u];
	let mut newPoints : ~[Point] = ~[prevPoint];
	let mut point : Point = points[i];
	loop {
		if (getSquareDistance(point, prevPoint) > sqTolerance) {
			newPoints.push(point);
			prevPoint = point;
		}
		i+=1;
		if (i < len) {
			point = points[i];
		}else{
			break;
		}
	}
	if (prevPoint.x != point.x && prevPoint.y != point.y) {
		newPoints.push(point);
	}
	return newPoints;
}
fn simplifyDouglasPeucker(points : ~[Point], sqTolerance : float, hq:bool) -> ~[Point]{
	let pts:~[Point]=match hq {
		true=>points,
		_=>simplifyRadialDistance(points,sqTolerance)
	};
	let len : uint = vec::len(pts);
	io::println(fmt!("from:%?",len));
	let mut markers : ~[uint] = ~[0u, ..0x1000000];
	let mut first : uint = 0u;
	let mut last : uint = len - 1u;
	let mut firstStack : ~[uint] = ~[];
	let mut lastStack : ~[uint] = ~[];
	let mut newPoints : ~[Point] = ~[];
	markers[first] = 1u;
	markers[last] = 1u;
	loop {
		let mut maxSqDist : float = 0.0f;
		let mut i : uint = first + 1u;
		let mut index : uint = 0;
		while (i < last) {
			let sqDist :float  = getSquareSegmentDistance(
				pts[i], 
				pts[first], 
				pts[last]
			);
			if (sqDist > maxSqDist) {
				index = i;
				maxSqDist = sqDist;
			}
			i += 1;
		}
		if (maxSqDist > sqTolerance) {
			markers[index] = 1u;
			firstStack.push(first);
			lastStack.push(index);
			firstStack.push(index);
			lastStack.push(last);
		}
		if(firstStack.len()>0u && lastStack.len()>0u){
			first = firstStack.pop();
			last = lastStack.pop();
		}else{
			break;
		};
	};
	markers.eachi(|j,marker|{
		if(*marker==1u){
			newPoints.push(pts[j]);
		};
		true
	});
	return newPoints;
}
fn dealList(l:~[Json])->~[Point]{
	l.map(|b|{
		match *b{
			List([Number(x),Number(y)])=>Point{x:x,y:y},
			_=>Point{x:0.0,y:0.0}
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
	match json::from_reader(reader){
		Ok(points)=> io::println(fmt!("to %?",vec::len(simplifyDouglasPeucker(dealJson(points),0.8f,false)))),
		Err(e)=>io::println(fmt!("%?",e))
	}
}
