#import "../util.typ": flex-caption

= Emulation des 6502 Prozessors
Das folgende Kapitel beschreibt die Emulation der zentralen Recheneinheit des NES, den 6502.
In @requirements werden alle funktionalen Anforderungen an die Emulation und die Rahmenbedingungen dieser gestellt. 
@design beschreibt dann den Entwurf verschiedener Grundkonzepte der Emulation, wie einen Zustandsautomaten und ein Ausführungsmodell.
Die tatsächliche Implementierung wird dann in @emulation_implementation vorgestellt.
Hier werden verschiedene Konzepte wie die Ausführung von Befehlen oder Schwierigkeiten mit bestimmten Interrupts diskutiert.

== Anforderungen <requirements>
#[
#set heading(numbering: none)
=== *REQ-CPU-0* Zu emulierender Prozessor
Der zu emulierende Prozessor ist der Ricoh 2A03.
Da es sich beim Kern dieses Prozessors um einen Nachbau des 6502 handelt, kann der 6502 als Referenz für die genaue Funktionsweise verwendet werden.
Die zusätzlichen Audiokapazitäten, welcher der 2A03 besitzt, werden in einem separaten Modul emuliert.
Der Dezimalmodus, welcher im Kern des 6502 nicht vorhanden ist, soll im Emulator auch nicht emuliert werden.

=== *REQ-CPU-1* Granularität der Emulation <req-cpu-1>
Die anzustrebende Granularität der Emulation soll einzelne Zyklen darstellen können.
Der Emulator muss in einzelnen Taktzyklen vorangeschritten werden können.
Die einzelnen Zyklen, auch wenn diese innerhalb eines Befehls sind, müssen also völlig voneinander entkoppelt sein.

=== *REQ-CPU-2* Vollständigkeit des Befehlssatzes <req-cpu-2>
Der zu emulierende Befehlssatz soll auf die 151 offiziellen 6502 Opcodes begrenzt werden.  
Durch partielles Decoding der Befehlsdecoders entstehen zwar noch 115 inoffizielle Opcodes, diese sollen jedoch aufgrund steigender Komplexität nicht emuliert werden.
Dazu kommt, dass inoffizielle Opcodes nur von wenigen Spielen benutzt werden #cite(<6502unofficial>). 

=== *REQ-CPU-3* Korrektheit der Emulation <req-cpu-3>
Die Emulation des Zielsystems soll auf der Ebene der gefordeten Granularität (#link(<req-cpu-1>, "REQ-CPU-1")) mit dem Zielsystem übereinstimmen.
Das Kriterium für die Übereinstimmung zweier Zustände von Zielsystem und emuliertem System sind die Inhalte der Register (*x*, *y*, *a*, *sp*, *pc*), der Busse (*db*, *ab*) und des Ausführungszustands (*Fetch*, *Execute*, *FetchExecute*).

Die Überprüfung der Korrektheit soll mittels verschiedener Testverfahren geschehen.
Im Mittelpunkt steht der Vergleich der internen Zustände mit dem Simulator Visual6502, oder alternativ einem anderen Simulator, welcher auf der Netlist des Visual6502 basiert.
Des Weiteren sollen das Verhalten mit Test-Roms validiert werden.
=== *REQ-CPU-3.1* Implementierung der Pipeline <req-cpu-3.1>
Die Pipeline des 6502 (vgl. #ref(<6502_pipeline>)) soll durch den Emulator realitätsgetreu nachgebildet werden.
Der Zustand eines gleichzeitigen Fetch- und Execute-Taktes soll im Zustand des emulierten Prozessors klar erkennbar sein. 

=== *REQ-CPU-3.2* Implementierung von verspäteten Register-Writes <req-cpu-3.1>
Das Verhalten des 6502 mit verspäteten Register-Writes bei abschließenden ALU-Takten soll in der Emulation ebenfalls vorhanden sein. 
Der Nächste Execute-Takt soll diesen Schreibvorgang dann durchführen.

=== *REQ-CPU-4* Geschwindigkeit der Emulation <req-cpu-4>
Die erreichbare Geschwindigkeit der Emulation muss mindestens der Geschwindigkeit des echten Zielsystems entsprechen.
Im Falle des Ricoh 2A03 entspricht dies also etwa $1,8"MHz"$, wie in #ref(<nes_architecture_clock>) gezeigt wird.
Dies ist elementar für den Echtzeitbetrieb des Emulators, sodass Spiele realitätsnah gespielt werden können.

Zur Validierung der Emulationsgeschwindigkeit sollen Benchmarks durchgeführt werden, welche die geforderte Fähigkeit bestätigen.

=== *REQ-CPU-5* Programmierschnittstelle <req-cpu-5>
Die emulierte CPU soll eine vorgegebene minimale Programmierschnittstelle implementieren, dass externe Applikation den Emulator in verschiedene Anwendungsszenarien einbinden können.

#[
#set par(first-line-indent: 0pt)
*Initialisierung*
- #raw("new_and_init() -> Cpu", lang: "rust")
- #raw("reset(&mut Cpu) -> ()", lang: "rust")
]

*Kontrollfluss*
- #raw("step_cycle(&mut Cpu, &mut Bus) -> ()", lang: "rust")
- #raw("step_instruction(&mut Cpu, &mut Bus) -> ()", lang: "rust")
- #raw("set_pc(&mut Cpu, new_pc: u16) -> ()", lang: "rust")

Des weiteren sollen Register, Busse und weitere Pins nach außen hin öffentlich sein, sodass externe Anwendungen diese sowohl lesen als auch überschreiben können.
]
== Design <design>
=== Ausführungsmodell 
Die Ausführung von Befehlen wird im Emulator durch einen mikrocodeähnlichen Ansatz realisiert.
Dahinter steckt die Idee, dass ein einzelner Befehl in mehrere kleine und simple Mikrocodeschritte aufgeteilt werden kann. 
Im Ausführungsmodell des Emulators entspricht ein Mikrocodeschritt genau einem Taktzyklus.
Diese Aufteilung wird gewählt, da sie exakt der gewünschten Granularität des Emulators entspricht.
Eine feinere Gestaltung des Mikrocodes, etwa auf Basis von $phi.alt_1$ und $phi.alt_2$, ist aus Gründen der Performanz nicht gewünscht und ein gröbereres Design würde gegen die Anforderungen verstoßen.

Ein positiver Aspekt, welcher aus der Aufteilung der Befehle entsteht, ist die Wiederverwendbarkeit einzelner Schritte.
Besonders beim Fetch von Befehlsoperanden gleichen sich die Operationen des Prozessors.
Somit kann das redundante Implementieren der Schritte verhindert werden, womit der Entwicklungsaufwand gesenkt werden kann.

#v(10pt)
#grid(
  columns: (1fr, 1fr, 1fr, 1fr), 
  rows: (auto),
  [ 
    #figure(
      table(
        align: auto,
        columns: (auto),
        table.header("ADC abs"),
        "AbsOperand1",
        "AbsOperand2",
        "AbsOperand3",
        "Adc"
      )
      , caption: flex-caption("", "Mikrocodeschritte von ADC abs") 
    ) <microcode_adc_abs>
  ],
  [
    #figure(
      table(
        align: auto,
        columns: (auto),
        table.header("ADC abs,x"),
        "AbsOperand1",
        "AbsOperand2",
        "AbsXOperand",
        "AbsIndexedPageCross",
        "Adc"
      )
      , caption: flex-caption("", "Mikrocodeschritte von ADC abs,x") 
    ) <microcode_adc_abs_x>
  ],
  [
    #figure(
    table(
      align: auto,
      columns: (auto),
      table.header("BVC rel"),
      "RelOperand",
      "Bvc",
      "RelBranch1",
      "RelBranch2"
    )
    , caption: flex-caption("", "Mikrocodeschritte von BVC rel") 
    ) <microcode_bvc_rel>
  ],
  [
    #figure(
      table(
        align: auto,
        columns: (auto),
        table.header("BCC rel"),
        "RelOperand",
        "Bcc",
        "RelBranch1",
        "RelBranch2"
      ) 
      , caption: flex-caption("", "Mikrocodeschritte von BCC rel") 
    ) <microcode_bcc_rel>
  ]
) 

In @microcode_adc_abs und @microcode_adc_abs_x ist der gleiche Additionsbefehl mit zwei verschiedenen Addressierungsmodi zu sehen.
Hierbei können Mikrocodeschritte aus dem Fetchen des Operanden und der finalen Berechnung wiederverwendet werden.

In Tabelle @microcode_bvc_rel und @microcode_bcc_rel sind hingegen zwei unterschiedliche Verzweigungbefehle mit demselben Addressierungsmodus aufgelistet.
Auch hierbei können Schritte wiederverwendet werden, nämlich das Fetchen des Operanden und die Verarbeitung der Verzweigung.

Eine Herausforderung hierbei sind Befehle, welche unter bestimmten Bedingungen in der Ausführungslänge variieren.
Ein Beispiel hierfür sind bestimmte Verzweigungsbefehle, welche beim Überschreiten einer Seitengrenze einen zusätzlichen Taktzyklus benötigen.
=== Zustandsmodell
Der Kontrollfluss des Emulators wird intern über einen Zustandsautomaten realisiert, welcher die Übergänge zwischen den Zyklen koordiniert.
Für diesen Automaten werden insgesamt vier Zustände benötigt, wobei drei dieser Zustände für die Pipeline-Schritten aus @6502_pipeline benötigt werden.
Der vierte Zustand behandelt besonderes Verhalten, welches durch das *Reset*-Signal ausgelöst wird.

#figure(
  image("../resources/jade_states.svg"),
  caption: "Zustandsautomat des Emulators"
) <fig:jade_state_diagram>

In @fig:jade_state_diagram können die Zustände sowie die Übergänge zwischen diesen gesehen werden.
Durchgezogene Pfeile signifizieren hierbei Zustandsübergänge zwischen Zyklusgrenzen, während gestrichelte Pfeile einen Übergang innerhalb eines Zyklus darstellen.
Die Benennung der Zustände erfolgt hierbei folgendermaßen:
#list(
  indent: 15pt,
  [*F*: Fetch],
  [*E*: Execute],
  [*FE*: FetchExecute],
  [*RL*: ResetLow],
  )

Ein Fetch-Zyklus ist stets der erste Zyklus eines Befehls, welcher ausgeführt wird.
In diesem Zyklus wird der nächste Opcode aus dem Hauptspeicher gelesen und dekodiert.
Außerdem wird in diesem Zyklus überprüft, ob seit dem letzten Befehl eine neue Interruptanfrage gestellt wurde.
Ist dies der Fall, wird eine Abwandlung des BRK Befehls in den Ausführungskontext injiziert.
In einem Prozessorzustand mit regulärem Kontrollfluss wird der Fetch-Zyklus immer in einen Execute-Zyklus übergehen.
Ausnahmen zu diesem Fall werden mit dem ResetLow Zustand erläutert.

In den Execute-Zyklen geschieht das tatsächliche Ausführen eines Befehls.
Da die meisten Befehle des 6502 mehr als 2 Taktzyklen benötigen geht ein Execute-Zyklus meist in einen weiteren Execute-Zyklus über.
Falls der Emulator beim letzten Ausführungszyklus eines Befehls angekommen ist, kann dieser Zyklus nun bei regulärem Kontrollfluss in zwei verschiedene Zyklen übergehen, wie in Diagramm @fig:state_transition_algorithm gezeigt wird.
Wurde der aktuell ausgeführte Zyklus als Lesezyklus markiert, so geht der Emulator in den FetchExecute-Status über und dekodiert bereits den nächsten Befehl aus dem Speicher.
Dies ist konsistent mit dem Pipeline-Verhalten des 6502, welches in @6502_pipeline erläutert wurde.
Am Anfang des nächsten Zyklus findet dann der Übergang zum nächsten Execute-Zyklus statt, da der nächste Befehl in diesem Fall bereits im FetchExecute-Zyklus geholt wurde.
Ist der letzte Zyklus eines Befehls jedoch ein Schreibzyklus, so is es dem Prozessor nicht möglich, gleichzeitig einen Befehl aus dem Speicher zu fetchen.
Deshalb wird in diesem Fall ein Übergang zu einem regulären Fetch-Zyklus durchgeführt.

Der ResetLow-Zyklus ist eine besondere Art von Zyklus und entspricht dem Verhalten, wie es in @6502_interrupts zum Reset-Interrupt erklärt wird.
Dieser Zustand kann aus jedem anderen Zustand erreicht werden, da in jedem Zyklus überprüft wird, ob der Reset-Pin auf logisch Low ist.
Solange diese Bedingung erfüllt ist, wird dieser Zustand stets zu sich selbst übergehen.
Wird der Reset-Pin jedoch wieder auf High gesetzt, so geht der Zustand in einen Execute-Zyklus über.
Dies hat den Grund, dass ein Reset wie im echten Prozessor als zusätzlicher Befehl angedacht ist.
Da kein Fetch in diesem Fall nötig ist, kann der Befehl sofort ausgeführt werden.

== Implementierung <emulation_implementation>
Das folgende Kapitel beschreibt eine Auswahl verschiedener Algorithmen, Techniken und Entscheidungen, welche für die Implementierung des Emulators getroffen wurden. 
Obwohl die Präzision des Emulators höchste Priorität ist, gibt es Hardwareverhalten, was auf bestimmten Abstraktionsebenen nicht mehr vollständig korrekt nachgebildet werden kann, ohne andere Anforderungen einzuschränken.
Deshalb wurde in seltenen Fällen der Kompromiss eingegangen, von korrektem Hardwareverhalten abzuweichen.
Ist dies der Fall, so wird in dem jeweiligen Abschnitt darauf hingewiesen und es wird erklärt, warum das tatsächliche Hardwareverhalten schwierig zu emulieren ist.

=== Zustandsautomat <implementation_cycle_states>

#figure(
  image("../resources/jade_state_algorithm.svg", width: 105%),
  caption: [Algorithmus für die Ausführung eines Zyklus]
) <fig:state_transition_algorithm>

In @fig:state_transition_algorithm kann der Algorithmus gesehen werden, welcher einen Zyklus ausführt und dabei die Zustandsübergänge aus @fig:jade_state_diagram durchführt.

Der erste Schritt in einem Zyklus besteht daraus, den Ausführungsstatus (`execution_state`) und den Programmzähler (`pc`) mit dem nächsten Wert auszutauschen.
Hierbei wird eine Technik angewandt, welche gewisse Ähnlichkeiten mit Double Buffering hat, welches in der Bildverarbeitung angewandt wird /* TODO: Cite? Or remove, a bit far fetched*/.
In jedem Zyklus gibt es einen aktuellen Ausführungsstatus und den Status, welcher der nächste Zyklus annehmen soll, den `next_execution_state`.
Der nächste Zustand kann nämlich direkt aus dem aktuellen Status und weniger weiterer Daten bestimmt werden, wie im Folgenden erklärt wird.
Nach Ausführung eines Zyklus soll der Emulationszustand den gerade ausgeführten Zyklus korrekt widerspiegeln.
Demnach wird der aktuelle `execution_state` nicht direkt mit dem neuberechneten Zustand überschrieben.
Zu Anfang eines neuen Zyklus muss dieser dann jedoch mit dem neuen Zustand ausgetauscht werden, was direkt zu Beginn geschieht.
Wie der nächste Programmzähler bestimmt wird, wird genauer in @implementation_microcode_steps beschrieben.

Als nächster Schritt wird der aktuelle Ausführungszustand abgefragt.
Im einfachsten Fall ist dieser ein Fetch-Zyklus.
Dann kann einfach der nächste Befehl aus dem Speicher gelesen werden und der `next_execution_state` auf Execute gesetzt werden, da im normalen Kontrollfluss auf ein Fetch immer ein Execute folgen muss. 

Ist der aktuelle Zyklus ein Execute-Zyklus, so wird immer zuerst der nächste Mikrocodeschritt des momentanen Befehls ausgeführt.
Anschließend wird der Index des aktuellen Schritts mit der Länge des Befehls verglichen.
Ist dieser Schritt nicht der letzte Zyklus des Befehls, so kann der nächste Zyklus einfach als weiterer Execute-Zyklus markiert werden. 
War dies jedoch der letzte Schritt, so muss eine weitere Unterscheidung gemacht werden, welche in @6502_pipeline bereits angesprochen wurde.
Falls in diesem Zyklus von der CPU in den Speicher geschrieben wurde, so wird der nächste Zyklus als Fetch festgelegt.
Wurde jedoch nicht geschrieben, so kann die CPU neben dieser Operation noch in diesem Zyklus den nächsten Opcode aus dem Speicher lesen.
In diesem Fall wird der nächste Befehl gelesen, der aktuelle Ausführungsstatus auf FetchExecute gesetzt und der nächste Status wird als normaler Execute-Zyklus festgelegt, da ein Fetch bereits ausgeführt wurde.

Der einzige Zustand, der an diesem Punkt nicht erscheinen sollte, zumindest mit internem Kontrollfluss, ist FetchExecute.
Dies hat den Grund, dass der Zustand nur während der Ausführung eines Zyklus auf von Execute auf FetchExecute gesetzt werden kann und der darauffolgende Zyklus immer ein Execute ist.
Wie im vorherigen Absatz erklärt, wird also `next_execution_state` immer gleich Execute gesetzt, wenn ein `execution_state` von FetchExecute erreicht wird. 
Daraus resultiert, dass der FetchExecute-Zustand am Anfang des nächsten Zyklus immer direkt mit Execute ausgetauscht wird. 
Sollte dieser Zustand jedoch trotzdem erreicht werden, wird dies als ein invalider Zustand gewertet und die Ausführung wird abgebrochen.

Die letzte Möglichkeit für den Zustand ist ResetLow.
Ist dieser Zustand erreicht, führt der Emulator immer einen Lesezyklus durch.
Dies ist zwar nicht exakt konsistent mit dem Verhalten des echten Prozessors, worauf in @implementation_interrupts genauer eingegangen wird.
Anschließend wird überprüft, ob der Reset-Pin wieder auf logisch 1 gezogen wurde.
Ist dies der Fall, wird ein Reset-Befehl vorbereitet, indem dieser geladen wird und der nächste Ausführungszustand auf Execute gesetzt wird.
Blieb der Reset-Pin hingegen auf logisch 0, so verharrt der Emulator im ResetLow-Zustand.

Die letzte Verzweigung in der Ausführung eines Zyklus geschieht nach dem Setzen des `next_execution_state` auf Fetch oder Execute, wenn der Reset-Pin auf logisch 0 gezogen wurde.
Dies führt dazu, dass der `next_execution_state` auf ResetLow gezwungen wird.
Wie in @6502_interrupts beschrieben, wartet ein Reset nicht auf die Beendigung des aktuellen Befehls, sondern unterbricht diesen.

=== Mikrocodeschritte <implementation_microcode_steps>
=== Interrupts <implementation_interrupts>
Die Implementierung von Interrupts spaltet aufgrund des unterschiedlichen Verhaltens zweierlei.
Auf der einen Seite stehen die regulären Interrupts, NMI und IRQ, da diese sich bezüglich des Pollings gleich verhalten und sich leicht in den bestehenden Kontrollfluss einbinden lassen.
Auf der anderen Seite ist der Reset-Interrupt, welcher den gesamten Kontrollfluss des Emulators beeinflusst.

Obwohl sich das genaue Hardwarepolling den beiden Interrupts IRQ und NMI unterscheidet (Flankengetriggert und Zustandsgetriggert), ist dies für den Emulator nicht weiter interessant.
Im Emulator kann von außen eine Flagge gesetzt werden, welche das Eingehen eines Interrupts signalisiert.
Das tatsächliche Pollen dieser Flagge geschieht dann vor dem Fetchen eines neuen Befehls, da diese Interrupts auf die Beendigung des aktuellen Befehls warten, wie in @6502_interrupts erklärt.
Da diese Interrupts jedoch genau wie andere Befehle implementiert werden, können diese Interrupt-Befehle einfach aus der Fetch-Routine zurückgegeben werden. 
Falls beide Interrupts eingereiht wurden, so wird nach Präzedenz der NMI-Interrupt zurückgegeben.

#figure(
  grid(
    columns: (1fr, 1fr, 1fr),
    table(
      columns: (auto),
      [*IRQ*],
      [Read],
      [PushPch],
      [PushPcl],
      [PhpBrk],
      [IrqVecLo],
      [IrqVecLo],
      [Read]
    ),
    table(
      columns: (auto),
      [*NMI*],
      [Read],
      [PushPch],
      [PushPcl],
      [PhpBrk],
      [NmiVecLo],
      [NmiVecLo],
      [Read]
    ),
    table(
      columns: (auto),
      [*Reset*],
      [Read],
      [Read],
      [Read],
      [ReadStack],
      [ReadStackDec],
      [ReadStackDec],
      [ResetVecLo],
      [ResetVecHi],
      [Read]
    )
  ),
  caption: [Mikrocoderoutinen der Interrupts]
) <interrupts_microcode>

In @interrupts_microcode kann die genaue Mikrocode-Implementierung dieser Interrupts eingesehen werden.
Hierbei fällt auf, dass IRQ und NMI in der Implementierung weitgehend übereinstimmen.
Der erste Zyklus ist ein Dummy-Read, der keinen signifikanten Einfluss auf den Emulatorzustand hat, da das gelesene Byte verworfen wird.
Darauf folgt in Zyklus 2 und 3 das Pushen des Programmzählers auf den Stack.
Da der 6502 ein Little-Endian-Prozessor ist und der Stack nach unten wächst, muss zuerst das High-Byte und danach das Low-Byte des Programmzählers gepushed werden.
Anschließend wird noch das Prozessorstatuswort unterhalb des Programmzählers auf den Stack geschrieben.
Der nächste Schritt besteht daraus, die Adresse der Interrupt Service Routine aus dem Interrupt Vector Table zu holen.
Da der IRQ- und NMI-Interrupt unterschiedliche Serviceroutinen haben, unterscheidet sich hier der Mikrocode in Schritt 4 und 5.
Am Ende der beiden Befehle wird noch ein weiterer Dummy-Read ausgeführt.

=== Reset
Die Implementierung des Reset-Interrupts unterscheidet sich signifikant von den vorherigen Interrupts, da dieser ein deutlich anderes Verhalten zeigt.
Dies betrifft zum Einen den Tatsächlichen Reset-Befehl in der Ausführung, zum Anderen aber auch den ResetLow-Zustand des Emulators.

Laut Datenblatt wird jegliche Befehlsexekution seitens des Prozessors abgebrochen, sobald der Reset-Pin auf Logisch-Low gezogen wird #cite(<Data6502>). 
Obwohl sich dies simpel anhört, entsteht hier komplexes Verhalten, welches den Prozessor in einen stabilen Zustand bringt.
// TODO: Show weird reset behaviour here
Da es mit den getroffenen Anforderungen nicht möglich ist, das tatsächliche Verhalten sinnvoll zu emulieren, wird dieser Vorgang im Emulator deutlich vereinfacht dargestellt.
Im Reset-Low Zustand führt der Emulator in jedem Zyklus einen einfachen Dummy-Read durch.

Eine Auffälligkeit im Mikrocode des Reset-Befehls ist seine größere Länge im Vergleich zu den weiteren Interrupt-Routinen.
Die zwei zusätzlichen Dummy-Read-Zyklen kommen daher, dass der Prozessor nach detektieren der steigenden Flanke noch zwei Zyklen braucht, bis die Reset-Routine tatsächlich durchgeführt werden kann #cite(<Data6502>).
Diese beiden Zyklen werden im Emulator dann als Zyklen des Reset-Befehls gewertet, was zwar ein semantischer Unterschied ist, aber keine Auswirkung auf die Korrektheit der Emulation hat.
Dies ist ein reine modellierungsbedingte Entscheidung, welche besser in das Zustandsmodell des Emulators passt.
Die nächsten drei Zyklen sind Lesezyklen, welche eine Leseoperation auf dem Stack durchführen und den Stackpointer anschließend dekrementieren.
Anzumerken ist, dass diese gelesenen Werte jeweils anschließend verworfen werden, es sich hier also auch um Dummy-Reads handelt - allerdings auf dem Stack.
Der genaue Grund hierfür ist nicht genau geklärt.
Eine mögliche Vermutung ist, dass diese Schritte genau den Schritten 2-4 der IRQ- und NMI-Routinen entsprechen, jedoch mit dem Read-Pin auf 1 (Leseoperation), statt auf 0 (Schreiboperation).
Dies könnte darauf abzielen, Teile der Kontrollogik so minimal wie möglich zu halten und somit durch Wiederverwendung von Komponenten minimale Produktionskosten zu erreichen.
Analog zu den Interrupts wird anschließend der Reset-Vektor gelesen, welcher die Adresse enthält, die im Übergang zum nächsten Zyklus in den Programmzähler geschrieben wird.
Der letzte Zyklus ist dann ein Dummy-Read, welcher den ersten Wert am neuen Programmzähler liest.
Da dies aber ein Lesezyklus ist, liegt ein FetchExecute-Zyklus vor und der Dummy-Read wird mit dem Fetch des nächsten Befehls überlappt.