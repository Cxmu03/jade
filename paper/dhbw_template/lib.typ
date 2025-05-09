#import "titlepage.typ": titlepage
#import "declaration_of_authenticity.typ": declaration_of_authenticity
#import "@preview/hydra:0.3.0": hydra

#let dhbw_template(
  title: [Placeholder (title)],
  author: [Placeholder (author)],
  course: [Placeholder (course)],
  submissiondate: datetime(year: 2000, month: 1, day: 1),
  workperiod_from: datetime(year: 2000, month: 1, day: 1),
  workperiod_until: datetime(year: 2000, month: 1, day: 1),
  matr_num: 9999999,
  supervisor: [Placeholder (supervisor)],
  abstract: [Placeholder (abstract)],
  abstract_en: [Placeholder (abstract)],
  show_jump_to_table_of_contents: true,
  contents,
) = [
  // General formatting
  #set text(size: 12pt, font: "TeX Gyre Termes", lang: "de")
  #set page(paper: "a4", margin: 25mm)

  #show figure.where(kind: raw): set figure(supplement: [Quellcode])

  #show table.cell: set text(size: 10pt)

  #let in-outline = state("in-outline", false)

  #show outline: it => {
    in-outline.update(true)
    it
    in-outline.update(false)
  }

  // Titlepage
  #titlepage(
    title,
    author,
    course,
    submissiondate,
    workperiod_from,
    workperiod_until,
    matr_num,
    supervisor,
  )
  #pagebreak()

  // Start page numbering
  #set page(numbering: "I")
  #counter(page).update(1)

  // Declaration of authenticity
  #declaration_of_authenticity(title)
  #pagebreak()

  #align(horizon)[
  // Abstract
  #align(center)[
    #box(width: 80%)[
      #text(size: 1.2em)[#align(center)[*Zusammenfassung*]]
      #par(justify: true, first-line-indent: 0.2em)[
        #abstract
      ]
    ]
  ]

  #v(5em)

  #align(center)[
    #box(width: 80%)[
      #text(size: 1.2em)[#align(center)[*Abstract*]]
      #par(justify: true, first-line-indent: 0.2em)[
        #abstract_en
      ]
    ]
  ]
  ]
  #pagebreak()

  // Outline
  <outline_>
  #set par(leading: 1.20em)
  // Mark top-level headings as strong
  #show outline.entry.where(level: 1): it => {
    if it.element.func() == heading {
      v(10mm, weak: true)
      strong(it)
    } else {
      it
    }
  }
  #outline(indent: auto, depth: 3)
  #pagebreak()

  // List of figures
  #outline(title: [Abbildungsverzeichnis #v(7mm)], target: figure.where(kind: image))
  #pagebreak()

  // List of tables
  #outline(title: [Tabellenverzeichnis #v(7mm)], target: figure.where(kind: table))
  #pagebreak()

  #outline(title: [Quellcodeverzeichnis #v(7mm)], target: figure.where(kind: raw))
  #pagebreak()

  #let is_on_new_section_page = state("is_on_new_section_page", false)

  // General formatting for contents
  #set par(justify: true, first-line-indent: 5mm)
  #set heading(numbering: "1.1   ")
  #set page(
    numbering: "1",
    margin: ("top": 35mm, "left": 25mm, "right": 40mm, "bottom": 30mm),
    header: context {
      if is_on_new_section_page.get() {
        return []
      }
      let elems = query(selector(heading).before(here())).filter(elem => elem.outlined)
      if elems == () {
        []
      } else {
        elems.last()
      }
    } + line(length: 100%)
  )

  // Line height and resulting vertical spacings
  #let line_height = 1.0em
  #set par(leading: line_height, spacing: line_height)
  #show heading: it => {
    block(above: 2.0 * line_height, below: 1.2 * line_height)[ #it ]
  }

  // Reset page counter
  #counter(page).update(1)

  #show heading.where(level: 1): it => {
    is_on_new_section_page.update(true)
    colbreak(weak: true)
    it
    is_on_new_section_page.update(false)
  }

  #contents

];

