/*
 Copyright (c) 2012, Vladimir Agafonkin
 Simplify.js is a high-performance polyline simplification library
 mourner.github.com/simplify-js
*/

(function (global, undefined) {

	


	// to suit your point format, run search/replace for '.x' and '.y'
	// to switch to 3D, uncomment the lines in the next 2 functions
	// (configurability would draw significant performance overhead)

	function asmModule (stdlib, foreign, heap){
		"use asm";
		var abs = stdlib.Math.abs;
		var points = new stdlib.Float64Array(heap,0,(heap.byteLength)>>4);
		var out = new stdlib.Float64Array(heap,(heap.byteLength)/2);
		function getSquareDistance(p1x,p1y, p2x,p2y) { // square distance between 2 points
			//all doubles;
			p1x = +p1x;
			p1y = +p1y;
			p2x = +p2x;
			p2y = +p2y;
			var dx = 0.0,
				dy = 0.0;
			dx = p1x - p2x;
			dy = p1y - p2y;

			return dx * dx + dy * dy;
		}

		function getSquareSegmentDistance(px,py, x,y, p2x,p2y) { // square distance from a point to a segment
			x = +x;
			y = +y;
			p2x = +p2x;
			p2y = +p2y;
			var dx = 0.0,
				dy = 0.0,
				t = 0.0;
			dx = p2x - x;
			dy = p2y - y;
			

			if ((abs(dx)+abs(dy)) > 0.0) {

				t = ((px - x) * dx +
					 (py - y) * dy) /
						(dx * dx +
						 dy * dy);

				if (t > 1) {
					x = p2x;
					y = p2y;

				} else if (t > 0) {
					x += dx * t;
					y += dy * t;
				}
			}

			dx = px - x;
			dy = py - y;

			return dx * dx +
				   dy * dy;
		
		}

		// the rest of the code doesn't care for the point format


		function simplifyRadialDistance(sqTolerance) { // distance-based simplification
			sqTolerance = +sqTolerance;
			var i = 2,
				j = 0,
				len = 0,
				pointx = 0.0,
				pointy = 0.0,
				prevPointx = 0.0,
				prevPointy = 0.0,
				len = points.length;
				prevPointx = points[0];
				prevPointy = points[1];

			while(i < len) {
				pointx = points[i];
				pointy = points[i+1];

				if (getSquareDistance(pointx,pointy, prevPointx,prevPointy) > sqTolerance) {
					out[j]=pointx;
					out[j+1]=pointy;
					prevPointx = pointx;
					prevPointy = pointy;
					j = j +2;
				}
				i = i + 2;
			}

			out[j]=pointx;
			out[j+1]=pointy;

			return j+1;
		}


	/* simplification using optimized Douglas-Peucker algorithm with recursion elimination

		function simplifyDouglasPeucker(sqTolerance) {
			
			var len = points.length,

				MarkerArray = (typeof Uint8Array !== undefined + '')
							? Uint8Array
							: Array,

				markers = new MarkerArray(len),

				first = 0,
				last  = len - 1,

				i,
				maxSqDist,
				sqDist,
				index,

				firstStack = [],
				lastStack  = [],

				newPoints  = [];

			markers[first] = markers[last] = 1;

			while (last) {

				maxSqDist = 0;

				for (i = first + 1; i < last; i++) {
					sqDist = getSquareSegmentDistance(points[i], points[first], points[last]);
	
					if (sqDist > maxSqDist) {
						index = i;
						maxSqDist = sqDist;
					}
				}

				if (maxSqDist > sqTolerance) {
					markers[index] = 1;

					firstStack.push(first);
					lastStack.push(index);

					firstStack.push(index);
					lastStack.push(last);
				}

				first = firstStack.pop();
				last = lastStack.pop();
			}

			for (i = 0; i < len; i++) {
				if (markers[i]) {
					newPoints.push(points[i]);
				}
			}

			return newPoints;
		}
		*/
		return {simplifyRadialDistance:simplifyRadialDistance/*, simplifyDouglasPeucker:simplifyDouglasPeucker*/};
}
	var root = (typeof exports !== undefined + '')
			 ? exports
			 : global;
	root.simplify = function (points, tolerance, highestQuality) {
		highestQuality=0;
		var array = (new Float64Array(points.length*2));
		array.set(points);
		var buffer = array.buffer;
		var asm = asmModule(window,{},buffer);
		var sqTolerance = (tolerance !== undefined)
						? tolerance * tolerance
						: 1;
		var outlen = highestQuality?asm.simplifyDouglasPeucker(sqTolerance,points.length):asm.simplifyRadialDistance(sqTolerance,points.length);
		return new Float64Array(buffer,(buffer.byteLength)/2,outlen);
};


}(this));