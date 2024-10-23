#let format_date(date) = date.display("[day].[month].[year]")

#let key_value_table(..key_value_pairs) = [
  #let cells = key_value_pairs.pos().map(elem => ([#elem.at(0):#h(5mm)], elem.at(1))).flatten()
  #grid(columns: (auto, auto), gutter: 1.25em, ..cells)
]

#let titlepage(
  title, // contents
  author, // string
  course, // contents
  submissiondate, // datetime
  workperiod_from, // datetime
  workperiod_until, // datetime
  matr_num, // number
  supervisor, // string
) = [
  #v(7mm)
  #align(right)[#image("dhbw_icon.png", height: 2.2cm)]

  #set align(center)
  #v(15mm)
  *THEMA STUDIENARBEIT*
  #v(8mm)
  #text(size: 20pt)[
    *#title*
  ]
  #v(20mm)
  im Studiengang
  #v(5mm)
  #course
  #v(5mm)
  an der _Duale Hochschule Baden-Württemberg Mannheim_
  #v(10mm)
  von
  #v(5mm)

  #set align(left)
  #key_value_table(
    ([Name, Vorname], [#author]),
    ([Abgabedatum], format_date(submissiondate)),
  )
  #v(30mm)
  #key_value_table(
    (
      [Bearbeitungszeitraum],
      [#format_date(workperiod_from) - #format_date(workperiod_until)],
    ),
    ([Matrikelnummer, Kurs], [#matr_num, #course]),
    ([Wiss. Betreuer*in der Dualen Hochschule], [#supervisor]),
  )
]