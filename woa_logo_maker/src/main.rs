use std::fs::write;
use clap::Parser; 


enum PathStep {
    MoveTo(Point),
    Arc(Arc),
}

enum SweepDirection {
    Clockwise,
    CounterClockwise,
}

enum ArcPiece {
    Large,
    Small,
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn midpoint(&self, other: &Point) -> Point {
        Point::new((self.x + other.x) / 2.0, (self.y + other.y) / 2.0)
    }

    fn slope_to(&self, other: &Point) -> Option<f64> {
        if other.x == self.x {
            return None;
        }
        Some((other.y - self.y) / (other.x - self.x))
    }

    fn to_the_right(&self, x_displacement: f64) -> Point {
        Point::new(self.x + x_displacement, self.y)
    }

    fn underneath(&self, y_displacement: f64) -> Point {
        Point::new(self.x, self.y + y_displacement)
    }

    fn svg_move_to(&self) -> String {
        format!("M {} {}", self.x, self.y)
    }

    fn d2_offset(&self, x: f64, y: f64) -> Point {
        Point::new(self.x + x, self.y + y)
    }

    fn clone(&self) -> Point {
        Point::new(self.x, self.y)
    }

    fn x_displacement(&self, other: &Point) -> f64 {
        other.x - self.x
    }
}



struct Arc {
    end: Point,
    radius: f64,
    piece: ArcPiece,
    sweep: SweepDirection,
}

impl Arc {
    fn new(end: Point, radius: f64, piece: ArcPiece, sweep: SweepDirection) -> Arc {
        Arc {
            end,
            radius,
            piece,
            sweep,
        }
    }

    fn to_svg(&self) -> String {
        let piece = match self.piece {
            ArcPiece::Large => "1",
            ArcPiece::Small => "0",
        };
        let sweep = match self.sweep {
            SweepDirection::Clockwise => "1",
            SweepDirection::CounterClockwise => "0",
        };
        format!(
            "A {} {} 0 {} {} {} {}",
            self.radius, self.radius, piece, sweep, self.end.x, self.end.y
        )
    }
}

struct Circle {
    center: Point,
    radius: f64,
    stroke_color: String,
    stroke_width: f64,
}

impl Circle {
    fn new(center: Point, radius: f64, stroke_color: String, stroke_width: f64) -> Circle {
        Circle { center, radius, stroke_color, stroke_width}
    }

    fn to_svg(&self) -> String {
        format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\"/>", self.center.x, self.center.y, self.radius, self.stroke_color, self.stroke_width)
    }
    
}

struct Path {
    d: String,
    stroke_color: String,
    stroke_width: f64,
}

impl Path {
    fn from_steps(steps: Vec<PathStep>, stroke_color: String, stroke_width: f64) -> Path {
        let mut d_bits: Vec<String> = Vec::new();
        for step in steps {
            match step {
                PathStep::MoveTo(point) => d_bits.push(point.svg_move_to()),
                PathStep::Arc(arc) => d_bits.push(arc.to_svg()),
            }
        }
        let d = d_bits.join(" ");
        Path { d, stroke_color, stroke_width }
    }
    
    fn to_svg(&self) -> String {
        format!("<path d=\"{}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\"/>", self.d, self.stroke_color, self.stroke_width)
    }
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}


fn radius_of_bottom_arc(bottom_left: &Point, center: &Point) -> f64 {
    let midpoint = bottom_left.midpoint(center);
    let perp_slope = match bottom_left.slope_to(center) {
        Some(slope) if slope == 0.0 => None,
        Some(slope) => Some(-1.0 / slope),
        None => Some(0.0),
    };
    let displacement = midpoint.x_displacement(center);
    // lower_left x + displacement = circum_center_x
    // lower_left y + displacement * perpSlope = circum_center_y
    match perp_slope {
        Some(perpSlope) => {
            let circum_center_x = bottom_left.x + displacement;
            let circum_center_y = bottom_left.y + displacement * perpSlope;
            distance(&midpoint, &Point::new(circum_center_x, circum_center_y))
        },
        None => distance(&midpoint, &Point::new(center.x, bottom_left.y))
    }
}   

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    padding: f64,
    width: f64,
    height: f64,
    bottom_gap_height: f64,
    bottom_gap_width: f64,
    circle_radius: f64,
    color: String,
    stroke_width: f64,
    wing_arc_radius: f64, 
    svg_file: String,
}

fn main() {
    let args: Args = Args::parse();

    let upper_left = Point::new(args.padding,args.padding);
    let upper_right = upper_left.to_the_right(args.width);
    let lower_left = upper_left.underneath(args.height);
    let lower_right = lower_left.to_the_right(args.width);
    let center = upper_left.midpoint(&lower_right);
    let bottom_mid = lower_left.midpoint(&lower_right);

    let a = distance(&lower_left, &center);
    let b = distance(&lower_right, &center);

    let bottom_arc_radius = radius_of_bottom_arc(&lower_left, &center);
    
    let left_gap = bottom_mid.d2_offset(-args.bottom_gap_width, -args.bottom_gap_height);
    let right_gap = bottom_mid.d2_offset(args.bottom_gap_width, -args.bottom_gap_height);

    let path_steps: Vec<PathStep> = vec![
        PathStep::MoveTo(upper_left.clone()),
        PathStep::Arc(Arc::new(left_gap, args.wing_arc_radius, ArcPiece::Small, SweepDirection::CounterClockwise)),
        PathStep::MoveTo(upper_right.clone()),
        PathStep::Arc(Arc::new(right_gap, args.wing_arc_radius, ArcPiece::Small, SweepDirection::Clockwise)),
        PathStep::MoveTo(lower_left),
        PathStep::Arc(Arc::new(lower_right, bottom_arc_radius, ArcPiece::Small, SweepDirection::Clockwise)),
    ];

    let path = Path::from_steps(path_steps, args.color.clone(), args.stroke_width);
    let circle = Circle::new(center, args.circle_radius, args.color, args.stroke_width);

    let mut lines_to_write: Vec<String> = Vec::new(); 
    lines_to_write.push("<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"40 45 180 140\">".to_string());
    lines_to_write.push(format!("  {}", circle.to_svg()));
    lines_to_write.push(format!("  {}", path.to_svg()));
    lines_to_write.push("</svg>".to_string());
    let to_write = lines_to_write.join("\n");
    write(args.svg_file, to_write).expect("Unable to write file");
}
