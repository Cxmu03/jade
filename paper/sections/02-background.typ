#import "../util.typ": flex-caption

= Hintergrund
== Computer-Architektur
Im folgenden werden einige grundlegende Konzepte und Hintergründe zur Computer-Architektur vorgestellt, 
=== Pipeline <basics_architecture_pipeline>
Eine Befehls-Pipeline eines Prozessors beschreibt die parallele Durchführungen von aufeinander folgenden Befehlen, indem diese in Teilaufgaben zersetzt werden.
Diese Teilaufgaben sind beispielsweise der Befehls-Fetch, das Befehls-Dekodieren, die Befehls-Ausführung und das Write-Back, also zurückschreiben der Ergebnisse des Befehls.
Mit einer Pipeline kann also das Befehls-Fetchen des zweiten Befehls bereits ausgeführt werden, während der Prozessor den ersten Befehl dekodiert, wie in #ref(<fig_pipeline>) gesehen werden kann.

#figure(image("../resources/pipeline.png", width: 105%), caption: flex-caption([Befehls-Pipeline, aus #cite(<Tanenbaum2013>)], [Befehls-Pipeline])) <fig_pipeline>

Hieraus entsteht durch die Parallelisierung im Prinzip eine große Beschleunigung der Ausführungsgeschwindigkeit.
Jedoch sind Verzweigungen im Code ein Problem für Pipelines. 
Da die CPU nicht wissen kann, welche Verzweigungen gewählt werden, müssen diese geraten werden.
Wird von der CPU nicht die richtige Verzweigung gewählt, muss diese Pipeline geleert werden und neu aufgebaut werden.

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
Im Idealfall wird als Ziel für die Rekompilierung eine Intermediate Representation gewählt, beispielsweise LLVM-IR, welche von LLVM auf unterschiedlichste Architekturen kompiliert werden kann. 
Diese Technik wird hauptsächlich gewählt, wenn die Performanz eines Interpreters für das gewünschte Zielsystem nicht mehr rentabel ist, beispielsweise für neuere Spielekonsolen mit komplexer Hardware (Playstation, Xbox, usw.).

Die Entscheidung, welche Art von Emulator am sinnvollsten zu implementieren ist, wird anhand der Entscheidungsmatrix (@emulator_type_decision_matrix) getroffen.
Im Vordergrund steht die Genauigkeit, welche mit dem zu wählenden Ansatz erreicht werden kann.
Hierbei gewinnt der Interpreter, da ein Recompiler die Ausführung des Programms an die CPU des Host-Systems auslagert.
Die Schnelligkeit des gewählten Ansatzes ist für die Implementierung des NES-Emulators hingegen eher zu vernachlässigen.
Da die Hardwarekomponenten des NES im Vergleich zu modernen Konsolen sehr simpel sind, reicht ein Interpreter-Ansatz aus, um die Taktfrequenz des NES zu erreichen.
Dies wird dadurch bestätigt, dass alle Emulatoren, welche in @related_works gezeigt werden, einen Interpreter-Ansatz verwenden.

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
), caption: [Emulator-Typ-Entscheidungsmatrix]) <emulator_type_decision_matrix>


=== Granularität // Cycle Accurate, Instriction Accurate, Frame Accurate
#cite(<EmulationAccuracy>)
== Rust

== Verwandte Arbeiten <related_works>
=== Visual 6502 <visual6502>
Visual 6502 ist ein Simulator des 6502 Prozessors auf Transistor-Ebene, welcher von Brian Silverman, Barry Silverman, Greg James und Ed Spittles entwickelt wurde.
Die Basis hierfür waren Die-Shots, also mikrofotografische Bilder des Chips.
Anhand dessen wurde die Anordnung der Transistoren durch manueller Analyse der Bilder reverse-engineered.
#figure(image("../resources/visual_6502_screenshot.png", width: 65%), caption: "Digitales Abbild des 6502 in Visual 6502")

Aus der Simulation der einzelnen Transistoren ergibt sich eine sehr hohe Genauigkeit.
Die Zustände der Register und Busse werden mit Halbzyklus-Granularität ausgegeben.
Jedoch führt dies auch zu einer sehr geringen erreichten Taktfrequenz, da die Simulation sehr Zeitaufwendig ist.

Die originale Version des Visual 6502 ist in Javascript geschrieben, was die Geschwindigkeit der Simulation negativ beeinflusst.
Aus diesem Grund wurde die Simulation in verschiedene Sprachen geported, wie zum Beispiel mit dem #link("https://github.com/mist64/perfect6502", "perfect6502").
Dieses Projekt übersetzt die Simulation in die Programmiersprache C, übernimmt aber die Netlist des Visual 6502 Projekts. 
Laut Angaben des Repositories können Simulationsgeschwindigkeiten von etwa 30kHz erreicht werden, im Gegensatz zu etwa 50Hz im Javascript-basierten Visual 6502 #footnote("Die 50Hz Angabe stammt aus Erfahrungswerten mit dem Online-Simulator, soll jedoch " + [*nicht*] + " als repräsentativer Benchmark für diesen Simulator dienen, da die Geschwindigkeit von verschiedenen Faktoren abhängt. Dies ist eher als Vergleichswert bezüglich der Größenordnung angedacht."). 
Genauere Benchmarkdaten zum perfect6502 können im @benchmarks nachgelesen werden.
=== FCEUX
=== MESEN/MESEN 2
=== Simple NES