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

pub fn pcb(data: &str) -> String {
    format!(
r#"(kicad_pcb (version 20171130) (host pcbnew 5.1.5+dfsg1-2build1)

  (general
    (thickness 1.6)
    (drawings 0)
    (tracks 4)
    (zones 0)
    (modules 3)
    (nets 1)
  )

  (page A4)
  (layers
    (0 F.Cu signal)
    (31 B.Cu signal)
    (32 B.Adhes user)
    (33 F.Adhes user)
    (34 B.Paste user)
    (35 F.Paste user)
    (36 B.SilkS user)
    (37 F.SilkS user)
    (38 B.Mask user)
    (39 F.Mask user)
    (40 Dwgs.User user)
    (41 Cmts.User user)
    (42 Eco1.User user)
    (43 Eco2.User user)
    (44 Edge.Cuts user)
    (45 Margin user)
    (46 B.CrtYd user)
    (47 F.CrtYd user)
    (48 B.Fab user)
    (49 F.Fab user)
  )

  (setup
    (last_trace_width 0.25)
    (trace_clearance 0.2)
    (zone_clearance 0.508)
    (zone_45_only no)
    (trace_min 0.2)
    (via_size 0.8)
    (via_drill 0.4)
    (via_min_size 0.4)
    (via_min_drill 0.3)
    (uvia_size 0.3)
    (uvia_drill 0.1)
    (uvias_allowed no)
    (uvia_min_size 0.2)
    (uvia_min_drill 0.1)
    (edge_width 0.05)
    (segment_width 0.2)
    (pcb_text_width 0.3)
    (pcb_text_size 1.5 1.5)
    (mod_edge_width 0.12)
    (mod_text_size 1 1)
    (mod_text_width 0.15)
    (pad_size 1.524 1.524)
    (pad_drill 0.762)
    (pad_to_mask_clearance 0.051)
    (solder_mask_min_width 0.25)
    (aux_axis_origin 0 0)
    (visible_elements FFFFFF7F)
    (pcbplotparams
      (layerselection 0x010fc_ffffffff)
      (usegerberextensions false)
      (usegerberattributes false)
      (usegerberadvancedattributes false)
      (creategerberjobfile false)
      (excludeedgelayer true)
      (linewidth 0.150000)
      (plotframeref false)
      (viasonmask false)
      (mode 1)
      (useauxorigin false)
      (hpglpennumber 1)
      (hpglpenspeed 20)
      (hpglpendiameter 15.000000)
      (psnegative false)
      (psa4output false)
      (plotreference true)
      (plotvalue true)
      (plotinvisibletext false)
      (padsonsilk false)
      (subtractmaskfromsilk false)
      (outputformat 1)
      (mirror false)
      (drillshape 1)
      (scaleselection 1)
      (outputdirectory ""))
  )

  (net 0 "")

  (net_class Default "This is the default net class."
    (clearance 0.2)
    (trace_width 0.25)
    (via_dia 0.8)
    (via_drill 0.4)
    (uvia_dia 0.3)
    (uvia_drill 0.1)
  )
{}
)
"#,
        data
    )
}

pub fn switch(reference: &str, x: f64, y: f64) -> String {
    format!(
r#"
  (module Diode_SMD:D_0805_2012Metric (layer F.Cu) (tedit 5B36C52B) (tstamp 5E63CD2F)
    (at {diode_x} {diode_y} 270)
    (descr "Diode SMD 0805 (2012 Metric), square (rectangular) end terminal, IPC_7351 nominal, (Body size source: https://docs.google.com/spreadsheets/d/1BsfQQcO9C6DZCsRaXUlFlo91Tg2WpOkGARC1WS5S8t0/edit?usp=sharing), generated with kicad-footprint-generator")
    (tags diode)
    (path /5E61B054)
    (attr smd)
    (fp_text reference D{reference} (at 0 1.65 90) (layer B.SilkS)
      (effects (font (size 1 1) (thickness 0.15)) (justify mirror))
    )
    (fp_text value D (at 0 -1.65 90) (layer B.Fab)
      (effects (font (size 1 1) (thickness 0.15)) (justify mirror))
    )
    (fp_text user %R (at 0 0 90) (layer B.Fab)
      (effects (font (size 0.5 0.5) (thickness 0.08)) (justify mirror))
    )
    (fp_line (start 1.68 -0.95) (end -1.68 -0.95) (layer B.CrtYd) (width 0.05))
    (fp_line (start 1.68 0.95) (end 1.68 -0.95) (layer B.CrtYd) (width 0.05))
    (fp_line (start -1.68 0.95) (end 1.68 0.95) (layer B.CrtYd) (width 0.05))
    (fp_line (start -1.68 -0.95) (end -1.68 0.95) (layer B.CrtYd) (width 0.05))
    (fp_line (start -1.685 -0.96) (end 1 -0.96) (layer B.SilkS) (width 0.12))
    (fp_line (start -1.685 0.96) (end -1.685 -0.96) (layer B.SilkS) (width 0.12))
    (fp_line (start 1 0.96) (end -1.685 0.96) (layer B.SilkS) (width 0.12))
    (fp_line (start 1 -0.6) (end 1 0.6) (layer B.Fab) (width 0.1))
    (fp_line (start -1 -0.6) (end 1 -0.6) (layer B.Fab) (width 0.1))
    (fp_line (start -1 0.3) (end -1 -0.6) (layer B.Fab) (width 0.1))
    (fp_line (start -0.7 0.6) (end -1 0.3) (layer B.Fab) (width 0.1))
    (fp_line (start 1 0.6) (end -0.7 0.6) (layer B.Fab) (width 0.1))
    (pad 2 smd roundrect (at 0.9375 0 270) (size 0.975 1.4) (layers B.Cu B.Paste B.Mask) (roundrect_rratio 0.25))
    (pad 1 smd roundrect (at -0.9375 0 270) (size 0.975 1.4) (layers B.Cu B.Paste B.Mask) (roundrect_rratio 0.25))
    (model ${{KISYS3DMOD}}/Diode_SMD.3dshapes/D_0805_2012Metric.wrl
      (at (xyz 0 0 0))
      (scale (xyz 1 1 1))
      (rotate (xyz 0 0 0))
    )
  )

  (module keyswitches:Kailh_socket_MX (layer F.Cu) (tedit 5DD4FB17) (tstamp 5E63CCFC)
    (at {switch_x} {switch_y} 180)
    (descr "MX-style keyswitch with Kailh socket mount")
    (tags MX,cherry,gateron,kailh,pg1511,socket)
    (path /5E61AAE5)
    (attr smd)
    (fp_text reference SW{reference} (at 0 -8.255) (layer B.SilkS)
      (effects (font (size 1 1) (thickness 0.15)) (justify mirror))
    )
    (fp_text value SW_Push (at 0 8.255) (layer F.Fab)
      (effects (font (size 1 1) (thickness 0.15)))
    )
    (fp_line (start -7 -6) (end -7 -7) (layer F.SilkS) (width 0.15))
    (fp_line (start -7 7) (end -6 7) (layer F.SilkS) (width 0.15))
    (fp_line (start -6 -7) (end -7 -7) (layer F.SilkS) (width 0.15))
    (fp_line (start -7 7) (end -7 6) (layer F.SilkS) (width 0.15))
    (fp_line (start 7 6) (end 7 7) (layer F.SilkS) (width 0.15))
    (fp_line (start 7 -7) (end 6 -7) (layer F.SilkS) (width 0.15))
    (fp_line (start 6 7) (end 7 7) (layer F.SilkS) (width 0.15))
    (fp_line (start 7 -7) (end 7 -6) (layer F.SilkS) (width 0.15))
    (fp_line (start -6.9 6.9) (end 6.9 6.9) (layer Eco2.User) (width 0.15))
    (fp_line (start 6.9 -6.9) (end -6.9 -6.9) (layer Eco2.User) (width 0.15))
    (fp_line (start 6.9 -6.9) (end 6.9 6.9) (layer Eco2.User) (width 0.15))
    (fp_line (start -6.9 6.9) (end -6.9 -6.9) (layer Eco2.User) (width 0.15))
    (fp_line (start -7.5 -7.5) (end 7.5 -7.5) (layer F.Fab) (width 0.15))
    (fp_line (start 7.5 -7.5) (end 7.5 7.5) (layer F.Fab) (width 0.15))
    (fp_line (start 7.5 7.5) (end -7.5 7.5) (layer F.Fab) (width 0.15))
    (fp_line (start -7.5 7.5) (end -7.5 -7.5) (layer F.Fab) (width 0.15))
    (fp_arc (start -3.81 -4.445) (end -3.81 -6.985) (angle -90) (layer B.SilkS) (width 0.15))
    (fp_line (start -6.35 -1.016) (end -6.35 -0.635) (layer B.SilkS) (width 0.15))
    (fp_arc (start 0 0) (end 0 -2.54) (angle -75.96375653) (layer B.SilkS) (width 0.15))
    (fp_line (start 5.08 -3.556) (end 5.08 -2.54) (layer B.SilkS) (width 0.15))
    (fp_line (start 5.08 -2.54) (end 0 -2.54) (layer B.SilkS) (width 0.15))
    (fp_line (start -2.464162 -0.635) (end -4.191 -0.635) (layer B.SilkS) (width 0.15))
    (fp_line (start -5.969 -0.635) (end -6.35 -0.635) (layer B.SilkS) (width 0.15))
    (fp_line (start -6.35 -4.445) (end -6.35 -4.064) (layer B.SilkS) (width 0.15))
    (fp_line (start -3.81 -6.985) (end 5.08 -6.985) (layer B.SilkS) (width 0.15))
    (fp_line (start 5.08 -6.985) (end 5.08 -6.604) (layer B.SilkS) (width 0.15))
    (fp_arc (start -3.81 -4.445) (end -3.81 -6.985) (angle -90) (layer B.Fab) (width 0.12))
    (fp_arc (start 0 0) (end 0 -2.54) (angle -75.96375653) (layer B.Fab) (width 0.12))
    (fp_line (start -6.35 -0.635) (end -2.54 -0.635) (layer B.Fab) (width 0.12))
    (fp_line (start -6.35 -0.635) (end -6.35 -4.445) (layer B.Fab) (width 0.12))
    (fp_line (start -3.81 -6.985) (end 5.08 -6.985) (layer B.Fab) (width 0.12))
    (fp_line (start 5.08 -6.985) (end 5.08 -2.54) (layer B.Fab) (width 0.12))
    (fp_line (start 5.08 -2.54) (end 0 -2.54) (layer B.Fab) (width 0.12))
    (fp_line (start 5.08 -6.35) (end 7.62 -6.35) (layer B.Fab) (width 0.12))
    (fp_line (start 7.62 -6.35) (end 7.62 -3.81) (layer B.Fab) (width 0.12))
    (fp_line (start 7.62 -3.81) (end 5.08 -3.81) (layer B.Fab) (width 0.12))
    (fp_line (start -6.35 -1.27) (end -8.89 -1.27) (layer B.Fab) (width 0.12))
    (fp_line (start -8.89 -1.27) (end -8.89 -3.81) (layer B.Fab) (width 0.12))
    (fp_line (start -8.89 -3.81) (end -6.35 -3.81) (layer B.Fab) (width 0.12))
    (fp_text user %R (at -0.635 -4.445) (layer B.Fab)
      (effects (font (size 1 1) (thickness 0.15)) (justify mirror))
    )
    (fp_text user %V (at -0.635 0.635) (layer B.Fab)
      (effects (font (size 1 1) (thickness 0.15)) (justify mirror))
    )
    (pad 1 smd rect (at 6.29 -5.08 180) (size 2.55 2.5) (layers B.Cu B.Paste B.Mask))
    (pad "" np_thru_hole circle (at 2.54 -5.08 180) (size 3 3) (drill 3) (layers *.Cu *.Mask))
    (pad "" np_thru_hole circle (at -3.81 -2.54 180) (size 3 3) (drill 3) (layers *.Cu *.Mask))
    (pad "" np_thru_hole circle (at 0 0 180) (size 3.9878 3.9878) (drill 3.9878) (layers *.Cu *.Mask))
    (pad "" np_thru_hole circle (at 5.08 0 180) (size 1.7018 1.7018) (drill 1.7018) (layers *.Cu *.Mask))
    (pad "" np_thru_hole circle (at -5.08 0 180) (size 1.7018 1.7018) (drill 1.7018) (layers *.Cu *.Mask))
    (pad 2 smd rect (at -7.56 -2.54 180) (size 2.55 2.5) (layers B.Cu B.Paste B.Mask))  )

  (module LED_SMD:LED_WS2812B_PLCC4_5.0x5.0mm_P3.2mm (layer F.Cu) (tedit 5AA4B285) (tstamp 5E63CCE4)
    (at {switch_x} {led_y} 180)
    (descr https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf)
    (tags "LED RGB NeoPixel")
    (path /5EA26F29)
    (attr smd)
    (fp_text reference L{reference} (at 0 3.89) (layer F.SilkS)
      (effects (font (size 1 1) (thickness 0.15)))
    )
    (fp_text value WS2812B (at 0 4) (layer F.Fab)
      (effects (font (size 1 1) (thickness 0.15)))
    )
    (fp_text user 1 (at -4.15 -1.6) (layer F.SilkS)
      (effects (font (size 1 1) (thickness 0.15)))
    )
    (fp_text user %R (at 0 0) (layer F.Fab)
      (effects (font (size 0.8 0.8) (thickness 0.15)))
    )
    (fp_line (start 3.45 -2.75) (end -3.45 -2.75) (layer F.CrtYd) (width 0.05))
    (fp_line (start 3.45 2.75) (end 3.45 -2.75) (layer F.CrtYd) (width 0.05))
    (fp_line (start -3.45 2.75) (end 3.45 2.75) (layer F.CrtYd) (width 0.05))
    (fp_line (start -3.45 -2.75) (end -3.45 2.75) (layer F.CrtYd) (width 0.05))
    (fp_line (start 2.5 1.5) (end 1.5 2.5) (layer F.Fab) (width 0.1))
    (fp_line (start -2.5 -2.5) (end -2.5 2.5) (layer F.Fab) (width 0.1))
    (fp_line (start -2.5 2.5) (end 2.5 2.5) (layer F.Fab) (width 0.1))
    (fp_line (start 2.5 2.5) (end 2.5 -2.5) (layer F.Fab) (width 0.1))
    (fp_line (start 2.5 -2.5) (end -2.5 -2.5) (layer F.Fab) (width 0.1))
    (fp_line (start -3.65 -2.75) (end 3.65 -2.75) (layer F.SilkS) (width 0.12))
    (fp_line (start -3.65 2.75) (end 3.65 2.75) (layer F.SilkS) (width 0.12))
    (fp_line (start 3.65 2.75) (end 3.65 1.6) (layer F.SilkS) (width 0.12))
    (fp_circle (center 0 0) (end 0 -2) (layer F.Fab) (width 0.1))
    (pad 3 smd rect (at 2.45 1.6 180) (size 1.5 1) (layers F.Cu F.Paste F.Mask))
    (pad 4 smd rect (at 2.45 -1.6 180) (size 1.5 1) (layers F.Cu F.Paste F.Mask))
    (pad 2 smd rect (at -2.45 1.6 180) (size 1.5 1) (layers F.Cu F.Paste F.Mask))
    (pad 1 smd rect (at -2.45 -1.6 180) (size 1.5 1) (layers F.Cu F.Paste F.Mask))
    (model ${{KISYS3DMOD}}/LED_SMD.3dshapes/LED_WS2812B_PLCC4_5.0x5.0mm_P3.2mm.wrl
      (at (xyz 0 0 0))
      (scale (xyz 1 1 1))
      (rotate (xyz 0 0 0))
    )
  )

  (segment (start {diode_x} {switch_p2_y}) (end {diode_x} {diode_p1_y}) (width 0.25) (layer B.Cu) (net 0) (status 30))
  (segment (start {diode_x} {diode_p2_y}) (end {diode_x} {via_y}) (width 0.25) (layer B.Cu) (net 0) (status 30))
  (via (at {diode_x} {via_y}) (size 0.8) (drill 0.4) (layers F.Cu B.Cu) (net 0))
"#,
        reference=reference,
        diode_x=(x + 7.56),
        diode_y=(y - 4.1875),
        diode_p1_y=(y - 3.25),
        diode_p2_y=(y - 5.125),
        led_y=(y - 5.0),
        switch_x=x,
        switch_y=y,
        switch_p2_y=(y + 2.54),
        via_y=(y - 8.0),
    )
}
