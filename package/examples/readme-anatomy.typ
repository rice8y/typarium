#import "@local/typarium:0.1.0": font-showcase, anatomy-theme, anatomy-render

#set page(width: 180mm, height: auto, margin: 10mm)
#set text(size: 10pt)

#font-showcase(
  fonts: (
    (path: read("Jaldi-Regular.ttf", encoding: none)),
    (
      path: read("PlaywriteDKUloopetGuides-Regular.ttf", encoding: none),
      theme: (visual-sample-inset-bottom: 5em),
    ),
  ),
  theme: anatomy-theme,
  render: anatomy-render,
)
