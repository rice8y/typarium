#import "@local/typarium:0.1.0": font-showcase, default-theme

#set page(width: 180mm, height: auto, margin: 10mm)
#set text(size: 10pt)

#font-showcase(
  theme: default-theme + (
    paragraph-text: lorem(20),
    waterfall: (1.4em, 2.4em),
    card-inset: 2em,
    color-primary: rgb("202124"),
  ),
  fonts: ((path: read("Jaldi-Regular.ttf", encoding: none)),),
)
