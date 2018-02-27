//! Linear algebra routines


use self::super::super::super::ops::QuircPoint;


pub fn line_intersect(p0: &QuircPoint, p1: &QuircPoint, q0: &QuircPoint, q1: &QuircPoint) -> Option<QuircPoint> {
    /* (a, b) is perpendicular to line p */
    let a = -(p1.y - p0.y);
    let b = p1.x - p0.x;

    /* (c, d) is perpendicular to line q */
    let c = -(q1.y - q0.y);
    let d = q1.x - q0.x;

    /* e and f are dot products of the respective vectors with p and q */
    let e = a * p1.x + b * p1.y;
    let f = c * q1.x + d * q1.y;

    /* Now we need to solve:
     *     [a b] [rx]   [e]
     *     [c d] [ry] = [f]
     *
     * We do this by inverting the matrix and applying it to (e, f):
     *       [ d -b] [e]   [rx]
     * 1/det [-c  a] [f] = [ry]
     * */
    let det = (a * d) - (b * c);

    if det == 0 {
        None
    } else {
        Some(QuircPoint {
            x: (d * e - b * f) / det,
            y: (-c * e + a * f) / det,
        })
    }
}

pub fn perspective_setup(rect: &[QuircPoint], w: f64, h: f64) -> [f64; 8] {
    let x0 = rect[0].x as f64;
    let y0 = rect[0].y as f64;
    let x1 = rect[1].x as f64;
    let y1 = rect[1].y as f64;
    let x2 = rect[2].x as f64;
    let y2 = rect[2].y as f64;
    let x3 = rect[3].x as f64;
    let y3 = rect[3].y as f64;

    let wden = w * (x2 * y3 - x3 * y2 + (x3 - x2) * y1 + x1 * (y2 - y3));
    let hden = h * (x2 * y3 + x1 * (y2 - y3) - x3 * y2 + (x3 - x2) * y1);

    let mut c = [0f64; 8];

    c[0] = (x1 * (x2 * y3 - x3 * y2) + x0 * (-x2 * y3 + x3 * y2 + (x2 - x3) * y1) + x1 * (x3 - x2) * y0) / wden;
    c[1] = -(x0 * (x2 * y3 + x1 * (y2 - y3) - x2 * y1) - x1 * x3 * y2 + x2 * x3 * y1 + (x1 * x3 - x2 * x3) * y0) / hden;
    c[2] = x0;
    c[3] = (y0 * (x1 * (y3 - y2) - x2 * y3 + x3 * y2) + y1 * (x2 * y3 - x3 * y2) + x0 * y1 * (y2 - y3)) / wden;
    c[4] = (x0 * (y1 * y3 - y2 * y3) + x1 * y2 * y3 - x2 * y1 * y3 + y0 * (x3 * y2 - x1 * y2 + (x2 - x3) * y1)) / hden;
    c[5] = y0;
    c[6] = (x1 * (y3 - y2) + x0 * (y2 - y3) + (x2 - x3) * y1 + (x3 - x2) * y0) / wden;
    c[7] = (-x2 * y3 + x1 * y3 + x3 * y2 + x0 * (y1 - y2) - x3 * y1 + (x2 - x1) * y0) / hden;

    c
}

pub fn perspective_map(c: &[f64], u: f64, v: f64) -> QuircPoint {
    assert!(c.len() >= 8);

    let den = c[6] * u + c[7] * v + 1.0;
    let x = (c[0] * u + c[1] * v + c[2]) / den;
    let y = (c[3] * u + c[4] * v + c[5]) / den;

    QuircPoint {
        x: x.round() as isize,
        y: y.round() as isize,
    }
}

pub fn perspective_unmap(c: &[f64], in_p: &QuircPoint) -> (f64, f64) {
    let x = in_p.x as f64;
    let y = in_p.y as f64;
    let den = -c[0] * c[7] * y + c[1] * c[6] * y + (c[3] * c[7] - c[4] * c[6]) * x + c[0] * c[4] - c[1] * c[3];

    let u = -(c[1] * (y - c[5]) - c[2] * c[7] * y + (c[5] * c[7] - c[4]) * x + c[2] * c[4]) / den;
    let v = (c[0] * (y - c[5]) - c[2] * c[6] * y + (c[5] * c[6] - c[3]) * x + c[2] * c[3]) / den;

    (u, v)
}
