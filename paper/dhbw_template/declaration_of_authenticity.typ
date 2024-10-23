#let labeled_field(label, width) = [
  #v(2.2em)
  #line(length: width)
  #v(-0.3em)
  #h(5mm)
  #label
];

#let declaration_of_authenticity(title) = [
  #box(
    width: 100%,
    stroke: black,
    inset: 4mm,
  )[
    #set par(justify: true)
    #align(center)[*Erklärung zur Eigenleistung*]
    Ich versichere hiermit, dass ich meine Projektarbeit mit dem \
    #v(3mm)
    THEMA \
    *#title*
    #v(2mm)
    selbstständig verfasst und keine anderen als die angegebenen Quellen und
    Hilfsmittel benutzt habe. Ich versichere zudem, dass die eingereichte
    elektronische Fassung mit der gedruckten Fassung übereinstimmt.

    #grid(
      columns: (40%, 60%),
      labeled_field([Ort, Datum], 90%),
      labeled_field([Unterschrift], 90%),
    )
  ]
]