#import "../util.typ": validation-results

= Ergebnisse und Diskussion
In diesem Kapitel werden die Ergebnisse der Validierung und der Benchmarks, welche in @verification_validation eingeführt wurden, vorgestellt und diskutiert.
Dabei werden Laufzeiten für Programme und Benchmarks angegeben, welche abhängig von dem Computer sind, auf dem sie ausgeführt werden.
Diese sind deshalb primär im Verhältnis zu betrachten.
Alle Programme wurden mit einem Intel i5-10500h auf einem einzelnen Kern mit ca. 4.2GHz ausgeführt.
Bei der Ausführungsumgebung handelt es sich um Ubuntu 22.04, welches unter WSL 2.4.13.0 lief.
Für alle Benchmarks und Validierungsoperationen wurde ein Release-Build benutzt.

== Validierung
Dieses Kapitel stellt die Ergebnisse der Validierung des entwickelten Emulators mit den verschiedenen Programmen aus der `jade-programs` Crate vor.
Die validierten Programme sind das Standardprogramm des Visual6502, die funktionale Testsuite von Klaus Dormann und der MD5-Hashalgorithmus, welche einzeln behandelt werden.

In jedem Validierungsdurchlauf wird nach dessen Beendigung ausgegeben, wie viele Fehler zwischen dem Generator und dem Validator erkannt wurden.
Die Kategorisierung dieser Fehler ist die Gleiche, wie sie in @cycle_validation beschrieben wird.

=== Visual6502 Standardprogramm
Der erste Validierungsdurchlauf fand mit dem Standardprogramm des Visual6502 statt, welches in @visual6502_default_program gezeigt wird.
Dieses wurde als erster Validierungsanlauf genutzt, da das Programm recht simpel ist und die Infrastruktur damit einfach getestet werden kann.

Die Ergebnisse der Validierung sind in @validation_results_visual6502_default_program zu sehen.
Wie zu erwarten läuft der Emulator mit diesem Programm deckungsgleich zu dem Perfect6502 ohne einen Fehler zu produzieren.

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

=== Dormann <dormann_validation_results>
Die Testsuite von Klaus Dormann (siehe @sec:dormann), wurde mit beiden vorgestellten Validierungsverfahren, welche in @verification_validation vorgestellt wurden, vollständig validiert.
Hierfür wurde das funktionale Testprogramm ohne Dezimalmodus und mit Speicherüberprüfung kompiliert, welches nach 84030454 Zyklen terminiert.

Die Testrom-Validierung war hierbei das deutlich einfachere Verfahren, da diese nur mit einem Emulator durchgeführt wird und es somit keinen Validator gibt, der die Ausführung verlangsamt.
Ein kompletter Durchlauf dieses Programms mit dem Emulator `Jade` terminiert in etwa .../*TODO*/ Sekunden.

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
/*TODO: Krieg ich hier traces die das zeigen?*/


=== MD5 <md5_validation_results>
Das Hashen eines Wertes mit dem MD5 Algorithmus wurde ebenfalls mithilfe beider Validierungsmethoden validiert.
Hier besteht das Problem der Laufzeit weiter, da ein vollständiger Durchlauf des MD5-Algorithmus mit der vorhandenen Implementierung 9058257 Zyklen benötigt.

Das MD5-Programm aus der `jade-programs` Crate kann als Testrom benutzt werden, da nach dem Vergleichen Hashes mit einem Sollwert der Wert des Akkumulators auf einen bestimmten Wert gesetzt wird, welcher einen Erfolg oder ein Fehlschlagen des Algorithmus signalisiert.
Nach dem Durchlauf des Programms durch den Emulator `Jade` kann im Akkumulator der Wert $"BE"_16$ abgelesen werden, was bedeutet dass der richtige Hashwert errechnet wird.
Da, wie in @jade_programs_md5 erwähnt wird, durch den Algorithmus ein großer Lawineneffekt besteht, kann mit großer Sicherheit gesagt werden, dass die Emulation von MD5 somit vollständig korrekt ist.

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

Um diesen Algorithmus noch auf Zyklenebene zu validieren, wurde ein Validierungsdurchlauf mit dem perfect6502 durchgeführt.
Das Ergebnis hiervon ist in @validation_md5 zu sehen.
Diese Ergebnisse bestätigen die Testrom-Validierung, da es außer dem Prozessorstatus keine fehlerhaften Zyklen gab.
Bei diesen Statusfehlern, welche in nur 0.7% der Zyklen erscheinen, handelt es sich um die selben Fehler, welche bereits in @dormann_validation_results beleuchtet wurden.


== Benchmarks
=== Jade
=== Perfect6502
=== emulator_6502
=== Vergleich