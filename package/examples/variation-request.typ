#import "@local/typarium:0.1.0": font-showcase, set-variations

#set page(width: 190mm, height: auto, margin: 10mm)
#set text(size: 10pt)

#let variation-theme = (
  ui-font: "Hiragino Sans",
  mono-font: "Courier",
  color-card: rgb("f8fafc"),
  color-stroke: rgb("d6deeb"),
  color-title: rgb("1f2937"),
  color-meta: rgb("64748b"),
  color-accent: rgb("0f6dff"),
  card-inset: 1.2em,
  card-radius: 0.7em,
  size-title: 1.05em,
  size-meta: 0.78em,
  size-sample: 1.55em,
  sample-text: "Variation request payload preview.",
)

#let variation-render = it => {
  let ft = it.font-text
  let overrides = it.at("item-overrides", default: (:))
  let opt = (key, default) => if type(overrides) == dictionary and key in overrides {
    overrides.at(key)
  } else {
    it.theme.at(key, default: it.at(key, default: default))
  }

  let req = it.at("variation-request", default: (:))
  let values = req.at("values", default: (:))
  let support = it.at("variation-support", default: (:))
  let value-lines = ()
  for key in values.keys() {
    value-lines.push([
      #text(font: opt("mono-font", "Courier"), fill: opt("color-accent", blue))[#key]
      #h(0.5em)
      #values.at(key)
    ])
  }

  block(
    breakable: false,
    width: 100%,
    fill: opt("color-card", white),
    stroke: 0.5pt + opt("color-stroke", luma(220)),
    inset: opt("card-inset", 1em),
    radius: opt("card-radius", 0.6em),
    [
      #text(font: opt("ui-font", "Hiragino Sans"), size: opt("size-title", 1em), weight: "bold", fill: opt("color-title", black))[#it.name]
      #v(0.35em)
      #text(font: opt("ui-font", "Hiragino Sans"), size: opt("size-meta", 0.8em), fill: opt("color-meta", gray))[rendered family=#it.render-name / support=#support.at("native-font-variations", default: false)]
      #v(0.85em)
      #ft(size: opt("size-sample", 1.4em), fill: opt("color-title", black))[#opt("sample-text", "Variation request payload preview.")]
      #v(0.9em)
      #text(font: opt("ui-font", "Hiragino Sans"), size: opt("size-meta", 0.8em), weight: "bold", fill: opt("color-meta", gray))[variation-request]
      #v(0.35em)
      #if value-lines.len() == 0 [
        #text(font: opt("ui-font", "Hiragino Sans"), size: opt("size-meta", 0.8em), fill: opt("color-meta", gray))[No axis values requested.]
      ] else [
        #stack(spacing: 0.25em, ..value-lines)
      ]
      #v(0.8em)
      #text(font: opt("ui-font", "Hiragino Sans"), size: 0.72em, fill: opt("color-meta", gray))[
        Typst 0.14.x does not yet apply these axis requests to shaping; this example shows the normalized host payload that a custom renderer can inspect today.
      ]
    ],
  )
}

#font-showcase(
  columns: 2,
  theme: variation-theme,
  render: variation-render,
  fonts: (
    set-variations((
      name: "Skia",
      display-name: "Skia requested wght/wdth",
      sample-text: "Axis request metadata only.",
    ), (wght: 0.72, wdth: 1.18)),
    set-variations((
      path: read("Jaldi-Regular.ttf", encoding: none),
      display-name: "Jaldi with manual axes",
      sample-text: "Static fonts still carry the request contract.",
    ), (wght: 0.55)),
  ),
)
