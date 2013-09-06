#[link(name = "simplify", vers = "0.0.5")];
extern mod extra;
use extra::json::*;
use std::hashmap::HashMap;
use std::vec;
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
impl Sub<Point,Point> for Point {
    #[inline]
    fn sub(&self, other: &Point) -> Point { Point {x:self.x-other.x,y:self.y-other.y} }
}
impl Add<Point,Point> for Point {
    #[inline]
    fn add(&self, other: &Point) -> Point { Point { x:self.x+other.x,y:self.y+other.y }}
}
impl Mul<Point,Point> for Point {
    #[inline]
    fn mul(&self, other: &Point) -> Point { Point { x:self.x*other.x,y:self.y*other.y }}
}

impl Point {
    fn sum(self) -> float { self.x+self.y }
    fn sqsum(self) -> float { self.x * self.x + self.y * self.y}
    fn sub(self, other: float) -> Point { Point { x:self.x - other , y:self.y - other }}
    fn mul(self, other: float) -> Point { Point { x:self.x * other, y:self.y * other }}
    fn add(self, other: float) -> Point {  Point {x:self.x + other, y:self.y + other }}
}

fn calcStuff(p:Point,p1:Point,d1:Point)->float {
	let top : float = ((p - p1) * d1).sum();
	let bottom : float =  d1.sqsum();
	match bottom{
		0.0=>0.0,
		_=>top/bottom
	}
}
fn getSquareSegmentDistance(p: Point, p1: Point, p2: Point) -> float {
	let d1 : Point = p2-p1;
	let d2 : Point = match d1{
		Point {x:0.0,_} | Point {y:0.0,_}=> {p1}
		_=>{
			let t : float = calcStuff(p,p1,d1);
			match t {
				tt if tt>1.0=>p2,
				tt if tt>0.0=> d1.mul(tt)+p1,
				_=>p1
			}
		}
	};
	let d4 : Point = p-d2;
	d4.sqsum()
}
fn simplifyRadialDistance(points:~[Point], sqTolerance:float) -> ~[Point]{
    let mut it = points.iter();
	let mut prevPoint : Point = it.next().get();
	let mut newPoints : ~[Point] = ~[prevPoint];
	let last : Point = points.last();
	for point in it.filter(|point|(point - prevPoint).sqsum() > sqTolerance) {
			newPoints.push(point);
			prevPoint = point;
	}
	if (prevPoint!= last) {
		newPoints.push(last);
	}
	newPoints
}
fn simplifyDouglasPeucker(points : ~[Point], tolerance : float) -> ~[Point]{
	let len : uint = points.len();
	let mut markers = HashMap::new();
	let mut first : uint = 0u;
	let mut last : uint = len - 1u;
	let mut firstStack : ~[uint] = vec:: with_capacity(len);
	let mut lastStack : ~[uint] = vec:: with_capacity(len);
	markers.insert(first,1u);
	markers.insert(last,1u);
	let mut index : uint = 0;
	loop {
		let mut maxSqDist : float = 0.0f;
		let mut i : uint = first + 1u;
		while (i < last) {
			
			let sqDist :float  = getSquareSegmentDistance(
				points[i], 
				points[first], 
				points[last]
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
    vec::from_fn(markers.len(),|k| points[k])
}

pub fn simplify(points : ~[Point], sqTolerance : float, hq:bool) -> ~[Point]{
	let tolerance : float = sqTolerance*sqTolerance;
	let pts:~[Point]=match hq {
		true=>points,
		_=>simplifyRadialDistance(points,tolerance)
	};
	simplifyDouglasPeucker(pts,tolerance)
}
