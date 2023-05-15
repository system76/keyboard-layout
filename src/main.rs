use dxf::{Drawing, Point, entities::*};
use serde_json::Value;
use std::{collections::HashMap, fs};

mod launch;
mod virgo;

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
    let json = fs::read_to_string("res/launch_1.json").unwrap();
    let v: Value = serde_json::from_str(&json).unwrap();
    println!("{:#?}", v);

    for &spacing in &[19.0, 20.5] {
        // X switch
        let key = Key {
            margin: Rect::new(0.0, 0.0, spacing, spacing),
            cap: Rect::new(1.25, 1.25, 16.5, 16.5),
            hole: Rect::new(2.5, 2.5, 14.0, 14.0),
        };

        // Mini-choc switch
        // let key = Key {
        //     margin: Rect::new(0.0, 0.0, 19.0, 19.0),
        //     cap: Rect::new(0.75, 1.25, 17.5, 16.5),
        //     hole: Rect::new(2.575, 3.25, 13.85, 12.5),
        // };

        // MX full size switch
        // let key = Key {
        //     margin: Rect::new(0.0, 0.0, 19.0, 19.0),
        //     cap: Rect::new(1.25, 1.25, 16.5, 16.5),
        //     hole: Rect::new(2.5, 2.5, 14.0, 14.0),
        // };

        let mut drawing = Drawing::default();
        let mut switches = String::new();
        let mut mounts = HashMap::new();

        let mut row_i = 0;
        let mut col_i = 0;
        let mut x = 0.0;
        let mut y = 0.0;
        let mut w = 1.0;
        let mut h = 1.0;

        if let Value::Array(ref rows) = v {
            let mut total_rows = 0;
            for row in rows.iter() {
                match row {
                    Value::Array(_) => total_rows += 1,
                    _ => (),
                }
            }

            for row in rows {
                match row {
                    Value::Array(cols) => {
                        let mut total_cols = 0;
                        for col in cols.iter() {
                            match col {
                                Value::String(_) => total_cols += 1,
                                _ => (),
                            }
                        }

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
                                    if let Some(h_v) = o.get("h") {
                                        if let Value::Number(h_n) = h_v {
                                            if let Some(h_f) = h_n.as_f64() {
                                                h = h_f;
                                            }
                                        }
                                    }
                                },
                                Value::String(s) => {
                                    println!("Key {}, {} = {:?}", x, y, s);

                                    x += key.margin.w * (w - 1.0) / 2.0;
                                    y -= key.margin.h * (h - 1.0) / 2.0;

                                    drawing.entities.extend_from_slice(
                                        &key.entities(x, y)
                                    );

                                    switches.push_str(&virgo::switch(
                                        &virgo::reference(row_i, col_i),
                                        x,
                                        -y,
                                        col_i,
                                    ));

                                    {
                                        //HACKS
                                        let mount_center_x = (24.775 + 43.775) / 2.0;
                                        let mount_offset_x = key.margin.w * w / 2.0;
                                        let mount_center_y = (32.643 + 51.643) / 2.0;
                                        let mount_offset_y = key.margin.h * h / 2.0;
                                        for &mount_y in &[
                                            -y + mount_center_y - mount_offset_y,
                                            -y + mount_center_y + mount_offset_y,
                                        ] {
                                            for &mount_x in &[
                                                x + mount_center_x - mount_offset_x,
                                                x + mount_center_x + mount_offset_x,
                                            ] {
                                                let mount_x_int = (mount_x * 1000.0).floor() as i64;
                                                let mount_y_int = (mount_y * 1000.0).floor() as i64;
                                                mounts.insert(
                                                    (mount_x_int, mount_y_int),
                                                    virgo::mount(mount_x, mount_y)
                                                );
                                            }
                                        }
                                    }

                                    x += key.margin.w * (w - 1.0) / 2.0;

                                    x += key.margin.w;
                                    y += key.margin.h * (h - 1.0) / 2.0;
                                    w = 1.0;
                                    h = 1.0;

                                    col_i += 1;
                                }
                                _ => (),
                            }
                        }

                        x = 0.0;
                        y -= key.margin.h;

                        col_i = 0;
                        row_i += 1;
                    },
                    _ => (),
                }
            }
        }

        for (_, mount) in mounts {
            switches.push_str(&mount);
        }

        drawing.save_file(&format!("test-{}mm.dxf", spacing)).unwrap();
        fs::write(&format!("test-{}mm.kicad_pcb", spacing), virgo::pcb(&switches)).unwrap();
    }
}
