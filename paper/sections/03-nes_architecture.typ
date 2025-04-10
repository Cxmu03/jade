#import "../util.typ": flex-caption

= NES-Architektur
== CPU <architecture_cpu>
Die zentrale Recheneinheit des NES ist eine Teilkomponente des Ricoh 2A03#footnote("In der NTSC Version. In der PAL Version der NES wird der Ricoh 2A07 Chip benutzt") Chips, auch RP2A03 genannt.
Hierbei handelt es sich um eine inoffizielle Nachbildung des MOS 6502 Mikroprozessors, weshalb der Prozessor des NES im Folgenden 6502 genannt wird #cite(<TEGMAN2005>).
Der MOS 6502 wurde 1975 von MOS Technology auf den Markt gebracht, um mit Prozessoren wie dem Motorolla 6800 oder dem Intel 8080 direkt zu konkurrieren. //Citation needed
Das Ziel von MOS Technology war, einen leistungsfähigen Mikroposzessor anzubieten, welcher günstig produziert und erworben werden konnte #cite(<Sachs2022>).
Mit dem 6502 wurde der Mikroprozessor-Markt revolutioniert und er fand schnell Anwendung in vielen Systemen, wir dem Apple I, dem Apple II, dem Commodore 64, dem Atari 2600 oder dem NES.

Beim 6502 handelt es sich um einen 8-Bit-Mikroprozessor mit einer Addressbusbreite von 16 Bit, was eine Addressierung von 64kiB erlaubt.
Der Befehlssatz beschränkt sich auf 56 verschiedene Befehle.
Des weiteren gibt es 13 verschiedene Addressierungsmodi, mit denen die Operanden für die Befehle spezifiziert werden können.
Kombiniert mit den 56 Befehlen ergeben sich 150 valide #link(<glossary_opcode>, "Opcodes") für den #link(<glossary_ins_set>, "Befehlssatz"), da nicht jeder Befehl mit jedem Addressierungsmodus verfügbar ist.
Verschiedene Variantes des 6502 können somit weitere inoffizielle Befehle unterstützen, da ein Maximum von 256 Opcodes möglich ist.

Bei der Architektur handelt es sich um eine klassische Von-Neumann-Architektur mit einem 8-Bit Datenbus für Programm und Daten.
Jedoch muss der Datenbus unterschieden werden in den öffentlichen Datenbus und dem internen S-Bus.
Der öffentliche Datenbus wird hauptsächlich für die Eingabe und Ausgabe von Daten mit Peripherie verwendet.
Im Gegensatz dazu wird der interne S-Bus dafür verwendet, die Daten in die Register zu und zwischen Registern zu übertragen.
Der S-Bus kann in @6502_block_diagram gesehen werden, hier wird er als "internal databus" bezeichnet.
Im Folgenden wird mit dem Begriff "Datenbus" jedoch stets der öffentliche Datenbus bezeichnet.

#figure(
  image(
    "../resources/6502_block_diagram.png", height: 50%
  ),
  caption: flex-caption([Blockdiagramm des 6502, aus @Data6502], [Blockdiagramm des 6502])
) <6502_block_diagram>

Der 6502 verfügt über 3 Hauptregister, welche vom Programmierer verwendet werden können, nämlich den Akkumulator sowie das X- und Y-Indexregister.
Der Akkumulator wird bei arithmetischen und logischen Operationen als impliziter Operand und für die Rückgabe von Werten verwendet.
Die Indexregister X und Y können benutzt werden, um Speicher-Offsets für bestimmte Addressierungsmodi zu speichern.
Darüber hinaus gibt es einen 8-Bit Stackpointer, einen 16-Bit Programcounter und eine 8-Bit Status-Flag für 7 verschiedenen Flaggen. 

=== Interrupts <6502_interrupts>
Der 6502 verfügt über vier verschiedene Möglichkeiten, einen Interrupt auszulösen. 
Dabei werden drei von diesen Interrupts über Hardwaremechanismen von externen Quellen ausgelöst.
Eine Besonderheit hierbei ist, dass auch diese Interrupts wie normale Befehle behandelt werden.
Das bedeutet, dass die Hardware einen zusätzlichen Break-Befehl in den aktuellen Kontrollfluss injiziert und dieser dann wie jeder andere Befehl ausgeführt wird /* TODO: citation needed */. 

Unterschieden wird hier hauptsächlich zwischen maskierbaren und nicht-maskierbaren Interrupts.
Die Ausführung eines maskierbaren Interrupts ist an die Bedingung geknüpft, dass die I-Flagge (Interrupt Disable) im Prozessorstatuswort auf 0 gesetzt ist. 
Ein nicht-maskierbarer Interrupt wird unabhängig davon immer ausgeführt.

Die einzige Möglichkeit einen maskierbaren Interrupt auszulösen ist der irq-Pin.
Dieser Pin ist Zustandsgesteuert und Low-Aktiv.
Ist der Pin also auf dem Low-Pegel, während die Interrupts gepolled werden, wird ein maskierbarer Interrupt eingereiht.

Nicht-maskierbare Interrupts können auf zwei Wege ausgeführt werden.
Eine Möglichkeit ist der nmi-Pin, welcher von externer Hardware angesteuert werden kann.
Dieser Pin ist flankengetriggert und löst auf einer negativen Flanke aus.
Diese Flanke kann zu jeder Zeit während eines Befehls passieren, was in einer Einreihung des Interrupts resultiert.
Mit der Ausführung des Interrupts wird gewartet, bis der aktuelle Befehl zuende geht.
Hierbei hat ein nicht-maskierbarer Interrupt immer Präzedenz zu einem maskierbaren Interrupt. 
Die zweite Möglichkeit einen nicht-maskierbaren Interrupt auszuführen ist der BRK-Befehl (0x00).

Die letzte Art von Interrupts ist der Reset-Interrupt, welcher im Gegensatz zu den anderen Interrupts unterschiedlich funktioniert.
Der Reset-Pin ist zwar auch flankengesteuert, jedoch im Gegensatz zum nmi-Pin auf einer positiven Flanke.
Dazu kommt, dass der Reset-Pin für eine normale Operation des Prozessors auf einem High-Pegel sein muss. 
Um nun einen Reset auszulösen, muss der Pin zuerst auf ein logisches Low-Level gezogen werden.
Solange dieser Low-Pegel anliegt, ist nur garantiert dass der Prozessor für diese Zeit Lesezyklen ausführt. 
Sobald dann der Pegel wieder auf High gezogen wird, kann ein Reset stattfinden, welcher grundsätzlich nur eine abgewandelte Break-Sequenz ist.
Dieser Mechanismus unterscheidet sich von den vorherigen jedoch insofern, dass ein Reset-Low-Pegel präemptiv ist, und den aktuelle ausführenden Befehl unterbricht.
Dabei kommt es jedoch darauf an, in welcher Phase sich der Befehl gerade befindet, da manche Befehlsschritte tatsächlich im Reset-Low-Modus korrekt ausgeführt werden. 

=== Clock
Der Takt des 6502 ist eine Zwei-Phasen-Takt, welcher aus den nicht-überlappenden Phasen $phi_1$ und $phi_2$ besteht.
Dieser Takt wird durch einen eingebauten Clock-Generator erzeugt, welcher über einen externen einphasigen Oszillator angesteuert werden kann.

#figure(
  image("../resources/6502_clocks.png", width: 100%),
  caption: 
    flex-caption(
      [Clocksignale des 6502, siehe #cite(<Data6502>)],
      [Clocksignale des 6502]
    )
) <6502_clocks>

In #ref(<6502_clocks>) sind zu sehen die Gatterzeit des Clock-Generators, bezeichnet mit $T_(01+)$ und $T_(02-)$, die Dauer $T_(L phi.alt_0)$ des Low-Pegels sowie die verkürzten High-Pegel $T_(P W H phi.alt 1)$ und $T_(P W H phi.alt 2)$ der beiden Phasen #cite(<Data6502>).

Ein besonderes Merkmal der 6502-Clocksignale sind die verkürten High-Pegel.
Diese stellen sicher, dass sich die Signale $phi.alt_1$ und $phi.alt_2$ zu keinem Zeitpunkt überschneiden. 
Dies ist ein wichtiger Mechanismus mit dem externe Komponenten mit der CPU synchronisiert können.
Sobald $phi.alt_1$ High ist, kann vom Datenbus gelesen werden, also wird eine Schreiboperation des 6502 durchgeführt. 
Ist jedoch $phi.alt_2$ High, dann kann in den Datenbus des 6502 geschrieben werden, was bedeutet das in diesem Moment eine Leseoperation geschieht #cite(<6502clocks>).

Die möglichen Frequenzen des externen Oszillators können sich je nach Modell und Anwendung unterscheiden.
In den originalen Varianten (MOS 6502 A, B, C und D), kann dieser mit einer Frequenz von 1MHz bis 4Mhz getaktet sein #cite(<SynCatalog>).

=== Pipeline <6502_pipeline>
Moderne CPUs verfügen oft über eine komplizierte, wie sie beispielsweise in #ref(<basics_architecture_pipeline>) vorgestellt wird.
Die Pipeline des 6502 ist jedoch deutlich simpler.
Der letzte Zyklus eines Befehls kann gleichzeitig mit dem Befehls-Fetch des nächsten Befehls ausgeführt werden.
Dies ist jedoch nicht möglich, falls der letzte Zyklus eines Befehls ein Schreibzyklus ist, da der Datenbus nicht zur selben Zeit für eine Lesevorgang und einen Schreibvorgang genutzt werden kann.
Diese vereinfachte Pipeline kann in #ref(<6502_pipeline_table>) gesehen werden.

#[
#set table.cell(align: horizon)
#let t_red = table.cell.with(fill: rgb("#536fff"))
#let t_green = table.cell.with(fill: rgb("#f44747"))
#let t_black = table.cell.with([], fill: black)

#figure(table(
  columns: (auto, auto, auto, auto, auto),
  table.header([*Zyklus*], [*r/w*], table.cell([*Befehle*], colspan: 2), [*Datenbus*]),
  "0", "r", t_green(rowspan: 3)[LDA imm], t_black(rowspan: 2), t_green[],
  "1", "r", t_green[],
  "2", "r", t_red(rowspan: 4)[STA abs], t_red[],
  "3", "r", t_black(rowspan: 3),  t_red[],
  "4", "r", t_red[], 
  "5", "w", t_red[],
  "6", "r", t_green(rowspan: 3)[LDX imm], t_black(rowspan: 3), t_green[],
  "7", "r", t_green[],
  "8", "r", ""
), caption: "Befehls-Pipeline des 6502") <6502_pipeline_table>
]

In dieser Tabelle kann verfolgt werden, welche Arten von Befehlen unter bestimmten Voraussetzungen miteinander überlappen können.
Die erste Spalte gibt an, in welchem Zyklus sich der Prozessor an einem Zeitpunkt befindet.
In der zweiten Spalte kann dann abgelesen werden, ob es sich um einen Lesezyklus (*r*) oder einen Schreibzyklus (*w*) handelt. 
Die tatsächliche Ausführung der Befehle ist in der dritten Spalte als eine Art Gantt-Diagramm aufgetragen.
In der letzten Spalte ist farblich kodiert, welcher Befehl in diesem Zyklus den Datenbus benötigt, unabhängig davon, ob dies ein Lese- oder ein Schreibzyklus ist.

Der Befehl *LDA imm* ("Load A Register with immediate") besteht nur aus Lesezyklen.
Die ersten beiden Zyklen, 0 und 1, werden dafür benötigt den Opcode und den 8-Bit Operanden zu aus dem Speicher zu lesen, was der Grund für die Beanspruchung des Datenbusses ist.
Im letzten Zyklus, 2, wird der Operand vom internen S-Datenbus in das A-Register geladen, weshalb der öffentliche Datenbus nicht mehr benötigt wird.
Aufgrund des nicht mehr benötigten öffentlichen Datenbusses kann der Befehls-Fetch des nächsten Befehls während des letzten Zyklus bereits ausgeführt werden.
Deshalb wird die Zahl der benötigten Zyklen in Befehlssatz-Referenzen nur als 2 angegeben, da der dritte Zyklus sozusagen maskiert wird #cite(<6502org>) #cite(<Masswerk>).

Der Befehl *STA abs* (Store A Register to absolute address) schreibt den Wert des A Registers in den Speicher.
Das Schreiben des Akkumulators in den Speicher geschieht im letzten Zyklus des Befehls, nachdem der Opcode und die zwei 8-Bit Operanden in den vorherigen drei Zyklen eingelesen wurden.
Da hier der öffentliche Datenbus benutzt wird, kann der Fetch des nächsten Befehls nicht gleichzeitig durchgeführt werden. 

=== Verspätetes Register-Update
Eine Besonderheit des 6502 offenbart sich beim Ausführen von Befehlen, welche im letzten Zyklus eine Berechnung mit der ALU durchführen und das Ergebnis dieser Berechnung in einem Register speichern.
Dies passiert beispielsweise bei Inkrementierungsbefehlen (INX, INY, DEX, DEY), arithmetischen Operationen (ADC, SBC), logischen Operationen (AND, EOR, ORA) und arithmetische Shift- oder Rotate-Operationen (SHL, SHR, ROR, ROL).
Dieses Verhalten kann aus dem Ausschnitt des modifizierten Visual6502-Logs in #ref(<visual_6502_inx_log>) abgelesen werden, welches durch das Programm aus #ref(<simple_inx_listing>) zustandekommt.

#figure(```asm
LDA #09
INX
DEY
```, caption: [Source Code zur Erzeugung von #ref(<visual_6502_inx_log>)]) <simple_inx_listing>

#figure(table(
  align: auto,
  columns: (auto, auto, auto, auto, auto, auto, auto, auto, auto, auto, auto, auto),
  fill: (x, y) => {
    if y == 0 {
      return rgb("bbccff");
    }
    let colors = ("cfdaff", "e3e9ff", "e3e9ff", "ffffff");
    rgb(colors.at(calc.rem(y, 2) * 2 + calc.rem(x, 2)))
  },
  table.header("cycle", [$phi_h$], "db", "sb", "x", "alucin", "alua", "alub", "alu", "Fetch", "Execute", "DPControl"),
  "5", [$phi_2$], "e6", [*ff*], "0a", "0", "0a", "0a", [*14*], "", "DEY", "",
  "5", [$phi_1$], [*e6*], "0a", [*0a*], [*0*], [*0a*], [*0a*], "0a", "", "DEY", "SBX",
  "4", [$phi_2$], "88", [*0a*], "09", "1", "09", "00", [*0a*], "DEY", "INX", table.cell("SUMS, ADDSB7, ADDSB06", align: left),
  "4", [$phi_1$], "88", [*09*], "09", [*1*], [*09*], [*00*], "24", "DEY", "INX", "XSB, SBADD",
  "3", [$phi_2$], "88", [*ff*], "09", "0", "12", "12", [*24*], "", "INX", "",
  "3", [$phi_1$], [*88*], "12", "09", "0", [*12*], [*12*], "12", "", "INX", "",
  "2", [$phi_2$], "e8", [*12*], "09", "0", "09", "09", [*12*], "INX", "LDX #", "",
  "2", [$phi_1$], "e8", "00", "09", "0", "09", "09", "fe", "INX", "LDX #", "",
), caption: [Registerzustände des 6502 mit Programm aus #ref(<simple_inx_listing>)#footnote([Generiert mit #link("http://www.visual6502.org/JSSim/expert.html?graphics=f&loglevel=-1&logmore=cycle,db,sb,x,alucin,alua,alub,alu,Fetch,Execute,DPControl&steps=12&a=0000&d=a209e888")])]) <visual_6502_inx_log>

In den ersten beiden Spalten ist zu sehen, in welchem Taktzyklus sich die CPU aktuell befindet.
Durch die Aufteilung der Systemclock in die Clocksignale $phi_1$ und $phi_2$ besteht hier jeder Zyklus aus zwei Subzyklen.
Die Spalten Drei bis Neun zeigen den Zustand verschiedener interner Register zum Ende eines bestimmten Zylkus.
Hier zu sehen sind der öffentliche Datenbus (db), der private Spezialbus (sb), das Indexregister X (x), der Übertrag der ALU (alucin), der erste Eingang der ALU (alua), der zweite Eingang der ALU (alub) und das Ergebnis der ALU (alu).
Die beiden vorletzten Zeilen beschreiben, welcher Befehl gerade in Ausführung ist (Execute) und ob gerade ein Befehl gefetched wird (Fetch).
In der letzten Zeile (DPControl) werden für bestimmte Zyklen ausgewählte Kontrollsignale dargestellt, um den Datenfluss verständlicher zu machen. 

In Zyklus Zwei der Ausführung wird das Indexregister X durch den Befehl *LDA imm* mit dem Wert $09_16$ geladen.
Der darauffolgende Befehl *INX impl* soll dieses Register nun inkrementieren.
Hierfür wird in $phi_1$ von Zyklus Vier der Wert des X-Registers mithilfe des Kontrollsignals *XSB* auf den Spezialbus geladen und von dort aus durch das Signal *SBADD* in den ALU-Eingang transferiert.
Außerdem wird das Übertragsbit auf 1 gesetzt, um den X-Wert zu inkrementieren. 
In $phi_2$ von Zyklus Vier erfolgt dann die Addition $"alucin" + "alua" + "alub" equiv 01_16 + 09_16 + 00_16 equiv "0A"_16 " " (mod 256)$ durch das Kontrollsignal *SUMS*, welches die Summenfunktion der ALU triggert.
Im selben Takt wird das Ergebnis der ALU auf den Spezialbus übertragen, durch die Kontrollsignale *ADDSB7* und *ADDSB06*
Das Laden eines Werts in ein Register geschieht im 6502 jedoch immer nur während $phi_1$ eines Takts #footnote("TODO: citation needed").  
Deshalb passiert dies in $phi_1$ von Takt Fünf durch das Kontrollsignal *SBX*, wobei der nächste Befehl in diesem Takt bereits ausgeführt wird.

== PPU <architecture_ppu>
== Speicher <architecture_memory>
=== CPU RAM <architecture_memory_cpu_ram>
== APU <architecture_apu>
== Gesamtsystem
=== NES vs PAL
Aus Gründen der Lokalisierung gibt es von dem NES zwei verschiedene Versionen, nämlich die NTSC- und die PAL-Version.
NTSC (National Television System Committee) und PAL (Phase Alternating Line) sind Standards, welche Video- und Farbformate für das analoge Fernsehen spezifizieren.
In Nordarmerika, kleinen Teilen von Südamerika und wenigen Ländern in Ostasien wurde NTSC benutzt #cite(<SonyNesPal>), welches 1941 vom National Television System Committee entwickelt wurde.
In großen Teilen von Südamerika, Europa, Afrika, Südostasien und Australien wurde hingegen PAL verwendet #cite(<SonyNesPal>), welches 1961 von der Firma Telefunken in Deutschland entwickelt wurde.
Aufgrund dieser geografischen Aufteilung wurden verschiedene NES-Versionen verkauft, wobei die erste NES-Konsole (Famicom) ein NTSC-System war.

=== Clock <nes_architecture_clock>
Das NES in der NTSC-Version wird mit einer Haupttakt von $f_("main")=21.477272"Mhz"$ angesteuert, wobei hier eine Varianz von $plus.minus 40"Hz"$ toleriert werden kann.
Der Takt für die Teilkomponenten ergibt sich über die Teilung des Taktes mittels mehreren Frequenzteilern.
Der Frequenzteiler für die CPU operiert mit einem Teilungsverhältnis von $1/12 dot f_("main")$, woraus sich eine Taktfrequenz von $f_("cpu")=1.789773"Mhz"$.
Für die PPU wird ein Verhältnis von $1/4 dot f_("main")$ verwendet.
Daraus ergibt sich ein Takt von $f_("ppu")=5.369318"Mhz"$, was drei PPU-Zyklen, auch Dot genannt, pro CPU-Zyklus entspricht. #cite(<CycleTimes>)
