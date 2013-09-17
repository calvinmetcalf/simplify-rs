#[link(name = "simplify", vers = "0.0.6")];
extern mod extra;
use extra::json::*;
use extra::treemap::TreeSet;
use std::vec;
#[deriving(Clone, Eq)]
pub struct Point {
	x: float,
	y: float
}
impl ToJson for Point {
	fn to_json(&self) -> Json { List(~[self.x.to_json(),self.y.to_json()]) }
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
type Pair = (uint, uint);
fn calcStuff(p:Point,p1:Point,d1:Point)->float {
	let top = ((p - p1) * d1).sum();
	let bottom =  d1.sqsum();
	if bottom == 0.0 {
	    0.0
	}else{
	    top/bottom
	}
}
fn getSquareSegmentDistance(p: Point, p1: Point, p2: Point) -> float {
	let d1 = p2-p1;
	let d2 = match d1{
		Point {x:0.0,_} | Point {y:0.0,_}=> {p1}
		_=>{
			let t = calcStuff(p,p1,d1);
			if t>1.0 {
			    p2
			}else if t>0.0{
			    d1.mul(t)+p1
			}else{
			    p1
			}
		}
	};
	(p-d2).sqsum()
}

fn simplifyRadialDistance(points:~[Point], sqTolerance:float) -> ~[Point]{
	let mut it = points.iter();
	it.next();
	let mut prevPoint : Point = points[0u];
	let mut newPoints : ~[Point] = ~[prevPoint];
	let &last = points.last();
	for &point in it{
		if (point - prevPoint).sqsum() > sqTolerance {
			newPoints.push(point);
			prevPoint = point;
		}
	}
	if (prevPoint!= last) {
		newPoints.push(last);
	}
	newPoints
}

fn simplifyDouglasPeucker(points : ~[Point], tolerance : float) -> ~[Point]{
	let len = points.len();
	let mut markers = TreeSet::new();
	let mut stack : ~[Pair] = ~[];
	markers.insert(0u);
	markers.insert(len-1u);
	let mut pair:Pair = (0u,len-1u);
	loop {
        let first = pair.first();
        let second = pair.second();
        let (first_pt, second_pt) = (points[first], points[second]);
        let mut index = 0;
        let mut max_sq_dist = 0.0f;
        let i = first + 1u;
        for (i, &point_i) in points.slice_from(i)
            .iter()
            .enumerate()
            .map(|(new_i, point)| (i + new_i, point))
            .take_while(|&(i, _)| i < second) {
            let sq_dist = getSquareSegmentDistance(point_i, first_pt, second_pt);
            if (sq_dist > max_sq_dist) {
                index = i;
                max_sq_dist = sq_dist;
            }
        }
        if max_sq_dist > tolerance {
            markers.insert(index);
            stack.push((first,index));
            stack.push((index,second));
        }
        match stack.pop_opt() {
            Some(p)=>pair=p,
            None=>break
        };
    }
	vec::from_fn(markers.len(),|k| points[k])
}

pub fn simplify(points : ~[Point], sqTolerance : float, hq:bool) -> ~[Point]{
	let tolerance = sqTolerance*sqTolerance;
	let pts:~[Point] = if hq {
	    points
	} else {
	    simplifyRadialDistance(points,tolerance)
	};
	simplifyDouglasPeucker(pts,tolerance)
}
