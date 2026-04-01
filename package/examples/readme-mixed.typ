#import "@local/typarium:0.1.0": font-showcase

#set page(width: 180mm, height: auto, margin: 10mm)
#set text(size: 10pt)

#font-showcase(
  columns: 2,
  theme: (
    sample-text: "Shared default sample text.",
  ),
  fonts: (
    "Libertinus Serif",
    (path: read("Jaldi-Regular.ttf", encoding: none), sample-text: "Parsed from file"),
    (name: "Courier", display-name: "Courier (Manual Label)"),
  ),
)
