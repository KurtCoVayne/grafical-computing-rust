use crate::geometry::segment::Segment;

type OutCode = u8;

const INSIDE: OutCode = 0b0000;
const LEFT: OutCode = 0b0001;
const RIGHT: OutCode = 0b0010;
const BOTTOM: OutCode = 0b0100;
const TOP: OutCode = 0b1000;

#[derive(Clone, Copy, Debug)]
pub enum Acceptance {
    Accept,
    Reject,
    Clip,
}

fn compute_outcode(x: f64, y: f64, xmin: f64, ymin: f64, xmax: f64, ymax: f64) -> OutCode {
    let mut code = INSIDE; 

    if x < xmin {
        code |= LEFT;
    } else if x > xmax {
        code |= RIGHT;
    }
    if y < ymin {
        code |= BOTTOM;
    } else if y > ymax {
        code |= TOP;
    }

    code
}

pub fn cohen_sutherland_segment_clip(
    segment: &mut Segment,
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
) -> Acceptance {
    let x0 = &mut segment.p0.x;
    let y0 = &mut segment.p0.y;
    let x1 = &mut segment.p1.x;
    let y1 = &mut segment.p1.y;
    let mut outcode0 = compute_outcode(*x0, *y0, xmin, ymin, xmax, ymax);
    let mut outcode1 = compute_outcode(*x1, *y1 , xmin, ymin, xmax, ymax);
    let mut accept: Acceptance = Acceptance::Reject;

    loop {
        if (outcode0 | outcode1) == INSIDE {
            accept = match accept {
                Acceptance::Reject => Acceptance::Accept,
                _ => accept,
            };
            break;
        } else if (outcode0 & outcode1) != INSIDE {
            break;
        } else {
            let x: f64;
            let y: f64;

            let outcode_out = if outcode1 > outcode0 { outcode1 } else { outcode0 };

            if outcode_out & TOP == TOP {
                x = *x0 + (*x1 - *x0) * (ymax - *y0) / (*y1
                    - *y0);
                y = ymax;
            } else if outcode_out & BOTTOM == BOTTOM {
                x = *x0 + (*x1 - *x0) * (ymin - *y0) / (*y1
                    - *y0);
                y = ymin;
            } else if outcode_out & RIGHT == RIGHT {
                y = *y0 + (*y1 - *y0) * (xmax - *x0) / (*x1
                    - *x0);
                x = xmax;
            } else if outcode_out & LEFT == LEFT {
                y = *y0 + (*y1 - *y0) * (xmin - *x0) / (*x1
                    - *x0);
                x = xmin;
            } else {
                panic!("Unexpected outcode_out: {}", outcode_out);
            }

            if outcode_out == outcode0 {
                *x0 = x;
                *y0 = y;
                outcode0 = compute_outcode(*x0, *y0, xmin, ymin, xmax, ymax);
                accept = Acceptance::Clip;
            } else {
                *x1 = x;
                *y1 = y;
                outcode1 = compute_outcode(*x1, *y1, xmin, ymin, xmax, ymax);
                accept = Acceptance::Clip;
            }
        }
    }
    accept
}

pub fn liang_barsky_clipper(
    segment: &mut Segment,
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
) -> Acceptance {
    let x1 = segment.p0.x;
    let y1 = segment.p0.y;
    let x2 = segment.p1.x;
    let y2 = segment.p1.y;

    let p1 = -(x2 - x1);
    let p2 = -p1;
    let p3 = -(y2 - y1);
    let p4 = -p3;

    let q1 = x1 - xmin;
    let q2 = xmax - x1;
    let q3 = y1 - ymin;
    let q4 = ymax - y1;

    let mut posarr = vec![1.0];
    let mut negarr = vec![0.0];

    if (p1 == 0.0 && q1 < 0.0)
        || (p2 == 0.0 && q2 < 0.0)
        || (p3 == 0.0 && q3 < 0.0)
        || (p4 == 0.0 && q4 < 0.0)
    {
        return Acceptance::Reject;
    }
    if p1 != 0.0 {
        let r1 = q1 / p1;
        let r2 = q2 / p2;
        if p1 < 0.0 {
            negarr.push(r1);
            posarr.push(r2);
        } else {
            negarr.push(r2);
            posarr.push(r1);
        }
    }
    if p3 != 0.0 {
        let r3 = q3 / p3;
        let r4 = q4 / p4;
        if p3 < 0.0 {
            negarr.push(r3);
            posarr.push(r4);
        } else {
            negarr.push(r4);
            posarr.push(r3);
        }
    }

    let rn1 = negarr.iter().fold(f64::NEG_INFINITY, |acc, &x| acc.max(x));
    let rn2 = posarr.iter().fold(f64::INFINITY, |acc, &x| acc.min(x));

    if rn1 > rn2 {
        println!("Line is outside the clipping window!");
        return Acceptance::Reject;
    }

    if rn1 == 0.0 && rn2 == 1.0 {
        return Acceptance::Accept;
    }

    segment.p0.x = x1 + p2 * rn1;
    segment.p0.y = y1 + p4 * rn1;
    segment.p1.x = x1 + p2 * rn2;
    segment.p1.y = y1 + p4 * rn2;

    Acceptance::Clip
}


pub fn clip_line_copy(
    segment: &Segment,
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
) -> Option<(Segment, Acceptance)> {
    let mut seg = segment.clone();
    let accept = liang_barsky_clipper(&mut seg, xmin, ymin, xmax, ymax);
    Some((seg, accept))
}