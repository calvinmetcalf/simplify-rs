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

fn main() {
	io::println(fmt!("%f",getSquareDistance(Point { x : 1.0, y: 1.0}, Point { x : 3.0, y : 3.0})));
}
