#import "../util.typ": flex-caption

= Hintergrund
== Computer-Architektur
Im folgenden werden einige grundlegende Konzepte und Hintergründe zur Computer-Architektur vorgestellt, 
=== Zweierkomplement <basics_twos_complement>
Das Zweierkomplement ist eine Möglichkeit, positive und negative Zahlen in einem $n$-Bit Wert darzustellen #cite(<LaMeres_2019>). 
Um das Zweierkomplement eines Wertes $a$ zu bestimmen, wird $-a=2^n-a$ gesetzt.
Dies ist Äquivalent zu einer bitweisen Negierung des Wertes gefolgt von einer Addition.
$
  -a=overline(a)+1=2^n-a
$
Damit können Zahlen von $-2^n$ bis $2^n-1$ dargestellt werden.
Der große Vorteil des Zweierkomplements ist, dass eine Subtraktion keine besondere Schaltung benötigt, da die Subtraktion $a-b$ äquivalent ist zu $a+(-b)$, also $a+(overline(b)+1)$.

Eine weitere Methode, welche zur Visualisierung des Zweierkomplements benutzt werden kann, ist eine unterschiedliche Gewichtung der Stellenwerte der Bits.
Ein n-Bit Wert, welcher nur als positiv interpretiert wird, hat die folgenden Stellenwerte:
#figure(
table(
  columns: (auto,) * 5,
  [$2^(n-1)$], [$2^(n-2)$], [...], [$2^1$], [$2^0$]
)
)

Im Zweierkomplement ändert sich der Stellenwert des höchsten Bits:

#figure(
  table(
    columns: (auto,) * 5,
    [$-2^(n-1)$], [$2^(n-2)$], [...], [$2^1$], [$2^0$]
  )
)

Sei nun ein Wert $a=11000011_2$ gegeben.
Mit der Interpretierung als rein positiver Wert würde dies $1 dot 2^7+1 dot 2^6+1 dot 2^1+1 dot 2^0=195$.
Wird der Wert aber als Zweierkomplement interpretiert, ergibt sich $1 dot (-2^7)+1 dot 2^6+1 dot 2^1+1 dot 2^0=-61$.

=== Pipeline <basics_architecture_pipeline>
Eine Befehls-Pipeline eines Prozessors beschreibt die parallele Durchführungen von aufeinander folgenden Befehlen, indem diese in Teilaufgaben zersetzt werden.
Diese Teilaufgaben sind beispielsweise der Befehls-Fetch, das Befehls-Dekodieren, die Befehls-Ausführung und das Write-Back, also zurückschreiben der Ergebnisse des Befehls.
Mit einer Pipeline kann also das Befehls-Fetchen des zweiten Befehls bereits ausgeführt werden, während der Prozessor den ersten Befehl dekodiert, wie in #ref(<fig_pipeline>) gesehen werden kann.

#figure(placement: top, image("../resources/pipeline.png", width: 105%), caption: flex-caption([Befehls-Pipeline, aus #cite(<Tanenbaum2013>)], [Befehls-Pipeline])) <fig_pipeline>

Hieraus entsteht durch die Parallelisierung im Prinzip eine große Beschleunigung der Ausführungsgeschwindigkeit.
Jedoch sind Verzweigungen im Code ein Problem für Pipelines. 
Da die CPU nicht wissen kann, welche Verzweigungen gewählt werden, müssen diese geraten werden.
Wird von der CPU nicht die richtige Verzweigung gewählt, muss diese Pipeline geleert werden und neu aufgebaut werden.
Hierbei handelt es sich jedoch um eine deutlich kompliziertere Pipeline, als die Pipelines, welche im Laufe dieser Arbeit behandelt werden.

== Emulation
Der Begriff Emulation beschreibt die Fähigkeit eines Computersystems (Host-System), Programme, welche für ein anderes Computersystem (Gast-System) geschrieben wurden, auszuführen.
Nach Mallach #cite(<Mallach1975>) kann ein Emulator definiert werden als jegliche Hardware, Software und Mikroprogramme, welche einem Computer hinzugefügt werden, um eine solche Ausführung von Software für ein Gast-System zu ermöglichen.

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


=== Granularität und Genauigkeit // Cycle Accurate, Instriction Accurate, Frame Accurate
Die interne Funktionsweise eines Emulators hängt von vielen verschiedenen Faktoren abwie zum Beispiel der gewünschten Genauigkeit oder Performanz.
Zwei dieser Faktoren sind die Genauigkeit und Granularität der Ausführung des Emulators.
Für diese Begriffe gibt es keine allgemein akzeptierte Definition oder Unterteilung, stattdessen hierfür wird sich an #cite(<EmulationAccuracy>) orientiert.

Die Genauigkeit beschreibt, auf welcher Abstraktionsebene der Emulator arbeitet.
Dies wird besonders wichtig, wenn die zu emulierende Hardware beispielsweise sehr komplex ist.
Emulatoren mit geriner Genauigkeit arbeiten auf einer sehr hohen Abstraktionsebene und versuchen die reine Funktionsweise der Hardware nachzuahmen, anstatt die Hardware in der Software nachzubilden.
Diese Vorangehensweise ist bei komplexer Hardware so elementar, da die Nachbildung der Hardware sonst zu großen Performanzeinbußen führen würde.

Ab der mittleren Genauigkeit eines Emulators kommt auch die Granularität ins Spiel. 
Die Granularität beschreibt, in welchen atomaren Einheiten der Ausführungszustand von außen vorangeschritten werden kann.
Im Kontext eines Prozessors können dies zum Beispiel Befehle, Taktzyklen oder sogar Subzyklen sein.

Ein Emulator mittlerer Genauigkeit könnte beispielsweise eine Granularität auf Befehlsebene haben.
Hierbei wird ein Befehl immer als eine Einheit ausgeführt und ein Voranschreiten in kleineren Einheiten ist unmöglich.
Manche Emulatoren versuchen den Schein zu wahren, dass dies möglich sei, indem die ersten Zyklen eines Befehls nichts getan wird und im letzten Zyklus der ganze Befehl ausgeführt wird#footnote(link("https://github.com/GarettCooper/emulator_6502/blob/0e0a8cd0392b9e8694f4c5ba022d94f154fc65ca/src/lib.rs#L277-L322")).
Da die Hardware mit dieser Vorgehensweise nicht allzu genau modelliert werden muss, kann somit einfach eine gute Performanz gewährleistet werden.

Um eine hohe Genauigkeit zu erreichen, muss das Hardwareverhalten genauer studiert werden.
Unter diese Genauigkeitsstufe fallen zyklenbasierte und genauere Methoden (bspw. @visual6502), von denen in diesem Kapitel aber nur die zyklenbasierten Methoden behandelt werden.
In #cite(<EmulationAccuracy>) findet hier eine weitere Aufteilug in partielle Zyklengenauigkeit und vollständige Zyklengenauigkeit.
Die partielle Zyklengenauigkeit beschreibt eine Emulation mit den einzelnen Taktzyklen als atomare Einheit.
Die gesamte Arbeit, welche in einem Taktzyklus geschehen würde, passiert auf einmal.
Für eine vollständige Zyklengenauigkeit muss die Hardware noch weiter nachgebildet werden, da diese eine genaue Replikation der Timings zwischen allen Hardwarekomponenten erfordert.

Da für den hier entwickelten Emulator eine hohe Genauigkeit gewünscht wird, jedoch auch ein Kompromiss mit der Performanz gefunden werden soll, wird dieser mit einer partiellen Zyklengenauigkeit entwickelt.

== Verwandte Arbeiten <related_works>
In diesem Kapitel werden einige verwandte und bereits existierende Emulatoren für den 6502-Prozessor und das NES vorgestellt um eine Vergleichsbasis herzustellen.
=== Visual 6502 <visual6502>
Visual 6502 ist ein Simulator des 6502 Prozessors auf Transistor-Ebene, welcher von Brian Silverman, Barry Silverman, Greg James und Ed Spittles entwickelt wurde.
Die Basis hierfür waren Die-Shots, also mikrofotografische Bilder des Chips.
Anhand dessen wurde die Anordnung der Transistoren durch manueller Analyse der Bilder reverse-engineered.

Aus der Simulation der einzelnen Transistoren ergibt sich eine sehr hohe Genauigkeit.
Die Zustände der Register und Busse werden mit Halbzyklus-Granularität ausgegeben.
Jedoch führt dies auch zu einer sehr geringen erreichten Taktfrequenz, da die Simulation sehr Zeitaufwendig ist.

Die originale Version des Visual 6502 ist in Javascript geschrieben, was die Geschwindigkeit der Simulation negativ beeinflusst.
Aus diesem Grund wurde die Simulation in verschiedene Sprachen geported, wie zum Beispiel mit dem #link("https://github.com/mist64/perfect6502", "perfect6502").
Dieses Projekt übersetzt die Simulation in die Programmiersprache C, übernimmt aber die Netlist des Visual 6502 Projekts. 
Laut Angaben des Repositories können Simulationsgeschwindigkeiten von etwa 30kHz erreicht werden, im Gegensatz zu etwa 50Hz im Javascript-basierten Visual 6502 #footnote("Die 50Hz Angabe stammt aus Erfahrungswerten mit dem Online-Simulator, soll jedoch " + [*nicht*] + " als repräsentativer Benchmark für diesen Simulator dienen, da die Geschwindigkeit von verschiedenen Faktoren abhängt. Dies ist eher als Vergleichswert bezüglich der Größenordnung angedacht."). 
Genauere Benchmarkdaten zum perfect6502 können im @benchmarks nachgelesen werden.

#figure(image("../resources/visual_6502_screenshot.png", width: 60%), caption: "Digitales Abbild des 6502 in Visual 6502")

=== FCEUX
Ein bekannter Emulator für das NES ist der FCEUX, welcher nach eigenen Angaben eine "hohe" Emulationsgenauigkeit besitzt #cite(<fceux>).
Besonders beliebt ist dieser Emulator, da er eine große Anzahl an Manipulationsoptionen für den aktuellen Ausführungszustand bietet, sowie dediziertes Tooling für Tool-Assisted Speedruns#footnote("Automatisierte Agenten, welche ein Spiel so schnell wie möglich durchspielen") und Debugging mitliefert.

#figure(
  image(
    "../resources/fceux_screenshot.png", width: 90%
  ),
  caption: flex-caption([Benutzerschnittstelle von FCEUX, aus #cite(<fceux>)], [Benutzerschnittstelle von FCEUX])
)

Auch wenn die Schnittstelle der CPU-Implementierung von FCEUX nach außen hin vermuten lässt, dass die Ausführung tatsächlich zyklengenau ist, wird diese nur nachgeahmt.
Die Befehle werden intern in einem atomaren Schritt durchgeführt#footnote(link("https://github.com/TASEmulators/fceux/blob/master/src/ops.inc")).

=== MESEN/MESEN 2
Mesen2 ist ein weiterer, sehr bekannter Multiplattform-Emulator.
Außer dem NES werden noch andere, sehr bekannte Spielekonsolen unterstützt, wie zum Beispiel GameBoy (GB), GameBoy Advance (GBA), oder das Super Nindendo Entertainment System (SNES).

Die interne Funktionsweise von Mesen ist analog zu FCEUX, da die atomare Ausführung nur auf Befehlsebene stattfindet

#figure(
  placement: top,
  image("../resources/mesen_screenshot.png"),
  caption: flex-caption([Benutzerschnittstelle von Mesen2, aus #cite(<mesen>)], [Benutzerschnittstelle von Mesen2])
)

=== BeesNES
BeesNES ist ein NES Emulator, welcher eine Subzyklen-Genauigkeit anstrebt #cite(<beesNES>).
Dies bedeutet, dass jeder Takt der beiden Clocksignale (siehe @6502_clock_sec) einzeln emuliert wird.
Obwohl dieses Projekt relativ jung ist, wurde bereits ein breiter Teil der Funktionalität für einen vollständig funktionalen NES-Emulator implementiert.

#pagebreak()