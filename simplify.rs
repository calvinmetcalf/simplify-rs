#[link(name = "simplify", vers = "0.0.3")];
extern mod std;
use std::json::*;
use std::smallintmap::*;
pub struct Point {
	x: float,
	y: float
}
impl ToJson for Point {
    fn to_json(&self) -> Json { List(~[self.x.to_json(),self.y.to_json()]) }
}
impl Eq for Point {
    fn eq(&self, other: &Point) -> bool { other.x == self.x && other.y == self.y }
    fn ne(&self, other: &Point) -> bool { other.x != self.x || other.y != self.y }
}

fn getSquareDistance(p1: Point, p2: Point) -> float { 
	let dx : float = p1.x - p2.x;
	let dy : float = p1.y - p2.y;
	return dx * dx + dy * dy;
}
fn calcStuff(p:Point,p1:Point,d1:Point)->float {
	let top : float = (p.x - p1.x) * d1.x + (p.y - p1.y) * d1.y;
	let bottom : float =  d1.y * d1.y + d1.y * d1.y;
	match bottom{
		0.0=>0.0,
		_=>top/bottom
	}
}
fn getSquareSegmentDistance(p: Point, p1: Point, p2: Point) -> float {
	let d1 = Point { x: p2.x - p1.x, y: p2.y-p1.x};
	let d2 : Point = match d1{
		Point {x:0.0,_}| Point {y:0.0,_}=> {p1}
		_=>{
			let t = calcStuff(p,p1,d1);
			match t {
				tt if tt>1.0=>p2,
				tt if tt>0.0=>Point { x: d1.x * tt + p1.x,y: d1.y * tt + p1.y},
				_=>p1
			}
		}
	};
	let d4 = Point {x: (p.x - d2.x), y: (p.y - d2.y)};
	return d4.x * d4.x + d4.y * d4.y;
}
fn simplifyRadialDistance(points:~[Point], sqTolerance:float) -> ~[Point]{ 
	let mut i : uint = 1u;
	let len : uint = points.len();
	let mut prevPoint : Point = points[0u];
	let mut newPoints : ~[Point] = ~[prevPoint];
	let mut point : Point = points[i];
	while i < len {
		point = points[i];
		i+=1;
		if (getSquareDistance(point, prevPoint) > sqTolerance) {
			newPoints.push(point);
			prevPoint = point;
		}
	}
	if (prevPoint!= point) {
		newPoints.push(point);
	}
	return newPoints;
}
pub fn simplifyDouglasPeucker(points : ~[Point], sqTolerance : float, hq:bool) -> ~[Point]{
	let tolerance : float = sqTolerance*sqTolerance;
	let pts:~[Point]=match hq {
		true=>points,
		_=>simplifyRadialDistance(points,tolerance)
	};
	io::println(fmt!("in2 %?",pts.len()));
	let len : uint = pts.len();
	let mut markers = SmallIntMap::new();
	let mut first : uint = 0u;
	let mut last : uint = len - 1u;
	let mut firstStack : ~[uint] = ~[];
	let mut lastStack : ~[uint] = ~[];
	let mut newPoints : ~[Point] = ~[];
	markers.insert(first,1u);
	markers.insert(last,1u);
	let mut index : uint = 0;
	loop {
		let mut maxSqDist : float = 0.0f;
		let mut i : uint = first + 1u;
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
		if (maxSqDist > tolerance) {
			markers.insert(index,1u);
			firstStack.push(first);
			lastStack.push(index);
			firstStack.push(index);
			lastStack.push(last);
		}
		if(lastStack.len()>0u){
			first = firstStack.pop();
			last = lastStack.pop();
		}else{
			break;
		};
	};
	markers.each_key(|j|{
		newPoints.push(pts[*j]);
		true
	});
	return newPoints;
}
