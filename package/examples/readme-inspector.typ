#import "@local/typarium:0.1.0": font-showcase, inspector-theme, inspector-render

#set page(width: 190mm, height: auto, margin: 10mm)
#set text(size: 10pt)

#font-showcase(
  theme: inspector-theme + (
    sample-text: "Inspector preview text.",
    probe-glyph-ids: (0, 1, 2, 10),
    sample-codepoint-count: 4,
  ),
  render: inspector-render,
  fonts: ((path: read("Jaldi-Regular.ttf", encoding: none)),),
)
