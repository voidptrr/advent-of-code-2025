use crate::file;

struct Point {
    col: usize,
    row: usize,
}

struct Rectangle<'a> {
    min_point: &'a Point,
    max_point: &'a Point,
}

impl<'a> Rectangle<'a> {
    fn area(&self) -> usize {
        let height = self.max_row().abs_diff(self.min_row()) + 1;
        let width = self.max_col().abs_diff(self.min_col()) + 1;
        width * height
    }

    fn min_row(&self) -> usize {
        self.min_point.row.min(self.max_point.row)
    }
    fn max_row(&self) -> usize {
        self.min_point.row.max(self.max_point.row)
    }
    fn min_col(&self) -> usize {
        self.min_point.col.min(self.max_point.col)
    }
    fn max_col(&self) -> usize {
        self.min_point.col.max(self.max_point.col)
    }

    fn overlaps(&self, other: &Rectangle) -> bool {
        // only interior overlap counts
        !(self.max_row() <= other.min_row()
            || other.max_row() <= self.min_row()
            || self.max_col() <= other.min_col()
            || other.max_col() <= self.min_col())
    }
}

impl<'a> From<(&'a Point, &'a Point)> for Rectangle<'a> {
    fn from(value: (&'a Point, &'a Point)) -> Self {
        Rectangle {
            min_point: value.0,
            max_point: value.1,
        }
    }
}

fn parse_coords<T: AsRef<str>>(input: &[T]) -> Vec<Point> {
    let mut points_vec: Vec<Point> = Vec::with_capacity(input.len());
    for line in input {
        let line_ref = line.as_ref();
        let (i, j) = line_ref.split_once(",").unwrap();

        let ni = i.parse::<usize>().unwrap();
        let ji = j.parse::<usize>().unwrap();
        points_vec.push(Point { col: ni, row: ji });
    }
    points_vec
}

fn solve_part_two(coords: &[Point]) -> usize {
    let rectangles: Vec<Rectangle> = coords
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            coords
                .iter()
                .skip(i + 1)
                .map(move |p2| Rectangle::from((p1, p2)))
        })
        .collect();

    let edges: Vec<Rectangle> = coords
        .iter()
        .enumerate()
        .map(|(i, p1)| {
            let p2 = &coords[(i + 1) % coords.len()];
            Rectangle::from((p1, p2))
        })
        .collect();

    

    rectangles
        .iter()
        .filter(|r| edges.iter().all(|edge| !r.overlaps(edge)))
        .map(|r| r.area())
        .max()
        .unwrap_or(0)
}

fn solve_part_one(coords: &[Point]) -> usize {
    let rects = coords.iter().enumerate().flat_map(|(i, coord_i)| {
        coords
            .iter()
            .enumerate()
            .skip(i + 1)
            .map(move |(_, coord_j)| Rectangle::from((coord_i, coord_j)))
    });

    rects.map(|r| r.area()).max().unwrap()
}

pub fn solve() {
    let lines = file::read_lines("days/9.txt");
    match lines {
        Ok(input) => {
            let to_vec_lines: Vec<String> = input.map_while(Result::ok).collect();
            let coords = parse_coords(&to_vec_lines);
            println!("day_nine [1] => {}", solve_part_one(&coords));
            println!("day_nine [2] => {}", solve_part_two(&coords));
        }
        _ => panic!("Error reading file"),
    }
}
