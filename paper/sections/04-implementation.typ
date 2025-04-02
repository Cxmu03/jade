#import "../util.typ": flex-caption

= Emulation des 6502 Prozessors
== Anforderungen
#[
#set heading(numbering: none)
=== *REQ-CPU-0* Zu emulierender Prozessor
Der zu emulierende Prozessor ist der Ricoh 2A03.
Da es sich beim Kern dieses Prozessors um einen Nachbau des 6502 handelt, kann der 6502 als Referenz für die genaue Funktionsweise verwendet werden.
Da der Kern des 2A03 jedoch keinen Dezimalmodus besitzt, soll dieser Modus hier auch nicht ermuliert werden. 

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
== Design 
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
  figure(
    table(
      align: auto,
      columns: (auto),
      table.header("ADC abs"),
      "AbsOperand1",
      "AbsOperand2",
      "AbsOperand3",
      "Adc"
    )
    , caption: ""
  ),
  figure(
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
    , caption: ""
  ),
  figure(
    table(
      align: auto,
      columns: (auto),
      table.header("BVC rel"),
      "RelOperand",
      "Bvc",
      "RelBranch1",
      "RelBranch2"
    )
    , caption: ""
  ),
  figure(
    table(
      align: auto,
      columns: (auto),
      table.header("BCC rel"),
      "RelOperand",
      "Bcc",
      "RelBranch1",
      "RelBranch2"
    ) 
    , caption: ""
  ) 
) 

// TODO: replace manual reference with automatic one
In Tabelle 4 und 5 ist der gleiche Additionsbefehl mit zwei verschiedenen Addressierungsmodi zu sehen.
Hierbei können Mikrocodeschritte aus dem Fetchen des Operanden und der finalen Berechnung wiederverwendet werden.

In Tabelle 6 und 7 sind hingegen zwei unterschiedliche Verzweigungbefehle mit demselben Addressierungsmodus aufgelistet.
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
Falls der Emulator beim letzten Ausführungszyklus eines Befehls angekommen ist, kann dieser Zyklus nun bei regulärem Kontrollfluss in zwei verschiedene Zyklen übergehen, wie in Diagramm .../* TODO: figure citation here*/ erläutert wird.
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
=== Zustandsautomat

#figure(
  image("../resources/jade_state_algorithm.svg", width: 105%)
)
