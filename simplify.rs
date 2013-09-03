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
impl Sub<Point,Point> for Point {
    #[inline]
    fn sub(&self, other: &Point) -> Point { x:self.x-other.x,y:self.y-other.y }
}
impl Sub<Point,float> for Point {
    #[inline]
    fn sub(&self, other: &float) -> Point { x:self.x - *other , y:self.y - *other }
}
impl Add<Point,Point> for Point {
    #[inline]
    fn add(&self, other: &Point) -> Point { x:self.x+other.x,y:self.y+other.y }
}
impl Add<Point,float> for Point {
    #[inline]
    fn add(&self, other: &float) -> Point { x:self.x + *other, y:self.y + *other }
}
impl Mul<Point,Point> for Point {
    #[inline]
    fn mul(&self, other: &Point) -> Point { x:self.x*other.x,y:self.y*other.y }
}
impl Mul<Point,float> for Point {
    #[inline]
    fn mul(&self, other: &float) -> Point { x:self.x * *other, y:self.y * *other }
}
impl Point {
    fn sum(&self) -> Float { self.x+self.y }
    fn sqsum(&self) -> Float { self.x * self.x + self.y * self.y}
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
			let t = calcStuff(p,p1,d1);
			match t {
				tt if tt>1.0=>p2,
				tt if tt>0.0=> d1*tt+p1,
				_=>p1
			}
		}
	};
	let d4 : Point = p-d2;
	d4.sqsum()
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
		if (point - prevPoint).sqsum() > sqTolerance {
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
