use array2d::Array2D;

pub fn infill_poly(data: &mut Array2D<u8>, point_marker: u8) {
    // Use scanline algorithm with even-odd rule to fill polygon and handle holes
    let rows = data.num_rows();
    let cols = data.num_columns();

    for y in 0..rows {
        // Find all boundary crossings on this scanline
        let mut raw_crossings = Vec::new();

        for x in 0..cols {
            if *data.get(y, x).unwrap() == point_marker {
                raw_crossings.push(x);
            }
        }

        // Consolidate consecutive boundary pixels into pairs of crossings (start and end)
        let mut crossings = Vec::new();
        let mut i = 0;
        while i < raw_crossings.len() {
            let start = raw_crossings[i];
            let mut end = start;

            // Find the end of this consecutive sequence
            while i + 1 < raw_crossings.len() && raw_crossings[i + 1] == raw_crossings[i] + 1 {
                i += 1;
                end = raw_crossings[i];
            }

            // For single pixels, add one crossing. For thick edges, add start and end
            if start == end {
                crossings.push(start);
            } else {
                crossings.push(start);
                crossings.push(end);
            }
            i += 1;
        }

        // Fill between pairs of crossings using even-odd rule
        for chunk in crossings.chunks(2) {
            if chunk.len() == 2 {
                let start = chunk[0];
                let end = chunk[1];

                // Fill the area between crossings (exclusive of boundaries)
                for x in (start + 1)..end {
                    data.set(y, x, point_marker).unwrap();
                }
            }
        }

        // Handle odd number of crossings - fill to the end
        if crossings.len() % 2 == 1 {
            if let Some(&last_crossing) = crossings.last() {
                for x in (last_crossing + 1)..cols {
                    data.set(y, x, point_marker).unwrap();
                }
            }
        }
    }
}

/// Given a grid that has points marked with `point_marker`,
/// Find these points and return a polygon defined by them.
pub fn find_polygon(data: &Array2D<u8>, point_marker: u8) -> geo::Polygon<f64> {
    // Walk the array, and find all the points that have point marker and return the polygon defined by those points
    let mut points = Vec::new();

    for y in 0..data.num_rows() {
        for x in 0..data.num_columns() {
            if *data.get(y, x).unwrap() == point_marker {
                points.push(geo::Coord {
                    x: y as f64,
                    y: x as f64,
                });
            }
        }
    }

    // Sort points using Graham scan approach for proper polygon ordering
    if points.len() > 2 {
        // Find the bottommost point (largest x), then rightmost if tied (largest y)
        let mut start = 0;
        for i in 1..points.len() {
            if points[i].x > points[start].x
                || (points[i].x == points[start].x && points[i].y > points[start].y)
            {
                start = i;
            }
        }

        // Move the start point to the beginning
        points.swap(0, start);
        let start_point = points[0];

        // Sort remaining points by polar angle with respect to start point
        points[1..].sort_by(|a, b| {
            let cross_product = (a.x - start_point.x) * (b.y - start_point.y)
                - (a.y - start_point.y) * (b.x - start_point.x);

            if cross_product == 0.0 {
                // Collinear points - sort by distance
                let dist_a = (a.x - start_point.x).powi(2) + (a.y - start_point.y).powi(2);
                let dist_b = (b.x - start_point.x).powi(2) + (b.y - start_point.y).powi(2);
                dist_a.partial_cmp(&dist_b).unwrap()
            } else {
                cross_product.partial_cmp(&0.0).unwrap().reverse()
            }
        });
    }

    // Create a polygon from the collected points
    geo::Polygon::new(geo::LineString::from(points), vec![])
}

#[cfg(test)]
mod tests {
    use crate::print_array;

    use super::*;

    #[test]
    fn test_infill_poly_simple_rectangle() {
        let mut data = Array2D::filled_with(b'.', 8, 8);

        // Create a rectangle outline
        for x in 1..6 {
            data.set(1, x, b'#').unwrap();
            data.set(5, x, b'#').unwrap();
        }
        for y in 1..6 {
            data.set(y, 1, b'#').unwrap();
            data.set(y, 6, b'#').unwrap();
        }
        print_array(&data);
        infill_poly(&mut data, b'#');
        print_array(&data);
        // Check that interior is filled
        for y in 1..4 {
            for x in 2..6 {
                assert_eq!(*data.get(y, x).unwrap(), b'#');
            }
        }

        // Check that exterior remains empty
        assert_eq!(*data.get(0, 0).unwrap(), b'.');
        assert_eq!(*data.get(4, 7).unwrap(), b'.');
    }
    #[test]
    fn test_infill_poly_interior() {
        let mut data = Array2D::filled_with(b'.', 16, 16);

        // Create a rectangle outline
        for x in 1..14 {
            data.set(1, x, b'#').unwrap();
            data.set(14, x, b'#').unwrap();
        }
        for y in 1..14 {
            data.set(y, 1, b'#').unwrap();
            data.set(y, 14, b'#').unwrap();
        }

        // And draw an inner box from 4,4 to 8,8
        // This should remain unfilled
        for x in 4..9 {
            data.set(4, x, b'#').unwrap();
            data.set(8, x, b'#').unwrap();
        }
        for y in 4..9 {
            data.set(y, 4, b'#').unwrap();
            data.set(y, 8, b'#').unwrap();
        }
        // Then colour in the inside
        print_array(&data);
        infill_poly(&mut data, b'#');
        print_array(&data);
        // Check that interior is filled
        for y in 2..4 {
            for x in 2..13 {
                assert_eq!(*data.get(y, x).unwrap(), b'#');
            }
        }

        // Check that exterior edges are empty
        for y in 0..16 {
            assert_eq!(*data.get(y, 0).unwrap(), b'.');
            assert_eq!(*data.get(y, 15).unwrap(), b'.');
        }
        for x in 0..16 {
            assert_eq!(*data.get(0, x).unwrap(), b'.');
            assert_eq!(*data.get(15, x).unwrap(), b'.');
        }
        // Assert the the inner box is empty
        // from 4,4 to 8,8
        for y in 5..7 {
            for x in 5..7 {
                assert_eq!(*data.get(y, x).unwrap(), b'.');
            }
        }
    }

    #[test]
    fn test_infill_poly_horizontal_line() {
        let mut data = Array2D::filled_with(0u8, 1, 5);

        // Set markers at positions 1 and 3
        data.set(0, 1, 2).unwrap();
        data.set(0, 3, 2).unwrap();

        infill_poly(&mut data, 2);

        // Check result
        assert_eq!(*data.get(0, 0).unwrap(), 0);
        assert_eq!(*data.get(0, 1).unwrap(), 2);
        assert_eq!(*data.get(0, 2).unwrap(), 2);
        assert_eq!(*data.get(0, 3).unwrap(), 2);
        assert_eq!(*data.get(0, 4).unwrap(), 0);
    }

    #[test]
    fn test_infill_poly_no_markers() {
        let mut data = Array2D::filled_with(0u8, 3, 3);

        infill_poly(&mut data, 5);

        // All should remain 0
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(*data.get(y, x).unwrap(), 0);
            }
        }
    }

    #[test]
    fn test_infill_poly_odd_number_of_markers() {
        let mut data = Array2D::filled_with(0u8, 1, 4);

        // Set single marker
        data.set(0, 1, 3).unwrap();

        infill_poly(&mut data, 3);

        // Everything after the marker should be filled
        assert_eq!(*data.get(0, 0).unwrap(), 0);
        assert_eq!(*data.get(0, 1).unwrap(), 3);
        assert_eq!(*data.get(0, 2).unwrap(), 3);
        assert_eq!(*data.get(0, 3).unwrap(), 3);
    }

    #[test]
    fn test_find_polygon() {
        let mut data = Array2D::filled_with(b'.', 10, 10);
        // Draw a triangle
        data.set(1, 1, b'#').unwrap();
        data.set(1, 5, b'#').unwrap();
        data.set(5, 3, b'#').unwrap();
        let polygon = find_polygon(&data, b'#');
        let expected_points = vec![(5, 3), (1, 5), (1, 1)];
        let mut poly_points: Vec<(u8, u8)> = polygon
            .exterior()
            .points()
            .map(|p| (p.x() as u8, p.y() as u8))
            .collect();
        // Remove the duplicate closing point that geo::LineString adds automatically
        if poly_points.len() > 1 && poly_points[0] == poly_points[poly_points.len() - 1] {
            poly_points.pop();
        }
        assert_eq!(poly_points, expected_points);
    }
    #[test]
    fn test_find_polygon_indent() {
        let mut data = Array2D::filled_with(b'.', 10, 10);
        // Draw a triangle, in indent
        data.set(1, 1, b'#').unwrap();
        data.set(1, 8, b'#').unwrap();
        data.set(3, 5, b'#').unwrap();
        data.set(8, 5, b'#').unwrap();
        let polygon = find_polygon(&data, b'#');
        for row in 0..data.num_rows() {
            for col in 0..data.num_columns() {
                let point = geo::Point::new(row as f64, col as f64);
                if geo::Contains::contains(&polygon, &point) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        let expected_points = vec![(8, 5), (1, 8), (3, 5), (1, 1)];
        let mut poly_points: Vec<(u8, u8)> = polygon
            .exterior()
            .points()
            .map(|p| (p.x() as u8, p.y() as u8))
            .collect();
        // Remove the duplicate closing point that geo::LineString adds automatically
        if poly_points.len() > 1 && poly_points[0] == poly_points[poly_points.len() - 1] {
            poly_points.pop();
        }
        assert_eq!(poly_points, expected_points);
    }
}
