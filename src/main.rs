use dxf::{Drawing, Point, entities::*};
use serde_json::Value;
use std::fs;

struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }

    fn entities(&self, offset_x: f64, offset_y: f64) -> [Entity; 4] {
        let left = self.x + offset_x;
        let right = left + self.w;
        let top = self.y + offset_y;
        let bottom = top + self.h;
        [
            // Top
            Entity::new(EntityType::Line(Line::new(
                Point::new(left, top, 0.0),
                Point::new(right, top, 0.0),
            ))),
            // Right
            Entity::new(EntityType::Line(Line::new(
                Point::new(right, top, 0.0),
                Point::new(right, bottom, 0.0),
            ))),
            // Bottom
            Entity::new(EntityType::Line(Line::new(
                Point::new(left, bottom, 0.0),
                Point::new(right, bottom, 0.0),
            ))),
            // Left
            Entity::new(EntityType::Line(Line::new(
                Point::new(left, top, 0.0),
                Point::new(left, bottom, 0.0),
            ))),
        ]
    }
}

struct Key {
    // Rectangle identifying the margins
    margin: Rect,
    // Rectangle identifying the keycap
    cap: Rect,
    // Rectangle identifying the plate-mount hole
    hole: Rect,
}

impl Key {
    fn entities(&self, offset_x: f64, offset_y: f64) -> Vec<Entity> {
        let mut entities = Vec::with_capacity(12);
        // entities.extend_from_slice(
        //     &self.margin.entities(offset_x, offset_y)
        // );
        // entities.extend_from_slice(
        //     &self.cap.entities(offset_x, offset_y)
        // );
        entities.extend_from_slice(
            &self.hole.entities(offset_x, offset_y)
        );
        entities
    }
}

fn main() {
    let json = fs::read_to_string("res/ugly-dense.json").unwrap();
    let v: Value = serde_json::from_str(&json).unwrap();
    println!("{:#?}", v);

    // X switch
    // let key = Key {
    //     margin: Rect::new(0.0, 0.0, 19.0, 19.0),
    //     cap: Rect::new(1.25, 1.25, 16.5, 16.5),
    //     hole: Rect::new(2.5, 2.5, 14.0, 14.0),
    // };

    // Mini-choc switch
    // let key = Key {
    //     margin: Rect::new(0.0, 0.0, 19.0, 19.0),
    //     cap: Rect::new(0.75, 1.25, 17.5, 16.5),
    //     hole: Rect::new(2.575, 3.25, 13.85, 12.5),
    // };

    // MX full size switch
    let key = Key {
        margin: Rect::new(0.0, 0.0, 19.0, 19.0),
        cap: Rect::new(1.25, 1.25, 16.5, 16.5),
        hole: Rect::new(2.5, 2.5, 14.0, 14.0),
    };

    let mut drawing = Drawing::default();

    let mut x = 0.0;
    let mut y = 0.0;
    let mut w = 1.0;

    if let Value::Array(rows) = v {
        for row in rows {
            match row {
                Value::Array(cols) => {
                    for col in cols {
                        match col {
                            Value::Object(o) => {
                                println!("Key metadata {:?}", o);
                                if let Some(x_v) = o.get("x") {
                                    if let Value::Number(x_n) = x_v {
                                        if let Some(x_f) = x_n.as_f64() {
                                            x += x_f * key.margin.w;
                                        }
                                    }
                                }
                                if let Some(w_v) = o.get("w") {
                                    if let Value::Number(w_n) = w_v {
                                        if let Some(w_f) = w_n.as_f64() {
                                            w = w_f;
                                        }
                                    }
                                }
                            },
                            Value::String(s) => {
                                println!("Key {}, {} = {:?}", x, y, s);

                                x += key.margin.w * (w - 1.0) / 2.0;

                                drawing.entities.extend_from_slice(
                                    &key.entities(x, y)
                                );

                                x += key.margin.w * (w - 1.0) / 2.0;

                                x += key.margin.w;
                                w = 1.0;
                            }
                            _ => (),
                        }
                    }

                    x = 0.0;
                    y -= key.margin.h;
                },
                _ => (),
            }
        }
    }

    drawing.save_file("test.dxf").unwrap();
}
