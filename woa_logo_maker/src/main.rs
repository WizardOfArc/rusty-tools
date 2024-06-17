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

const NIK_PURPLE: &str = "#3F006A";
const DOUG_RED: &str = "#B3050D";
const AZI_BLUE: &str = "#0000FF";

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

struct Crescent {
    start: Point,
    end: Point,
    thickness: f64,
    radius: f64,
    color: String,
}


impl Crescent {
    fn new(start: Point, end: Point, thickness: f64, radius: f64, color: String) -> Crescent {
        Crescent { start, end, thickness, radius, color }
    }

    fn to_svg(&self) -> String {
        let thickness = self.thickness;
        let small_radius = self.radius;
        let large_radius = self.radius * thickness;
        let start = self.start.clone();
        let end = self.end.clone();
        let arc1 = Arc::new(end, small_radius, ArcPiece::Small, SweepDirection::Clockwise);
        let arc2 = Arc::new(start.clone(), large_radius, ArcPiece::Small, SweepDirection::CounterClockwise);
        let arc1_svg = arc1.to_svg();
        let arc2_svg = arc2.to_svg();
        let path_d = format!("M {} {} {} {}", &start.x, &start.y, arc1_svg, arc2_svg);
        format!(
            "<path d=\"{}\" stroke=\"{}\" stroke-width=\"1\" fill=\"{}\"/>",
            path_d, self.color, self.color)
    }
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn dot_at(center: &Point) -> Circle {
    Circle::new(center.clone(), 1.0, "black".to_string(), 1.0)
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    padding: f64,
    width: f64,
    height: f64,
    circle_radius: f64,
    circle_stroke_width: f64,
    wing_arc_radius: f64, 
    wing_arc_thickness: f64, 
    horizon_radius: f64, 
    horizon_thickness: f64, 
    vb_x: f64,
    vb_y: f64,
    vb_width: f64,
    vb_height: f64,
    svg_file: String,
}


fn main() {
    /*
      refactor this to write crescents instead of arcs
      crescent is defined as a closed path 
      M startpoint
      A smaller radius smaller radius 0 0 sweep endpoint
      A larger radius larger radius 0 0 reverse sweep startpoint
      a crescent has a start point, an end point, a thickness, and a curviness
      curviness is inverse to the radius of the component arc with the smaller radius
      thickness is the difference between the two radii

      crescent thickness ranges from 1 sliver to a semi-circle
      thickness can be defined bewteen a minumum near but not exactly 0 and a maximum near but not exactly PI/2
      the difference of the radius will cotangent of that number added to the curve defining radius
      difference = thickness.cos()/thickness.sin() (put some guardrails to defend against 0s)
      curviness ranges from almost flat to a semi-circular arc (where radius is half the width of the crescent)

      logo points
      UL UR
      LL LR
      Center
      Bottom Middle: Midpoint of LL and LR
      Left Quarter: Midpoint of LL and Bottom Middle
      Right Quarter: Midpoint of LR and Bottom Middle

      Circle centered at Center with radius circle_radius
      horizon from LL to LR small radius calculated with function
      large radius is small radius + horizon thickness factor
      left wing from UL to Left Quarter with small radius wing_arc_radius
      right wing from UR to Right Quarter with small radius wing_arc_radius
      large radius is small radius + wing thickness factor
     */
    let args: Args = Args::parse();
    println!("{:?}", args);
    let upper_left = Point::new(args.padding,args.padding);
    let upper_right = upper_left.to_the_right(args.width);
    let lower_left = upper_left.underneath(args.height);
    let lower_right = lower_left.to_the_right(args.width);
    let center = upper_left.midpoint(&lower_right);
    let bottom_mid = lower_left.midpoint(&lower_right);

    let a = distance(&lower_left, &center);
    let b = distance(&lower_right, &center);

    let circle = Circle::new(center, args.circle_radius, AZI_BLUE.to_string(), args.circle_stroke_width);
    let horizon = Crescent::new(
         lower_left.clone().midpoint(&bottom_mid),
        lower_right.clone().midpoint(&bottom_mid),
          args.horizon_thickness,
          args.horizon_radius,
            DOUG_RED.to_string()
    );
    let left_wing = Crescent::new(
         lower_left.clone().midpoint(&bottom_mid),
        upper_left.clone(),
          args.wing_arc_thickness.clone(),
           args.wing_arc_radius.clone(),
            NIK_PURPLE.to_string()
    );
    let right_wing = Crescent::new(
         upper_right.clone(),
        lower_right.clone().midpoint(&bottom_mid),
          args.wing_arc_thickness,
           args.wing_arc_radius,
            NIK_PURPLE.to_string());

    let mut lines_to_write: Vec<String> = Vec::new(); 
    lines_to_write.push(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"{} {} {} {}\">", args.vb_x, args.vb_y, args.vb_width, args.vb_height));
    lines_to_write.push(format!("  {}", circle.to_svg()));
    lines_to_write.push(format!("  {}", horizon.to_svg()));
    lines_to_write.push(format!("  {}", left_wing.to_svg()));
    lines_to_write.push(format!("  {}", right_wing.to_svg()));
    lines_to_write.push("</svg>".to_string());
    let to_write = lines_to_write.join("\n");
    write(args.svg_file, to_write).expect("Unable to write file");


}
