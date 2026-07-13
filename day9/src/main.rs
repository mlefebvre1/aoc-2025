use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s.split(",");
        let x = ss.next().expect("X not present.").parse()?;
        let y = ss.next().expect("Y not present.").parse()?;
        Ok(Self { x, y })
    }
}

struct Edge<'a> {
    p1: &'a Point,
    p2: &'a Point,
}

impl<'a> Edge<'a> {
    fn new(p1: &'a Point, p2: &'a Point) -> Self {
        Self { p1, p2 }
    }

    fn intersection(&self, other: &Self) -> Option<Point> {
        // Not the full intersection implementation, but since we only have horizontal and vertical lines, we can just consider those
        match (self.p1.x == self.p2.x, other.p1.x == other.p2.x) {
            (true, true) => None, // both lines are vertical, no intersection is possible or it intersects at infinite points, we don't care about that.
            (false, false) => None, // both lines are horizontal, no intersection is possible or it intersects at infinite points, we don't care about that.
            (true, false) => Some(Point {
                x: self.p1.x,
                y: other.p1.y,
            }), // self is vertical, other is horizontal
            (false, true) => Some(Point {
                x: other.p1.x,
                y: self.p1.y,
            }), // self is horizontal, other is vertical
        }
    }

    fn is_inside(&self, point: &Point, reference: &Point) -> bool {
        // half plane test: check if the point is on the same side of the edge as the reference point
        let cross = |p: &Point| {
            (self.p2.x - self.p1.x) * (p.y - self.p1.y)
                - (self.p2.y - self.p1.y) * (p.x - self.p1.x)
        };
        let side_point = cross(point);
        let side_ref = cross(reference);
        side_point.signum() == side_ref.signum() || side_point == 0
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Point>> {
    let points = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<Point>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(points)
}

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(9)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let points = &parse_input(input)?;

    let ans = points
        .iter()
        .flat_map(|p1| {
            points.iter().map(move |p2| {
                let area = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
                area
            })
        })
        .max()
        .expect("failed..");

    Ok(ans.to_string())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let points = &parse_input(input)?;
    let main_poly = RectilinearPolygon::new(points.clone());

    let mut candidates = points
        .iter()
        .flat_map(|p1| {
            points.iter().map(move |p2| {
                let area = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
                (p1, p2, area)
            })
        })
        .collect::<Vec<_>>();

    // sort by area
    candidates.sort_by(|a, b| a.2.cmp(&b.2));

    let (_, _, ans) = candidates
        .iter()
        .rev() // ensure we start with the largest area first, so we can find the first one that matches
        .find(|(p1, p2, rect_area)| {
            let clip_poly = RectilinearPolygon::rect(p1, p2);
            let clipped = main_poly.clip(&clip_poly);
            let clipped_area = clipped.area();
            *rect_area == clipped_area
        })
        .expect("failed to find the solution..");

    Ok(ans.to_string())
}

struct RectilinearPolygon {
    points: Vec<Point>,
}
impl RectilinearPolygon {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
    fn rect(p1: &Point, p2: &Point) -> Self {
        let points = vec![
            p1.clone(),
            Point { x: p1.x, y: p2.y },
            p2.clone(),
            Point { x: p2.x, y: p1.y },
        ];
        Self::new(points)
    }
    fn ref_point(&self) -> Point {
        let min_x = self.points.iter().map(|p| p.x).min().unwrap();
        let max_x = self.points.iter().map(|p| p.x).max().unwrap();
        let min_y = self.points.iter().map(|p| p.y).min().unwrap();
        let max_y = self.points.iter().map(|p| p.y).max().unwrap();
        Point {
            x: (min_x + max_x) / 2,
            y: (min_y + max_y) / 2,
        }
    }

    fn clip(&self, clip_poly: &Self) -> Self {
        let mut output_list = self.points.clone();

        let ref_point = clip_poly.ref_point();

        for c in 0..clip_poly.points.len() {
            let c1 = &clip_poly.points[c];
            let c2 = &clip_poly.points[(clip_poly.points.len() - 1 + c) % clip_poly.points.len()];
            let clip_edge = Edge::new(c1, c2);

            let input_list = output_list.clone();
            output_list.clear();

            for i in 0..input_list.len() {
                let cur = &input_list[i];
                let prev = &input_list[(input_list.len() - 1 + i) % input_list.len()];
                let edge = Edge::new(cur, prev);
                let intersect_point = edge.intersection(&clip_edge);
                if clip_edge.is_inside(cur, &ref_point) {
                    if !clip_edge.is_inside(prev, &ref_point) {
                        if let Some(intersect_point) = intersect_point {
                            output_list.push(intersect_point);
                        }
                    }
                    output_list.push(cur.clone());
                } else if clip_edge.is_inside(prev, &ref_point) {
                    if let Some(intersect_point) = intersect_point {
                        output_list.push(intersect_point);
                    }
                }
            }
        }

        Self::new(output_list)
    }

    fn area(&self) -> i64 {
        // modified shoelace formula to calculate the area of a polygon and also include the boundary length
        let p = &self.points;
        let mut shoelace = 0;
        let mut boundary = 0;
        for i in 0..p.len() {
            let next_i = (i + 1) % p.len();
            shoelace += (p[i].x * p[next_i].y) - (p[next_i].x * p[i].y);
            // only true for rectilinear polygons, but our polygons are always rectilinear
            // either horizontal or vertical edges, so we can just add the length of the edge to the boundary length
            boundary += (p[next_i].x - p[i].x).abs() + (p[next_i].y - p[i].y).abs();
        }
        let shoelace = (shoelace / 2).abs();
        shoelace + (boundary / 2) + 1 // Once again, only true for rectilinear polygons
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "50");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "24");
    }
}
