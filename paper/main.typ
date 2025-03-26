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
]

#include("sections/introduction.typ")
#include("sections/background.typ")
#include("sections/nes_architecture.typ")
#include("sections/cpu_emulation.typ")
#include("sections/verification.typ")
#include("sections/results.typ")
#include("sections/outlook.typ")

#bibliography("bibliography.bib")