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