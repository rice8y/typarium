#import "@local/typarium:0.1.0": font-showcase

#set page(width: 180mm, height: auto, margin: 10mm)
#set text(size: 10pt)

#let poster-theme = (
  color-bg: rgb("0f1720"),
  color-primary: rgb("f8fafc"),
  color-muted: rgb("94a3b8"),
  stroke-card: 0.6pt + rgb("334155"),
  card-inset: 1.2em,
  size-title: 1.3em,
  size-meta: 0.78em,
  size-sample: 2.1em,
  gap-meta: 0.4em,
  gap-sample: 0.9em,
  sample-text: "Theme drives shared renderer options.",
)

#let poster-render = it => {
  let ft = it.font-text
  let opt = (key, default) => it.at(key, default: it.theme.at(key, default: default))

  block(
    breakable: false,
    width: 100%,
    fill: opt("color-bg", black),
    stroke: opt("stroke-card", none),
    inset: opt("card-inset", 1em),
    radius: 0.6em,
    [
      #text(size: opt("size-title", 1.2em), weight: "bold", fill: opt("color-primary", white))[#it.name]
      #v(opt("gap-meta", 0.4em))
      #text(size: opt("size-meta", 0.8em), fill: opt("color-muted", white))[
        family=#it.render-name / weight=#it.weight / style=#it.style
      ]
      #v(opt("gap-sample", 0.8em))
      #ft(size: opt("size-sample", 2em), fill: opt("color-primary", white))[
        #opt("sample-text", "Fallback specimen")
      ]
    ],
  )
}

#font-showcase(
  theme: poster-theme,
  render: poster-render,
  fonts: ((path: read("Jaldi-Regular.ttf", encoding: none)),),
)
