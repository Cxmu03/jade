#let in-outline = state("in-outline", false)
#show outline: it => {
  in-outline.update(true)
  it
  in-outline.update(false)
}

#let flex-caption(long, short) = context {
  if in-outline.at(here()) { short } else { long }
}

// Languages other than "bash" do not provide coloring for only a function name
#let fn-name(name) = raw(name, lang: "bash")

#let visual6502-trace(
  columns,
  header,
  ..cells
) = table(
  columns: columns,
  header,
  ..cells
)

#let visual6502(content) = {
  set table(
    fill: (x, y) => {
      if y == 0 {
        return rgb("bbccff");
      }
      let colors = ("cfdaff", "e3e9ff", "e3e9ff", "ffffff");
      rgb(colors.at(calc.rem(y, 2) * 2 + calc.rem(x, 2)))
    },
  )

  content
}

#let validation-results(result, caption, ref-key, placement: top) = {
  show raw.where(block: true): set block(fill: rgb("cfdaff"), inset: 7pt, radius: 0.5em)
  [
    #figure(
      placement: placement,
      result,
      caption: caption
    ) #ref-key
  ]
}