use std::fmt::Write;

pub fn reference(row_i: usize, mut col_i: usize) -> String {
    // WARNING: Board specific modifications for holes:
    match row_i {
        3 => if col_i >= 0xD {
            col_i += 1;
        },
        5 => if col_i >= 0x5 {
            col_i += 1;
        },
        _ => (),
    }

    format!(
        "{}{}",
        (b'A' + col_i as u8) as char,
        row_i
    )
}

pub fn mount(x: f64, y: f64) -> String {
    format!(
r#"
  (footprint "local:MountingHole_1.8mm_Pad" (layer "F.Cu")
    (tstamp 5c74d3f2-347f-4f03-ab4d-10600d820876)
    (at {x} {y})
    (descr "Mounting Hole 1.8mm")
    (tags "mounting hole 1.8mm")
    (attr exclude_from_pos_files exclude_from_bom)
    (fp_circle (center 0 0) (end 1.5 0)
      (stroke (width 0.15) (type solid)) (fill none) (layer "Cmts.User") (tstamp 94237e3a-78db-401d-a248-e0ca891ee1f8))
    (fp_circle (center 0 0) (end 1.75 0)
      (stroke (width 0.05) (type solid)) (fill none) (layer "F.CrtYd") (tstamp d7083825-bbae-49db-9f12-1f13302711cb))
    (pad "1" thru_hole circle (at 0 0) (size 3 3) (drill 1.8) (layers "*.Cu" "*.Mask") (tstamp f7de860d-2664-4827-9a17-e0308aa8b525))
  )
"#,
        x=x,
        y=y,
    )
}

pub fn switch(reference: &str, x: f64, y: f64, col_i: usize) -> String {
    let (switch_x, switch_y) = (x + 31.0, y + 45.0);
    let (led_x, led_y) = (x + 34.5, y + 37.25);

    let mut footprints = String::new();

    let diode = if col_i % 2 == 0 {
        let (diode_x, diode_y) = (x + 38.75, y + 32.55);
        format!(
r#"
  (footprint "Package_TO_SOT_SMD:SOT-23" (layer "F.Cu")
    (tstamp 00000000-0000-0000-0000-00005f283a1e)
    (at {diode_x} {diode_y} 90)
    (descr "SOT-23, Standard")
    (tags "SOT-23")
    (property "Manufacturer" "Nexperia")
    (property "Sheetfile" "virgo-keyboard-test.kicad_sch")
    (property "Sheetname" "")
    (property "ki_description" "Dual diode, common cathode on pin 3")
    (property "ki_keywords" "diode")
    (path "/00000000-0000-0000-0000-00006068616e")
    (attr smd)
    (fp_text reference "D{reference}" (at 0 -2.5 90) (layer "F.SilkS")
        (effects (font (size 1 1) (thickness 0.15)))
      (tstamp 121bc3e0-0266-46f3-a866-d04aaa626446)
    )
    (fp_text value "BAV70,215" (at 0 2.5 90) (layer "F.Fab")
        (effects (font (size 1 1) (thickness 0.15)))
      (tstamp 9517458c-3f8e-46df-8930-2d9b6adfbaa3)
    )
    (fp_text user "${{REFERENCE}}" (at 0 0) (layer "F.Fab")
        (effects (font (size 0.5 0.5) (thickness 0.075)))
      (tstamp 788b2edb-8d96-4cb6-bbe1-a05ba3a86c0e)
    )
    (fp_line (start 0.76 -1.58) (end -1.4 -1.58)
      (stroke (width 0.12) (type solid)) (layer "F.SilkS") (tstamp 24a823c1-6250-41a8-a8e4-9e9f9a092069))
    (fp_line (start 0.76 -1.58) (end 0.76 -0.65)
      (stroke (width 0.12) (type solid)) (layer "F.SilkS") (tstamp 85e07ef0-10e6-414b-bca4-870703935b9e))
    (fp_line (start 0.76 1.58) (end -0.7 1.58)
      (stroke (width 0.12) (type solid)) (layer "F.SilkS") (tstamp 0f65d1e6-78d8-4d2f-a55b-943e3cd3a0e8))
    (fp_line (start 0.76 1.58) (end 0.76 0.65)
      (stroke (width 0.12) (type solid)) (layer "F.SilkS") (tstamp 670545ae-6402-4cac-8ed2-d108e7a27600))
    (fp_line (start -1.7 -1.75) (end 1.7 -1.75)
      (stroke (width 0.05) (type solid)) (layer "F.CrtYd") (tstamp 8a8e7f2b-4157-4fb3-9e95-b0ff637179d0))
    (fp_line (start -1.7 1.75) (end -1.7 -1.75)
      (stroke (width 0.05) (type solid)) (layer "F.CrtYd") (tstamp 9a7eb483-a1f5-4d4a-92a7-91c0a794f17a))
    (fp_line (start 1.7 -1.75) (end 1.7 1.75)
      (stroke (width 0.05) (type solid)) (layer "F.CrtYd") (tstamp 37b3ce42-9ca0-4cf3-97ea-e545e8d4914b))
    (fp_line (start 1.7 1.75) (end -1.7 1.75)
      (stroke (width 0.05) (type solid)) (layer "F.CrtYd") (tstamp e2329483-4124-49d9-8585-285e01762673))
    (fp_line (start -0.7 -0.95) (end -0.7 1.5)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp 57b0b80f-3c9e-4954-bdd9-3c5a4c34bd3d))
    (fp_line (start -0.7 -0.95) (end -0.15 -1.52)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp 5be839c9-6a53-4876-b681-10e8f1bd1adf))
    (fp_line (start -0.7 1.52) (end 0.7 1.52)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp 5987b169-f087-465a-967b-e88aab85db87))
    (fp_line (start -0.15 -1.52) (end 0.7 -1.52)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp cb52785e-a405-45af-a4d7-27c9ef998ddd))
    (fp_line (start 0.7 -1.52) (end 0.7 1.52)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp 091066c9-e58a-425c-9cb9-44cba3c85baa))
    (pad "1" smd rect (at -1 -0.95 270) (size 0.9 0.8) (layers "F.Cu" "F.Paste" "F.Mask")
      (net 0 "") (pinfunction "A") (pintype "passive") (tstamp dbf014cb-d753-4ebc-8db0-d48f77fe78c9))
    (pad "2" smd rect (at -1 0.95 270) (size 0.9 0.8) (layers "F.Cu" "F.Paste" "F.Mask")
      (net 0 "") (pinfunction "A") (pintype "passive") (tstamp 9b70a78d-d955-461c-8f5f-cee026bd896b))
    (pad "3" smd rect (at 1 0 270) (size 0.9 0.8) (layers "F.Cu" "F.Paste" "F.Mask")
      (net 0 "") (pinfunction "K") (pintype "passive") (tstamp 9b260d5d-d234-40fb-96ee-f7b9ae0db9b3))
    (model "${{KISYS3DMOD}}/Package_TO_SOT_SMD.3dshapes/SOT-23.step"
      (offset (xyz 0 0 0))
      (scale (xyz 1 1 1))
      (rotate (xyz 0 0 0))
    )
  )
"#,
            reference=reference,
            diode_x=diode_x,
            diode_y=diode_y,
        )
    } else {
        String::new()
    };

    format!(
r#"
  (footprint "kailh-pg1425-x-switch:Kailh-PG1425-X-Switch" (layer "F.Cu")
    (tstamp 00000000-0000-0000-0000-00005e63e29e)
    (at {switch_x} {switch_y} 180)
    (property "Manufacturer" "Kailh")
    (property "Sheetfile" "virgo-keyboard-test.kicad_sch")
    (property "Sheetname" "")
    (property "ki_description" "Push button switch, generic, two pins")
    (property "ki_keywords" "switch normally-open pushbutton push-button")
    (path "/00000000-0000-0000-0000-00005e61aae5")
    (attr through_hole)
    (fp_text reference "SW{reference}" (at -3.4 13.2) (layer "F.SilkS") hide
        (effects (font (size 1 1) (thickness 0.15)))
      (tstamp 5ca9a131-d0d7-4d38-a01d-4e8b7f94fe5c)
    )
    (fp_text value "CPG151101S11" (at -3.4 -7.3) (layer "F.Fab") hide
        (effects (font (size 1 1) (thickness 0.15)))
      (tstamp 6b9f2da7-40a0-4a27-b48d-f9dca40bf894)
    )
    (fp_line (start -10.8 -4.1) (end 4 -4.1)
      (stroke (width 0.15) (type solid)) (layer "F.SilkS") (tstamp 3545059e-db9d-47c7-a107-d1351fd33127))
    (fp_line (start -10.8 9.9) (end -10.8 -4.1)
      (stroke (width 0.15) (type solid)) (layer "F.SilkS") (tstamp 3e939ae9-f7bb-4146-857b-88237cb9f387))
    (fp_line (start 4 -4.1) (end 4 9.9)
      (stroke (width 0.15) (type solid)) (layer "F.SilkS") (tstamp 1960904b-0b9b-46f0-8eaa-763b375df74a))
    (fp_line (start 4 9.9) (end -10.8 9.9)
      (stroke (width 0.15) (type solid)) (layer "F.SilkS") (tstamp a144163c-c5e7-4547-8a54-db89afef22ce))
    (fp_rect (start -11.5 -4.75) (end 4.75 10.75)
      (stroke (width 0.12) (type default)) (fill none) (layer "Dwgs.User") (tstamp 8e10c6e1-12b7-49bf-9724-d5c8d0158e01))
    (fp_line (start -5.975 1.725) (end -5.975 5.875)
      (stroke (width 0.12) (type default)) (layer "Edge.Cuts") (tstamp 513c1d33-64ba-4dc6-8777-8800b33bbbc2))
    (fp_line (start -4.275 5.875) (end -2.525 5.875)
      (stroke (width 0.12) (type default)) (layer "Edge.Cuts") (tstamp 8991bea3-9670-4251-b66f-2e5c03aeb4c4))
    (fp_line (start -2.525 1.725) (end -4.275 1.725)
      (stroke (width 0.12) (type default)) (layer "Edge.Cuts") (tstamp d170a11b-2156-465c-b693-dfa825068e56))
    (fp_line (start -0.825 5.875) (end -0.825 1.725)
      (stroke (width 0.12) (type default)) (layer "Edge.Cuts") (tstamp 580e5b4d-ec9a-44ae-a795-f05322ec149d))
    (fp_arc (start -5.975 1.725) (mid -5.125 0.875) (end -4.275 1.725)
      (stroke (width 0.12) (type default)) (layer "Edge.Cuts") (tstamp 65ecdf35-b620-4be3-badc-5f462cdbfc0d))
    (fp_arc (start -4.275 5.875) (mid -5.125 6.725) (end -5.975 5.875)
      (stroke (width 0.12) (type default)) (layer "Edge.Cuts") (tstamp 97dcca40-fae2-434a-829b-c5e9e578a3fd))
    (fp_arc (start -2.525 1.725) (mid -1.675 0.875) (end -0.825 1.725)
      (stroke (width 0.12) (type default)) (layer "Edge.Cuts") (tstamp 6d2802b6-e1dc-4df0-93c0-1a438ad6933a))
    (fp_arc (start -0.825 5.875) (mid -1.675 6.725) (end -2.525 5.875)
      (stroke (width 0.12) (type default)) (layer "Edge.Cuts") (tstamp c5269667-3281-4bcc-b221-31afc386eb12))
    (fp_rect (start -11.25 -4.5) (end 4.5 10.5)
      (stroke (width 0.05) (type default)) (fill none) (layer "F.CrtYd") (tstamp 777e0c4a-ab34-4c17-8793-8b4ff5e16aef))
    (fp_rect (start -5.8 6) (end -1 9.3)
      (stroke (width 0.05) (type default)) (fill none) (layer "F.CrtYd") (tstamp 7ceea0ca-0917-4f3e-bda8-2e147cbbed1e))
    (fp_line (start -5.950008 9.449836) (end -5.950008 5.849836)
      (stroke (width 0.15) (type solid)) (layer "B.Fab") (tstamp adb12625-c273-43f5-b605-3f54a517452c))
    (fp_line (start -0.850008 5.849836) (end -0.850008 9.449836)
      (stroke (width 0.15) (type solid)) (layer "B.Fab") (tstamp b2c10f4c-0d4e-4a45-9c8c-6e5db6b80658))
    (fp_line (start -0.850008 9.449836) (end -5.950008 9.449836)
      (stroke (width 0.15) (type solid)) (layer "B.Fab") (tstamp 2d14313d-f910-41ca-a9fb-110f865dd03c))
    (fp_line (start -5.95 5.85) (end -5.95 9.45)
      (stroke (width 0.15) (type solid)) (layer "F.Fab") (tstamp 9b56d2a7-1839-4644-bafc-3babdbadc6b5))
    (fp_line (start -5.95 9.45) (end -0.85 9.45)
      (stroke (width 0.15) (type solid)) (layer "F.Fab") (tstamp 6c91f4ec-b401-4e76-9686-32957c09ac86))
    (fp_line (start -0.85 5.85) (end -5.95 5.85)
      (stroke (width 0.15) (type solid)) (layer "F.Fab") (tstamp 2dcb5d58-d81f-440f-a1f4-c341d9961266))
    (fp_line (start -0.85 9.45) (end -0.85 5.85)
      (stroke (width 0.15) (type solid)) (layer "F.Fab") (tstamp 093b732d-2562-44d2-8732-bf818d6cf824))
    (pad "" np_thru_hole circle (at -8.900008 -2.600164) (size 1.3 1.3) (drill 1.3) (layers "*.Cu" "*.Mask") (tstamp 0d06ccea-8bd8-49fb-a6f2-2d1d61edc572))
    (pad "" np_thru_hole circle (at 2.099992 8.399836) (size 1.3 1.3) (drill 1.3) (layers "*.Cu" "*.Mask") (tstamp 10c9ecf6-672a-44c9-ab90-c10ab3433a09))
    (pad "1" thru_hole oval (at -6.800008 -0.000164 90) (size 1.6 1.8) (drill 1.2) (layers "*.Cu" "*.Mask")
      (net 0 "") (pinfunction "1") (pintype "passive") (tstamp 83392078-b0a5-4ec0-b9cb-49dd3646d5b4))
    (pad "2" thru_hole oval (at -6.8 4.9) (size 1.15 1.8) (drill 0.9) (layers "*.Cu" "*.Mask")
      (net 0 "") (pinfunction "2") (pintype "passive") (tstamp 43c3c721-3a16-4107-b847-7d894370f596))
    (model "${{KIPRJMOD}}/models/CPG142501DXX v3.step"
      (offset (xyz -4.05 12.05 1.5))
      (scale (xyz 1 1 1))
      (rotate (xyz 0 0 -90))
    )
  )

  (footprint "local:LED_SK6805-EC15" (layer "F.Cu")
    (tstamp 00000000-0000-0000-0000-00005f3055bc)
    (at {led_x} {led_y})
    (descr "1.5mm x 1.5mm LED")
    (tags "LED")
    (property "Sheetfile" "virgo-keyboard-test.kicad_sch")
    (property "Sheetname" "")
    (property "ki_description" "RGB LED with integrated controller")
    (property "ki_keywords" "RGB LED addressable")
    (path "/00000000-0000-0000-0000-00005ea26f29")
    (attr smd)
    (fp_text reference "L{reference}" (at 2.8 0) (layer "F.SilkS")
        (effects (font (size 1 1) (thickness 0.15)))
      (tstamp 3e3cf536-5655-4d8d-80c9-9cdf4fe02087)
    )
    (fp_text value "SK6805-EC15" (at 0 2.25) (layer "F.Fab") hide
        (effects (font (size 1 1) (thickness 0.15)))
      (tstamp 292b2987-9539-4c73-8f24-2d336de39034)
    )
    (fp_text user "${{REFERENCE}}" (at 0 0) (layer "F.Fab")
        (effects (font (size 0.5 0.5) (thickness 0.075)))
      (tstamp aa6bd532-976e-41be-83ef-e64c27098ce3)
    )
    (fp_line (start -1 -1) (end 1 -1)
      (stroke (width 0.12) (type solid)) (layer "F.SilkS") (tstamp 3394bab0-f42e-4d8b-ae90-b9194c7938da))
    (fp_line (start -1 -0.55) (end -1 -1)
      (stroke (width 0.12) (type solid)) (layer "F.SilkS") (tstamp 1b4ff3dd-616d-4421-8861-4336d6586b66))
    (fp_line (start -1 1) (end 1 1)
      (stroke (width 0.12) (type solid)) (layer "F.SilkS") (tstamp 3b9dde74-e93d-462a-81d6-fc11b33bc1da))
    (fp_poly
      (pts
        (xy 0.1 0.7)
        (xy -0.1 0.5)
        (xy 0.1 0.3)
      )

      (stroke (width 0) (type solid)) (fill solid) (layer "F.SilkS") (tstamp d8c433e6-9281-4bb0-a035-9a53c540cac8))
    (fp_line (start -1.2 -1.2) (end -1.2 1.2)
      (stroke (width 0.05) (type solid)) (layer "F.CrtYd") (tstamp e9bfb5fd-33f5-408f-90af-eeb1c986e48d))
    (fp_line (start -1.2 1.2) (end 1.2 1.2)
      (stroke (width 0.05) (type solid)) (layer "F.CrtYd") (tstamp a51e661d-bf8c-4418-80b1-0fa39d062705))
    (fp_line (start 1.2 -1.2) (end -1.2 -1.2)
      (stroke (width 0.05) (type solid)) (layer "F.CrtYd") (tstamp 5a995fba-6f00-4465-a321-c4615b2d9d44))
    (fp_line (start 1.2 1.2) (end 1.2 -1.2)
      (stroke (width 0.05) (type solid)) (layer "F.CrtYd") (tstamp 329ab187-a133-4820-90c5-614830a7fd80))
    (fp_line (start -1 -1) (end -1 1)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp b262dbb2-e0c8-41cc-b9e8-e4a74b2b054d))
    (fp_line (start -1 1) (end 1 1)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp 71beecf3-42ac-4ebe-bf28-2acb8779fd7c))
    (fp_line (start 0 -1) (end -1 0)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp 99182829-60ff-4608-aa61-b8ad9db6d99f))
    (fp_line (start 1 -1) (end -1 -1)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp 38e18b24-b894-465a-9f65-79841bb1e2e6))
    (fp_line (start 1 1) (end 1 -1)
      (stroke (width 0.1) (type solid)) (layer "F.Fab") (tstamp dba283ff-f889-4572-a46b-4ad2672cd80b))
    (fp_circle (center 0 0) (end 0.8 0)
      (stroke (width 0.1) (type solid)) (fill none) (layer "F.Fab") (tstamp f897fb18-96b0-4f0a-bd1f-82924b3b7726))
    (pad "1" smd rect (at -0.475 -0.475) (size 0.55 0.55) (layers "F.Cu" "F.Paste" "F.Mask")
      (net 0 "") (pinfunction "DIN") (pintype "input") (tstamp 7550e63b-e00d-445c-ab0e-52f9397b210f))
    (pad "2" smd rect (at -0.475 0.475) (size 0.55 0.55) (layers "F.Cu" "F.Paste" "F.Mask")
      (net 0 "") (pinfunction "VDD") (pintype "power_in") (tstamp 3cd3a9ca-b8e4-404b-9a0f-abfb5416721e))
    (pad "3" smd rect (at 0.475 0.475) (size 0.55 0.55) (layers "F.Cu" "F.Paste" "F.Mask")
      (net 0 "") (pinfunction "DOUT") (pintype "output") (tstamp a35870d2-c6fc-4c8f-aea5-5aef48b19fe4))
    (pad "4" smd rect (at 0.475 -0.475) (size 0.55 0.55) (layers "F.Cu" "F.Paste" "F.Mask")
      (net 0 "") (pinfunction "VSS") (pintype "power_in") (tstamp 3b47b305-d974-4d5d-b75b-8b0c6eb19573))
    (model "${{KIPRJMOD}}/models/LED_1515.step"
      (offset (xyz 0 0 0.1))
      (scale (xyz 1 1 1))
      (rotate (xyz 0 0 0))
    )
  )
{diode}
"#,
    reference=reference,
    switch_x=switch_x, switch_y=switch_y,
    led_x=led_x, led_y=led_y,
    diode=diode,
    )
}

pub fn pcb(data: &str) -> String {
    format!(
r#"(kicad_pcb (version 20221018) (generator pcbnew)

  (general
    (thickness 0.6)
  )

  (paper "A3")
  (layers
    (0 "F.Cu" signal)
    (1 "In1.Cu" power)
    (2 "In2.Cu" power)
    (31 "B.Cu" signal)
    (32 "B.Adhes" user "B.Adhesive")
    (33 "F.Adhes" user "F.Adhesive")
    (34 "B.Paste" user)
    (35 "F.Paste" user)
    (36 "B.SilkS" user "B.Silkscreen")
    (37 "F.SilkS" user "F.Silkscreen")
    (38 "B.Mask" user)
    (39 "F.Mask" user)
    (40 "Dwgs.User" user "User.Drawings")
    (41 "Cmts.User" user "User.Comments")
    (42 "Eco1.User" user "User.Eco1")
    (43 "Eco2.User" user "User.Eco2")
    (44 "Edge.Cuts" user)
    (45 "Margin" user)
    (46 "B.CrtYd" user "B.Courtyard")
    (47 "F.CrtYd" user "F.Courtyard")
    (48 "B.Fab" user)
    (49 "F.Fab" user)
  )

  (setup
    (stackup
      (layer "F.SilkS" (type "Top Silk Screen"))
      (layer "F.Paste" (type "Top Solder Paste"))
      (layer "F.Mask" (type "Top Solder Mask") (thickness 0.01))
      (layer "F.Cu" (type "copper") (thickness 0.035))
      (layer "dielectric 1" (type "prepreg") (thickness 0.1) (material "FR4") (epsilon_r 4.5) (loss_tangent 0.02))
      (layer "In1.Cu" (type "copper") (thickness 0.035))
      (layer "dielectric 2" (type "core") (thickness 0.24) (material "FR4") (epsilon_r 4.5) (loss_tangent 0.02))
      (layer "In2.Cu" (type "copper") (thickness 0.035))
      (layer "dielectric 3" (type "prepreg") (thickness 0.1) (material "FR4") (epsilon_r 4.5) (loss_tangent 0.02))
      (layer "B.Cu" (type "copper") (thickness 0.035))
      (layer "B.Mask" (type "Bottom Solder Mask") (thickness 0.01))
      (layer "B.Paste" (type "Bottom Solder Paste"))
      (layer "B.SilkS" (type "Bottom Silk Screen"))
      (copper_finish "None")
      (dielectric_constraints no)
    )
    (pad_to_mask_clearance 0.05)
    (solder_mask_min_width 0.075)
    (grid_origin 37.8 40.1)
    (pcbplotparams
      (layerselection 0x00012fc_ffffffff)
      (plot_on_all_layers_selection 0x0000000_00000000)
      (disableapertmacros false)
      (usegerberextensions false)
      (usegerberattributes false)
      (usegerberadvancedattributes false)
      (creategerberjobfile false)
      (dashed_line_dash_ratio 12.000000)
      (dashed_line_gap_ratio 3.000000)
      (svgprecision 6)
      (plotframeref false)
      (viasonmask false)
      (mode 1)
      (useauxorigin false)
      (hpglpennumber 1)
      (hpglpenspeed 20)
      (hpglpendiameter 15.000000)
      (dxfpolygonmode true)
      (dxfimperialunits true)
      (dxfusepcbnewfont true)
      (psnegative false)
      (psa4output false)
      (plotreference true)
      (plotvalue true)
      (plotinvisibletext false)
      (sketchpadsonfab false)
      (subtractmaskfromsilk false)
      (outputformat 1)
      (mirror false)
      (drillshape 0)
      (scaleselection 1)
      (outputdirectory "gerber")
    )
  )

  (net 0 "")

  {}
)
"#,
        data
    )
}
