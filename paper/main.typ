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
In den frühen Tagen der kommerziellen Videospielindustrie, etwa ab 1970, war der Markt für Videospiele und Videospielkonsolen von einer deutlichen Instabilität gekennzeichnet #cite(<Ernkvist2008>).
In den USA brach dieser Markt 1983 mit dem Fall von Arcade-Systemen ein.
Die Veröffentlichung des Nintendo Entertainment System (NES) änderte diesen Trend.
Das NES ist eine Videospielkonsole, welche von  Nintendo 1983 auf den japanischen- , und 1985 auf den amerikanischen Markt gebracht wurde. 
Trotz anfänglicher Bedenken #cite(<Keizer1989>) leistete einen fundamentalen Beitrag zur Videospiel- und allgemeinen Popkultur, indem es zahlreiche bedeutende Franchises einführte, darunter Super Mario Bros., The Legend of Zelda und Tetris.

Da alte Videospielkonsolen nicht mehr produziert werden, gibt es nur begrenzte Möglichkeiten, diese Videospiele in ihrer originalen Form zu spielen.
Die beiden Alternativen sind Nachbildung der Hardware und Emulation, also das Simulieren der Hardware in Software.

== Problemstellung
In dieser Studienarbeit wird ein Emulator für das Nintendo Entertainment System entwickelt, welcher aus Gründen der historischen Präservation eine möglichst große Genauigkeit aufweisen soll.

Die Implementierung eines Emulators ist eine Herausforderung in der Softwareendwicklung, abhängig von der Komplexität der emulierten Hardware und der gewünschten Granularität.
Zwar ist die verwendete Hardware im NES recht simpel im Vergleich zu modernen Konsolen und Computern, durch die Hardwarelimitationen entstanden jedoch clevere und knifflige Verhalten des Systems.
Dazu zählen beispielsweise besonderen Mapper-Hardware in den Kassetten, unterschiedliches Hardwareverhalten in verschiedenen Regionen oder Rendern des Bildes durch die PPU.
== Arbeitsschritte
== Anmerkungen an Leser

= Grundlagen
== Computer-Architekturen
== Emulation
=== Typen // Interpreter, Recompiler
In der Entwicklung von Emulatoren gibt es grundsätzlich zwei verschiedene Ansätze um das Zielsystem zu emulieren, nämlich Interpreter und Recompiler. #cite(<Hill1968>)

Mit dem Interpreter-Ansatz wird versucht, die einzelnen Schritte, welche von der zu emulierenden CPU ausgeführt würden, in Software nachzustellen.
Der Emulator besitzt hierbei eine virtuelle Umgebung, welche besipielsweise Register, Status-Flaggen oder simulierten Speicher bietet.
Innerhalb dieser virtuellen Umgebung wird ein Fetch-Decode-Execute-Zyklus in der Programmlogik ausgeführt.
Um die Befehle aus dem Befehlssatz der Zielarchitektur ausführen zu können, liegt eine Implementierung im Programmcode des Emulators vor, welche nach dem Dekodieren des Kommandos aufgerufen wird.
Ein Vorteil dieser Vorangehensweise ist die vergleichsweise einfache Implementierung eines Interpreters, da es bei Rekompilierung zu schwierigen Problemen kommen kann, wie zum Beispiel selbst-modifizierendem Code.
Außerdem hat der Programmierer die volle Kontrolle über die virtuelle System-Umgebung des Emulators, wodurch eine höhere Genauigkeit bezüglich der internen Vorgänge des Zielsystems erreicht werden kann. 

Ein weiterer Ansatz für das Emulieren von Systemen ist der Recompiler.
Ein Recompiler übersetzt ein Programm für das Zielsystem in ein Programm für das Host-System.
Daraus resultiert, dass der Emulator keine virtuelle Umgebung verwalten muss, um den Zustand des Zielsystems zu speichern, da das rekompilierte Programm direkt auf der Hardware des Host-Systems ausgeführt wird.
Der Vorteil von dieser Technik ist, dass die volle Geschwindigkeit des Host-Systems ausgenutzt werden kann, da der Prozessor des Zielsystems nicht in Software emuliert werden muss.
//Ein weiterer Vorteil davon ist, dass der komplette Befehlssatz nicht manuell implementiert werden muss, was bei moderneren CISC-Systemen viel Arbeit bedeuten würde.
Im Idealfall wird als Ziel für die Rekompilierung eine Intermediate Representation gewählt, beispielsweise LLVM-IR, welche von LLVM auf unterschiedlichste Architekturen kompiliert werden kann. 
Diese Technik wird hauptsächlich gewählt, wenn die Performanz eines Interpreters für das gewünschte Zielsystem nicht mehr rentabel ist, beispielsweise für neuere Spielekonsolen mit komplexer Hardware (Playstation, Xbox, usw.).

Die Entscheidung, welche Art von Emulator am sinnvollsten zu implementieren ist, wird anhand der Entscheidungsmatrix (@emulator_type_decision_matrix) getroffen.
Im Vordergrund steht die Genauigkeit, welche mit dem zu wählenden Ansatz erreicht werden kann.
Hierbei gewinnt der Interpreter, da ein Recompiler die Ausführung des Programms an die CPU des Host-Systems auslagert.
Die Schnelligkeit des gewählten Ansatzes ist für die Implementierung des NES-Emulators hingegen eher zu vernachlässigen.
Da die Hardwarekomponenten des NES im Vergleich zu modernen Konsolen sehr simpel sind, reicht ein Interpreter-Ansatz aus, um die Taktfrequenz des NES zu erreichen.
Dies wird dadurch bestätigt, dass alle Emulatoren, welche in @architecture_related_works gezeigt werden, einen Interpreter-Ansatz verwenden.

#figure(table(
  columns: (auto, auto, auto, auto),
  inset: 10pt,
  align: horizon,
  table.header(
    [Kriterium], [*Gewicht*], [*Interpreter*], [*Recompiler*],
  ),
  "Genauigkeit", "3", text("1", weight: "bold"), "0",
  "Schwierigkeit", "2", text("1", weight: "bold"), "0",
  "Schnelligkeit", "1", "0", text("1", weight: "bold"),
  "Summe", "-", text("2", weight: "bold"), "1",
  "Gewichtete Summe", "-", text("5", weight: "bold"), "1" 
), caption: "Emulator-Typ-Entscheidungsmatrix") <emulator_type_decision_matrix>


=== Granularität // Cycle Accurate, Instriction Accurate, Frame Accurate
== Rust

= NES-Architektur
== CPU <architecture_cpu>
== PPU <architecture_ppu>
== APU <architecture_apu>
== Verwandte Arbeiten <architecture_related_works>

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

#set heading(numbering: none)
= Glossar

/ Zielsystem: Das System, welches emuliert werden soll
/ Host-System: Das System, auf welchem der Emulator ausgeführt wird

#bibliography("bibliography.bib")