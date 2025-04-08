#import "dhbw_template/lib.typ": dhbw_template

#show:  dhbw_template.with(
  title: [Konzipierung und Entwicklung eines präzisen NES-Emulators],
  author: "Freunscht, Marek",
  course: "TINF22IT1",
  submissiondate: datetime(year: 2025, month: 04, day: 15),
  workperiod_from: datetime(year: 2024, month: 10, day: 15),  
  workperiod_until: datetime(year: 2024, month: 04, day: 15),
  matr_num: 6622800,
  supervisor: "Bauer, Johannes, Prof. Dr.",
  abstract: include "abstract.typ",
)

#show link: set text(navy)

#[
  #set heading(numbering: none)
  = Glossar

  / Zielsystem: Das System, welches emuliert werden soll
  / Host-System: Das System, auf welchem der Emulator ausgeführt wird
  / Opcode: Eine Nummer, welche einen Befehl, mit zugehörigem Addressierungsmodus, eindeutig identifiziert <glossary_opcode>
  / Befehlssatz: Die Menge aus allen Opcodes, die ein Prozessor unterstützt <glossary_ins_set>
  / Netlist: Eine Liste von elektronischen Komponenten und den Verbindungen zwischen diesen
  / Little Endian: Die Reihenfolge von Werten im Speicher, die größer als 1 Byte sind, ist vom niedrigsten Byte zum höchsten Byte
  / High-Byte: Die oberen 8 Bit eines 16-Bit Werts
  / Low-Byte: Die unteren 8 Bit eines 16-Bit Werts
]

#include("sections/01-introduction.typ")
#include("sections/02-background.typ")
#include("sections/03-nes_architecture.typ")
#include("sections/04-implementation.typ")
#include("sections/05-verification.typ")
#include("sections/06-results.typ")
#include("sections/07-outlook.typ")

#bibliography("bibliography.bib")