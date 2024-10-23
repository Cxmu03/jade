#import "dhbw_template/lib.typ": dhbw_template

#show:  dhbw_template.with(
  title: [Konzipierung und Entwicklung eines präzisen NES-Emulators],
  author: "Freunscht, Marek",
  course: "TINF22IT1",
  submissiondate: datetime(year: 2025, month: 04, day: 15),
  workperiod_from: datetime(year: 2024, month: 10, day: 15),  
  workperiod_until: datetime(year: 2024, month: 04, day: 15),
  matr_num: 6622800,
  supervisor: "Gerhards, Holger, Prof. Dr.",
  abstract: include "abstract.typ",
)

= Einleitung
== Motivation
== Problemstellung
== Arbeitsschritte
== Anmerkungen an Leser

= Grundlagen
== Computer-Architekturen
== Emulation
=== Typen // Interpreter, Recompiler
=== Granularität // Cycle Accurate, Instriction Accurate, Frame Accurate
== Rust

= NES-Architektur
== CPU
== PPU
== APU
== Verwandte Arbeiten
=== FCEUX
=== MESEN
=== Visual 6502/2C02
=== Simple NES

= Emulation des 6502 Prozessors
== Anforderungen
== Design 
== Implementierung
== Verifikation und Validierung

= Emulation der 2C02 Picture Processing Unit
== Anforderungen
== Design
== Implementierung
== Verifikation und Validierung

= Emulation des RP2A03 Soundchips
== Anforderungen 
== Design
== Implementierung
== Verifikation und Validierung

= Entwicklung einer grafischen Oberfläche

= Ergebnisse und Diskussion

= Ausblick