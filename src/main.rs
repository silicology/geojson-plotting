use geojson::{GeoJson, Geometry, Value};
use plotters::{coord::Shift, prelude::*};

/// Process top-level GeoJSON Object
fn process_geojson(gj: &GeoJson, root: &DrawingArea<SVGBackend, Shift>) {
    match *gj {
        GeoJson::FeatureCollection(ref ctn) => {
            for feature in &ctn.features {
                if let Some(ref geom) = feature.geometry {
                    match_geometry(geom, root)
                }
            }
        }
        GeoJson::Feature(ref feature) => {
            if let Some(ref geom) = feature.geometry {
                match_geometry(geom, root)
            }
        }
        GeoJson::Geometry(ref geometry) => match_geometry(geometry, root),
    }
}

/// Process GeoJSON geometries
fn match_geometry(geom: &Geometry, root: &DrawingArea<SVGBackend, Shift>) {
    match geom.value {
        Value::Polygon(ref coordinates) => {
            println!("Matched a Polygon");
            draw_polygon(coordinates, root);
        }
        Value::MultiPolygon(ref coordinates) => {
            println!("Matched a MultiPolygon");
            for polygon in coordinates {
                draw_polygon(polygon, root);
            }
        }
        Value::GeometryCollection(ref gc) => {
            println!("Matched a GeometryCollection");
            for geometry in gc {
                match_geometry(geometry, root)
            }
        }
        Value::Point(ref coordinates) => {
            println!("Matched a Point");
            draw_point(coordinates, root);
        }
        Value::MultiPoint(ref coordinates) => {
            println!("Matched a MultiPoint");
            for point in coordinates {
                draw_point(point, root);
            }
        }
        Value::LineString(ref coordinates) => {
            println!("Matched a LineString");
            draw_linestring(coordinates, root);
        }
        Value::MultiLineString(ref coordinates) => {
            println!("Matched a MultiLineString");
            for linestring in coordinates {
                draw_linestring(linestring, root);
            }
        }
        _ => println!("Matched some other geometry"),
    }
}

/// Draw a Polygon
fn draw_polygon(coordinates: &Vec<Vec<Vec<f64>>>, root: &DrawingArea<SVGBackend, Shift>) {
    let polygon = coordinates[0]
        .iter()
        .map(|coord| {
            (
                (coord[0] * 100.0 + 512.0) as i32,
                (coord[1] * 100.0 + 384.0) as i32,
            )
        })
        .collect::<Vec<_>>();
    let style = ShapeStyle {
        color: GREEN.mix(0.1).to_rgba(),
        filled: true,
        stroke_width: 1,
    };
    root.draw(&Polygon::new(polygon, style)).unwrap();
}

/// Draw a Point
fn draw_point(coordinates: &Vec<f64>, root: &DrawingArea<SVGBackend, Shift>) {
    root.draw(&Circle::new(
        (
            (coordinates[0] * 100.0 + 512.0) as i32,
            (coordinates[1] * 100.0 + 384.0) as i32,
        ),
        5,
        ShapeStyle {
            color: YELLOW.to_rgba(),
            filled: true,
            stroke_width: 1,
        },
    ))
    .unwrap();
}

/// Draw a LineString
fn draw_linestring(coordinates: &Vec<Vec<f64>>, root: &DrawingArea<SVGBackend, Shift>) {
    let linestring = coordinates
        .iter()
        .map(|coord| {
            (
                (coord[0] * 100.0 + 512.0) as i32,
                (coord[1] * 100.0 + 384.0) as i32,
            )
        })
        .collect::<Vec<_>>();
    root.draw(&PathElement::new(linestring, &BLUE)).unwrap();
}

fn main() {
    let geojson_data = std::fs::read_to_string("data/data2.geojson").unwrap();
    let geojson = geojson_data.parse::<GeoJson>().unwrap();

    let root = SVGBackend::new("plot.svg", (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    process_geojson(&geojson, &root);

    let mut chart = ChartBuilder::on(&root)
        .caption("GeoJSON Map", ("sans-serif", 50).into_font())
        .build_cartesian_2d(-180.0..180.0, -90.0..90.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();
}
