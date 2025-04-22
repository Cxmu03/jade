#import "../util.typ": validation-results, flex-caption

= Ergebnisse
In diesem Kapitel werden die Ergebnisse der Validierung und der Benchmarks, welche in @verification_validation eingeführt wurden, vorgestellt und diskutiert.
Dabei werden Laufzeiten für Programme und Benchmarks angegeben, welche abhängig von dem Computer sind, auf dem sie ausgeführt werden.
Diese sind deshalb primär im Verhältnis zu betrachten.
Alle Programme wurden mit einem Intel i5-10500h auf einem einzelnen Kern mit ca. 4.2GHz ausgeführt.
Bei der Ausführungsumgebung handelt es sich um Ubuntu 22.04, welches unter WSL 2.4.13.0 lief.
Für alle Benchmarks und Validierungsoperationen wurde ein Release-Build benutzt.

== Validierung <results_validation>
Dieses Kapitel stellt die Ergebnisse der Validierung des entwickelten Emulators mit den verschiedenen Programmen aus der `jade-programs` Crate vor.
Die validierten Programme sind das Standardprogramm des Visual6502, die funktionale Testsuite von Klaus Dormann und der MD5-Hashalgorithmus, welche einzeln behandelt werden.

=== Schweregrad von Validierungsfehlern
In jedem Validierungsdurchlauf wird nach dessen Beendigung ausgegeben, wie viele Fehler zwischen dem Generator und dem Validator erkannt wurden.
Die Kategorisierung dieser Fehler ist die Gleiche, wie sie in @cycle_validation beschrieben wird.
Für die Bewertung eines Durchlaufs werden die Fehlertypen zusätzlich nach ihrem Schweregrad klassifiziert.

Der Kontrollflussfehler ist dabei die schlimmste Fehlerart, da solch ein Fehler in den meisten Fällen weitere Fehler nach sich zieht, da meist ein völlig anderer Ausführungspfad gewählt wird.
Registerfehler und IO-Fehler können zwar auch zu weiteren Fehlern führen, wenn die Ergebnisse von diesen Operationen für Kontrollfluss verwendet werden, jedoch sind diese Fehler in der Regel etwas isolierter als Kontrollflussfehler.
Dennoch sind diese Fehler sehr unerwünscht, da sie oft zu einem falschen Ergebnis führen.
Isolierte Statusfehler sind in der Regel die wenigst am wenigsten schwerwiedenden Fehler, diese haben jedoch das Potenzial einen Kontrollflussfehler auszulösen, falls das falsche Statusbit für eine Verzweigung abgefragt wird.

=== Visual6502 Standardprogramm
Der erste Validierungsdurchlauf fand mit dem Standardprogramm des Visual6502 statt, welches in @visual6502_default_program gezeigt wird.
Dieses wurde als erster Validierungsanlauf genutzt, da das Programm recht simpel ist und die Infrastruktur damit einfach getestet werden kann.

#validation-results(
  ```
  Validated Jade with Perfect6502:
  Ran program Visual6502 Default for 1000000 cycles
  Status errors: 0 ~ 0%
  IO errors: 0 ~ 0%
  Register errors: 0 ~ 0%
  Control Flow errors: 0 ~ 0%
  ```,
  [Validierungsergebnisse der Visual6502 Standardprogramms],
  <validation_results_visual6502_default_program>,
  placement: none
)

Die Ergebnisse der Validierung sind in @validation_results_visual6502_default_program zu sehen.
Wie zu erwarten läuft der Emulator mit diesem Programm deckungsgleich zu dem Perfect6502 ohne einen Fehler zu produzieren.

=== Dormann <dormann_validation_results>
Die Testsuite von Klaus Dormann (siehe @sec:dormann), wurde mit beiden vorgestellten Validierungsverfahren, welche in @verification_validation vorgestellt wurden, vollständig validiert.
Hierfür wurde das funktionale Testprogramm ohne Dezimalmodus und mit Speicherüberprüfung kompiliert, welches nach 84030454 Zyklen terminiert.

Die Testrom-Validierung war hierbei das deutlich einfachere Verfahren, da diese nur mit einem Emulator durchgeführt wird und es somit keinen Validator gibt, der die Ausführung verlangsamt.
Ein kompletter Durchlauf dieses Programms mit dem Emulator `Jade` terminiert in etwa 2.7 Sekunden.

Die Zyklenvalidierung mit dem Perfect6502 ist deutlich aufwändiger als die reine Testrom-Validierung.
Der Grund dafür liegt in der Geschwindigkeit des Perfect6502.
Eine vollständige Zyklenvalidierung der 84030454 Zyklen dauerte 2 Stunden und 54 Minuten.
Die Ausgabe dieses Vorgangs kann in @dormann_validation gesehen werden.
Durch die vorherigen Zyklen, welche durch Erkennung der Trap als Traces ausgegeben werden, kann manuell verifiziert werden dass es sich tatsächlich um das Ende der Testsuite handelt, was bedeutet dass alle Tests erfolgreich durchgelaufen sind.
Am Ende der Ausgabe können die gefundenen Fehler abgelesen werden.

#validation-results(
    ```
    Matched exit condition trap detected after 84030454 cycles 
    cycle: 84030454, a: f0 x: 0e, y: ff, ab: 336d, db: 4c, r: ReadCycle, pc: 336d, sp: ff, Some("JMP abs"), Some(JmpAbs), FetchExecute, p: NV-BdizC, res: true

    Validated Jade with Perfect6502:
    Ran program Dormann for 84030454 cycles
    Status errors: 1147136 ~ 1.2%
    IO errors: 0 ~ 0%
    Register errors: 0 ~ 0%
    Control Flow errors: 0 ~ 0%
    ```,
    [Validierungsergebnisse der Dormann Testsuite],
    <dormann_validation>,
    placement: none
) 

Die einzige Fehlerquelle in dieser Testsuite sind Fehler in der Statusflagge des Generators, welche in $1.2%$ der Zyklen nicht mit dem Validator übereinstimmt.
Da es jedoch zu keinen Fehlern im Kontrollfluss kommt, liegt die Vermutung nahe, dass keine Statusflaggen, welche für Verzweigungen benutzt werden, falsch gesetzt wurden.
Dies würde die Break-Flagge, Interrupt-Disable-Flagge, Dezimal-Flagge und das unbenutzte Statusbit übrig lassen.
Da jedoch Interrupts und der Dezimalmodus in diesem Test nicht benutzt werden, können diese ausgeschlossen werden.
Durch eine manuelle Analyse konnte herausgefunden werden, dass es sich hierbei um ein Verhalten des dritthöchsten Statusbits handelt, welches eigentlich unbenutzt ist #cite(<StatusFlags>).

Der Fehler geschieht im dritten Zyklus des *RTS* (Return from Subroutine) Befehls, in dem dieses Statusbit entweder auf 0 oder 1 gesetzt wird. 
Ein Muster von diesem Verhalten konnte nicht erkannt werden, weshalb dieser Fehler bestehen bleibt.
Dies ist jedoch für die Korrektheit der Emulation nicht essenziell, da diese Flagge für kein funktionales Verhalten des Prozessors genutzt wird. 

=== MD5 <md5_validation_results>
Das Hashen eines Wertes mit dem MD5 Algorithmus wurde ebenfalls mithilfe beider Validierungsmethoden validiert.
Hier besteht das Problem der Laufzeit weiter, da ein vollständiger Durchlauf des MD5-Algorithmus mit der vorhandenen Implementierung 9058257 Zyklen benötigt.

Das MD5-Programm aus der `jade-programs` Crate kann als Testrom benutzt werden, da nach dem Vergleichen des generierten Hashes mit einem Sollwert, der Wert des Akkumulators auf einen bestimmten Wert gesetzt wird, welcher einen Erfolg oder ein Fehlschlagen des Algorithmus signalisiert.
Nach dem Durchlauf des Programms durch den Emulator `Jade` kann im Akkumulator der Wert $"BE"_16$ abgelesen werden, was bedeutet dass der richtige Hashwert errechnet wird.
Da, wie in @jade_programs_md5 erwähnt wird, durch den Algorithmus ein großer Lawineneffekt besteht, kann mit großer Sicherheit gesagt werden, dass die Emulation von MD5 somit vollständig korrekt ist.

Um diesen Algorithmus noch auf Zyklenebene zu validieren, wurde ein Validierungsdurchlauf mit dem perfect6502 durchgeführt.
Das Ergebnis hiervon ist in @validation_md5 zu sehen.
Diese Ergebnisse bestätigen die Testrom-Validierung, da es außer dem Prozessorstatus keine fehlerhaften Zyklen gab.
Bei diesen Statusfehlern, welche in nur 0.7% der Zyklen erscheinen, handelt es sich um die selben Fehler, welche bereits in @dormann_validation_results beleuchtet wurden.

#validation-results(
  ```
  Matched exit condition pc == 0x031e after 9058256 cycles
  cycle: 9058256, a: be x: 00, y: 00, ab: 031e, db: 00, r: ReadCycle, pc: 031e, sp: bb, Some("BRK impl"), Some(JmpAbs), FetchExecute, p: Nv-BdIzc, res: true

  Validated Jade with Perfect6502:
  Ran program MD5 for 9058256 cycles
  Status errors: 71895 ~ 0.7%
  IO errors: 0 ~ 0%
  Register errors: 0 ~ 0%
  Control Flow errors: 0 ~ 0%
  ```,
  [Validierungsergebnisse von MD5],
  <validation_md5>,
  placement: none
)

== Benchmarks
In diesem Kapitel werden die Ergebnisse der Benchmarks für alle drei Emulatoren vorgestellt, analysiert und am Ende verglichen.
Wie bereits gesagt, ist es besonders bei den Benchmarks wichtig, dass die Ergebnisse nur im Verhältnis betrachtet werden, da die Ausführungsgeschwindigkeit, und somit auch die Echtzeitfähigkeit, stark abhängig vom Computer sind, auf dem die Emulatoren ausgeführt werden.

Die gezeigten Tabellen mit verschiedenen statistischen Merkmalen und die gezeigten Graphen sind, falls nicht anders erwähnt, eigens aus den Sample-Daten von `Criterion` erstellt worden.

=== Jade
Die Ergebnisse des Benchmarks für den in dieser Arbeit entwickelten Emulator `Jade` sind in @jade_benchmark_results_graph dargestellt.
In diesem Diagramm werden alle ausgeführten Programme aufgetragen.
Die x-Achse beschreibt, wie viele Zyklen in einem Durchlauf eines Benchmarks durchgeführt werden.
Auf der y-Achse kann dann das arithmetische Mittel der Durchlaufzeiten aller Samples eines Durchlaufs abgelesen werden.

#figure(
  image(
    "../resources/jade_programs_cycles.png", width: 90%
  ),
  caption: [Programmlaufzeiten über verschiedene Zyklenanzahlen (Jade)]
) <jade_benchmark_results_graph>

In diesem Graphen, sowie den einzelnen Messwerten, kann abgelesen werden, dass es sich hierbei um eine lineare Beziehung handelt.
Dies bedeutet, dass eine steigende Zyklenanzahl keine Auswirkung auf die Ausführungsgeschwindigkeit pro Zyklus hat.
Einzig bei sehr kleinen Benchmarks mit weniger als 100 Zyklen verschlechtert sich die Zeit pro Zyklus durch fehlendes Aufwärmen der Cpu, da Aspekte wie Sprungvorhersage und Caching noch nicht vollständig ausgenutzt werden können.

Des weiteren kann hier gesehen werden, dass der MD5-Algorithmus in diesem Emulator schneller läuft, als die anderen beiden Programme, welche das gleiche Laufzeitverhalten haben.
Im letzten Datenpunkt, bei 1000000 Zyklen, ist MD5 hier etwa 1.58 mal schneller als Dormann.
Der Grund für diesen Effekt ist nicht abschließend geklärt.

Die genauen Ergebnisse dieses Benchmarks werden in @jade_md5_1e6_results dargestellt.
Hier werden die Ergebnisse aus @jade_benchmark_results_graph erneut bestätigt.
Für ein bestimmtes Programm ist die erreichte Taktfrequenz stabil, wobei diese bei MD5 deutlich höher als bei den anderen Programmen ist.
Ein interessanter Effekt ist jedoch, dass die Standardabweichung mit steigender Zyklenanzahl abnimmt.
Die Stabilität der Emulationsperformanz nimmt also mit steigender Ausführungslänge zu.
Dies kann bei allen drei Programmen beobachtet werden.

#figure(
  table(
    columns: (auto,)*7,
    [], table.cell(colspan: 2)[MD5], table.cell(colspan: 2)[Dormann], table.cell(colspan: 2)[Default],
    [Zyklen], [$mu$ (Mhz)], [$sigma$ (Mhz)], [$mu$ (Mhz)], [$sigma$ (Mhz)], [$mu$ (Mhz)], [$sigma$ (Mhz)],
    [$1 dot 10^2$], [69.09], [6.79], [43.46], [2.29], [42.38], [2.13],
    [$1 dot 10^3$], [69.65], [3.13], [43.56], [2.69], [42.89], [1.68],
    [$1 dot 10^4$], [69.78], [2.24], [43.99], [1.73], [42.65], [1.91],
    [$1 dot 10^5$], [69.26], [2.12], [44.19], [0.88], [42.57], [1.36],
    [$1 dot 10^6$], [68.32], [1.99], [43.09], [0.94], [42.97], [0.86],

  ),
  caption: flex-caption([Benchmarkergebnisse des `Jade`-Emulators. Dargestellt sind arithmetisches Mittel der Samples ($mu$), sowie die Standardabweichung ($sigma$), jeweils in MHz], [Benchmarkergebnisse des `Jade`-Emulators])
) <jade_md5_1e6_results>

Des weiteren wurde die Performanz des Emulators mit `perf` getestet, welches wichtige Metriken über den erreichten Cpu-Durchsatz oder die Anzahl der verfehlten Zweige durch den Branch Predictor ausgibt.

#validation-results(
  ```
 Performance counter stats for './jade-validate run --emulator jade --cycles 1000000 with-builtin md5':

             55.78 msec task-clock:u       # 0.619 CPUs utilized
         107751134      cycles:u           # 1.932 GHz
         293598441      instructions:u     # 2.72  insn per cycle
          48557275      branches:u         # 870.566 M/sec
            446617      branch-misses:u    # 0.92% of all branches 
  ```,
  [Ergebnisse von `perf` für `Jade`, MD5, $1 dot 10^6$ Zyklen],
  <jade_perf_results>,
  placement: none
)

Hier kann gesehen werden, dass bei einem Durchsatz von über 870 Millionen Verzweigungen pro Sekunde nur $0.93%$ dieser Zweige falsch vorhergesagt werden, was einen großen positiven Einfluss auf die Performanz hat.

=== Perfect6502
Die Durchlaufzeiten der Benchmark-Samples für den `Perfect6502` sind in @perfect6502_benchmark_graph abgebildet.
In diesem Graphen wird erneut ein linearer Zusammenhang zwischen der Anzahl an ausgeführten Zyklen und der Ausführungsdauer ersichtlich.
Im Unterschied zu `Jade` gibt es hier kein Programm, welches gegenüber den anderen Programmen deutlich schneller ist.
Die Dormann-Testsuite hat zwar den größten Anstieg in der Durchlaufzeit, jedoch ist der Unterschied zu den anderen Programmen bei , jedoch ist der Unterschied zu den anderen Programmen bei $1 dot 10^6$ Zyklen nicht so groß wie in @jade_benchmark_results_graph.
 
#figure(
  image(
    "../resources/perfect6502_programs_cycles.png", width: 90%
  ),
  caption: [Programmlaufzeiten über verschiedene Zyklenanzahl (Perfect6502)]
) <perfect6502_benchmark_graph>

In @perfect_md5_1e6_results wird dies bestätigt.
Erneut ist die erreichte Taktfrequenz innerhalb eines Programms sehr stabil.
Auch die Unterschiede in der Ausführungszeit zwischen den Programmen sind überaus klein.
Ein Effekt der hier jedoch nicht zu erkennen ist, ist die Korrelation zwischen der Anzahl der ausgeführten Zyklen und der Standardabweichung der erreichten Taktfrequenz.
Dies könnte mit den Ergebnissen von `perf`, welche in @perfect6502_perf_results zu sehen sind, zusammenhängen.
Die Quote von falsch gewählten Verzweigungen ist hier mit $11.77%$ sehr hoch.
Dies könnte mit der komplexen Simulation der Hardware in Verbindung stehen und es somit dem Branch Predictor erschweren, eine stabilere Performanz im Laufe des Programms zu gewährleisten.

#figure(
  table(
    columns: (auto,)*7,
    [], table.cell(colspan: 2)[MD5], table.cell(colspan: 2)[Dormann], table.cell(colspan: 2)[Default],
    [Zyklen], [$mu$ (kHz)], [$sigma$ (kHz)], [$mu$ (kHz)], [$sigma$ (kHz)], [$mu$ (kHz)], [$sigma$ (kHz)],
    [$1 dot 10^2$], [9.03], [0.04], [9.42], [0.43], [8.00], [0.11],
    [$1 dot 10^3$], [8.99], [0.01], [8.54], [0.12], [8.75], [0.08],
    [$1 dot 10^4$], [9.03], [0.03], [8.64], [0.06], [8.73], [0.11],
    [$1 dot 10^5$], [9.03], [0.01], [8.61], [0.05], [8.75], [0.06],
    [$1 dot 10^6$], [9.11], [0.19], [8.51], [0.29], [9.10], [0.32],

  ),
  caption: flex-caption([Benchmarkergebnisse des `Perfect6502`-Emulators. Dargestellt sind arithmetisches Mittel der Samples ($mu$), sowie die Standardabweichung ($sigma$), jeweils in kHz], [Benchmarkergebnisse des `Perfect6502`-Emulators])
) <perfect_md5_1e6_results>

#validation-results(
  ```
 Performance counter stats for './jade-validate run --emulator perfect --cycles 1000000 with-builtin md5':

         112597.19 msec task-clock:u       #    1.000 CPUs utilized
      456730388176      cycles:u           #    4.056 GHz
      484691320655      instructions:u     #    1.06  insn per cycle
       66704788618      branches:u         #  592.420 M/sec
        7854310440      branch-misses:u    #   11.77% of all branches
  ```,
  [Ergebnisse von `perf` für den `Perfect6502`, MD5, $1 dot 10^6$ Zyklen],
  <perfect6502_perf_results>,
  placement: none
)

=== emulator_6502
Die Graphen, welche sich für die Durchlaufzeit des `emulator_6502` in @emulator6502_benchmark_graph ergeben, weisen eine gewisse Ähnlichkeit zu @jade_benchmark_results_graph auf.
Das Verhältnis zwischen den Ausführungszeiten und der Anzahl der Zyklen ist hier ebenfalls wieder linear.
Außerdem ist MD5 erneut ein Ausreißer und wird sichtbar schneller mit steigender Zyklenzahl ausgeführt, wenn auch nicht so extrem wie in @jade_benchmark_results_graph.
Bei einer ausgeführten Anzahl von $1 dot 10^6$ Zyklen ist MD5 hier etwa $1.12$ mal schneller als die Dormann-Testsuite.

Die detaillierten Ergebnisse dieses Benchmarks sind @emulator6502_md5_1e6_results zu entnehmen.
Innerhalb eines Programms ist die Ausführungszeit pro Zyklus relativ stabil, wobei MD5 hier eine kleine Ausnahme datstellt.
Mit einer steigenden Anzahl an Zyklen nimmt die Taktfrequenz leicht ab, dies ist jedoch relativ gesehen eine kleine Abweichung.
In der Standardabweichung ist hier kein Muster zu erkennen.
Während diese bei MD5 für steigende Zyklenzahlen schlagartig zunimmt, geschieht das Gegenteil in der Dormann-Testsuite.
Hierbei könnte es sich rein um Rauschen in der Zeiterfassung handeln.

#figure(
  image(
    "../resources/emulator6502_programs_cycles.png", width: 90%
  ),
  caption: [Programmlaufzeiten über verschiedene Zyklenanzahl (emulator_6502)]
) <emulator6502_benchmark_graph>
#figure(
  table(
    columns: (auto,)*7,
    [], table.cell(colspan: 2)[MD5], table.cell(colspan: 2)[Dormann], table.cell(colspan: 2)[Default],
    [Zyklen], [$mu$ (Mhz)], [$sigma$ (Mhz)], [$mu$ (Mhz)], [$sigma$ (Mhz)], [$mu$ (Mhz)], [$sigma$ (Mhz)],
    [$1 dot 10^2$], [161.56], [5.45], [127.67], [12.92], [129.44], [4.17],
    [$1 dot 10^3$], [160.96], [6.44], [129.46], [11.72], [128.32], [4.48],
    [$1 dot 10^4$], [153.40], [16.11], [131.82], [4.86], [126.67], [7.46],
    [$1 dot 10^5$], [149.28], [16.62], [130.45], [6.59], [127.14], [3.19],
    [$1 dot 10^6$], [144.50], [13.34], [129.54], [7.74], [126.84], [3.40],

  ),
  caption: flex-caption([Benchmarkergebnisse des `emulator_6502`. Dargestellt sind arithmetisches Mittel der Samples ($mu$), sowie die Standardabweichung ($sigma$), jeweils in MHz], [Benchmarkergebnisse des `emulator_6502`])
) <emulator6502_md5_1e6_results>

Für diesen Emulator liegt leider kein Ergebnis aus `perf` vor, da zum Zeitpunkt der Benchmarks eine `Generator`-Implementierung (siehe @jade_validate_traits) gefehlt hat.
Somit konnten keine gleichen Voraussetzungen im Aufruf des Programms gewährleistet werden, da das `run`-Kommando des CLI solch eine Implementierung voraussetzt.

== Zusammenfassung und Diskussion
Um aus diesen einzelnen Resultate der Emulatoren einen Schluss ziehen zu können, müssen diese miteinander verglichen werden.
Dafür werden die Ergebnisse in @comparison_emulators_bar zusammengefasst, welche die durchschnittliche Durchlaufzeit jedes Emulators mit allen Programmen über $1 dot 10^6$ Zyklen darstellt.
Da der Wertebereich dieser Zeiten über mehrere Größenordnungen reicht, wird die y-Achse logarithmisch aufgetragen.

Besonders auffällig sind die hohen Laufzeiten des `Perfect6502` im Vergleich zu den anderen Emulatoren.
Diese liegen in der Größenordnung $10^9 mu s$ für $1 dot 10 ^6$ Zyklen, was etwa $9.1$kHz entspricht, wie der @perfect_md5_1e6_results entnommen werden kann.
Damit ist der `Perfect6502` auf dem Testsystem nicht echtzeitfähig, da somit nur ca. 0.54% der geforderten $1.7$MHz erreicht werden.
Dabei handelt es sich um das Trade-Off, welches für eine exakte Genauigkeit der Simulation eingegangen wird.

#figure(
  image(
    "../resources/emulators_comparison_all_programs_1e6.png"
  ),
  caption: [Vergleich der Programmlaufzeit zwischen den Emulatoren]
) <comparison_emulators_bar>

Der `emulator_6502` ist erwartungsgemäß der schnellste der getesteten Emulatoren.
Bei der Ausführung des MD5-Programms erreicht dieser bis zu $144.50$MHz, was den Echtzeitanforderungen mehr als genügt.
Hier wird ebenfalls ein großes Trade-Off mit der Genauigkeit eingegangen, indem diese geopfert wird.
Da die kleinstmögliche atomare Einheit in diesem Emulator ein Befehl ist, werden die einzelnen Zyklen nicht modelliert. 
Somit können bestimmte Hardwareinteraktionen auch nicht genau nachgebildet werden. 

Mit `Jade` wird versucht, möglichst wenig Genauigkeit einzubüßen und trotzdem eine hohe Performanz gewährleisten zu können, so dass dieser Emulator auch auf leistungsschwächeren Systemen echtzeitfähig bleibt.
Die Performanz ist zwar geringer als bei rein befehlsgenauen Emulatoren wie dem `emulator_6502`, jedoch übertrifft `Jade` mit einer erreichten Taktfrequenz von $68.32$MHz (siehe @jade_md5_1e6_results) im Durchlauf mit MD5 die von dem NES geforderten $1.7$MHz bei weitem.
Im direkten Vergleich zum `emulator_6502` liegt der Unterschied in der Geschwindigkeit mit einem Faktor von $(144.5"MHz")/(68.32"MHz")=2.11$ noch innerhalb einer Größenordnung.

Die Validierungsergebnisse für Jade sind äußerst positiv ausgefallen.
Sowohl für die Dormann-Testsuite als auch MD5 kann eine vollständige Validierung durchgeführt werden.
Die einzige gefundene Fehlerquelle ist das unbenutzte Statusbit, welches im dritten Zyklus von *RTS \#* kein erkennbares Muster aufweist.
Das funktionale Ergebnis wurde jedoch in beiden Fällen ohne weitere Fehler erreicht.

Die hier vorgestellte Validierungsmethode für einzelne Zyklen hebt diesen Emulator von anderen Emulatoren, wie beispielsweise den vorgestellten Emulatoren in @related_works, hervor, auch wenn diese unter Anderem mehr Features oder einen feinere Granularität bieten.