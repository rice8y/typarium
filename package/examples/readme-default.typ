#import "@local/typarium:0.1.0": font-showcase

#set page(width: 180mm, height: auto, margin: 10mm)
#set text(size: 10pt)

#font-showcase(
  theme: (
    sample-text: "Minimal local specimen.",
    show-glyphs: true,
    waterfall: (1.0em, 1.4em, 2.0em),
  ),
  fonts: ((path: read("Jaldi-Regular.ttf", encoding: none)),),
)
