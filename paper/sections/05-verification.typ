#import "../util.typ": fn-name

= Verifikation und Validierung
Die Verifikation und Validierung des Emulators für den Prozessor gliedert sich in zwei Teile auf.
Zum einen werden nach #link(<req-cpu-4>, [Anforderung 4]) Performanz-Tests durchgeführt.
Diese sollen überprüfen, ob der Emulator echtzeitfähig ist und in verschiedenen Situationen stabil bleibt. 
Zum anderen soll der Emulator nach #link(<req-cpu-3>, [Anforderung 3]) auf Korrektheit überprüft werden.

== Architektur <verification_architecture>
Für die Verifikation und Validierung wurde ein generisches Framework entworfen, um verschiedenste Emulatoren miteinander vergleichen zu können und verschiedene vordefinierte Programme auf diesen laufen lassen zu können.
Ein Fokus liegt hierbei auch darauf, dass Emulatoren, welche in verschiedensten Sprachen geschrieben wurden, eingebunden werden können. 
Somit kann die größte Flexibilität für eine Überprüfung der Funktionsweise erreicht werden.
Die allgemeine Architektur dieser Infrastruktur kann in @validation_framework gesehen werden.

#figure(
  image("../resources/jade_validate.svg", width: 100%),
  caption: "Architektur der Validierungsinfrastruktur"
) <validation_framework>

Die Funktionalität wird hierbei auf verschiedene Crates (siehe /*TODO: citation*/) aufgeteilt, um eine logische Trennung zu erhalten und die Wiederverwendbarkeit von Komponenten zu maximieren.
Zu der bereits vorhandenen Crate `jade`, welche die Kernfunktionalität des Emulators enthält (siehe @emulation_implementation), werden die beiden Crates `jade_programs` und `jade_validate` hinzugefügt.

=== `jade_programs`
Die `jade_programs` Crate enthält eine gemeinsame Schnittstelle für ein ausführbares 6502-Programm, sowie einige Programme, welche diese Schnittstelle implementieren.
Dabei gibt es bereits vordefinierte Programme, welche direkt aus der Crate geladen werden können, sowie eine Implementierung, welche beliebige weitere Programme aus Dateien laden kann.

Die allgemeine Schnittstelle für diese Programme ist wie folgt definiert:

#figure(
  ```rust
  pub trait JadeProgram {
      fn get_start_address(&self) -> u16;
      fn get_load_address(&self) -> u16;

      fn get_executable(&self) -> &[u8];

      fn get_name(&self) -> &str;
  }
  ```,
  caption: "Schnittstelle für ein Programm"
)

Die ersten beiden Funktionen #fn-name("get_start_address") und #fn-name("get_load_address") sind essenziell für das Laden des Programms in den Arbeitsspeicher.
Über #fn-name("get_load_address") weiß der Loader, an welche Adresse im RAM das Programm als zusammenhängender Speicherblock geladen werden soll.
Nach dem Laden des Programms schreibt der Loader dann mithilfe von #fn-name("get_start_address") die Startadresse des Programms in den Reset-Vektor.
Die Ladeadresse und die Startadresse sind zwar in vielen Fällen identisch, können sich jedoch auch unterscheiden.

Über #fn-name("get_executable") erhält der Loader dann das tatsächliche Programm als Referenz zu einem Bytearray.
Woher das Programm dann kommt ist nicht relevant, es kann auf dem Stack, Heap oder auch im Datensegment liegen.

Zuletzt muss ein Programm auch noch in der Lage sein, sich eindeutig zu identifizieren.
Hierfür muss die #fn-name("get_name") Funktion implementiert werden, welche einen String zurückgibt.
/* TODO: write more */

=== `jade_validate`
Die `jade_validate` Crate definiert die allgemeine Validierungsinfrastruktur.
Darunter fallen Schnittstellen für Emulatoren, Wrapper für 

== Validierung
Die Validierung auf Korrektheit geschieht auf mehreren Ebenen.
Dies geschieht aus dem Grund, dass es bei der gewünschten Granularität der Emulation Anforderungen an die Korrektheit gibt, welche unterschiedlich schwer zu testen sind.
Insbesondere muss hierbei die Zeit für einen Validierungsdurchlauf berücksichtigt werden, da engmaschige Validierung sehr zeitaufwändig sein kann.

=== Testroms
Als Testroms werden Programme beschrieben, mit welchen versucht wird einen Aspekt des Systems so vollständig wie möglich zu testen.
Dies können unterschiedlichste Aspekte sein, wie beispielsweise Befehlsausgaben, Befehlstimings, Interrupt-timings oder Speicherinhalte #cite(<6502testroms>).
Da diese Tests versuchen so umfangreich wie möglich zu sein, werden dementsprechent viele Taktzyklen benötigt, was zu einem Bottleneck für die taktgenaue Validierung sein kann (siehe @cycle_validation).

==== Klaus Dormann Test Suite

Die Testsuite von Klaus Dormann umfasst eine Reihe von Testroms für den 6502, welche Befehle, den Dezimalmodus, Interrupts und inoffizielle Befehle testen.
Aufgrund des nicht-vorhandenen Dezimalmodus in der NES-Variante der 6502 wird der Dezimaltest ignoriert.
Auch der Test für inoffizielle Opcodes wird aufgrund der Anforderungen nicht weiter behandelt.
Der wichtigste Test ist der Befehlstest, welcher überprüfen soll ob alle Befehle korrekt implementiert sind.

Da dieser Test ein simples 6502-Programm ist, gibt es keinen simplen Mechanismus um der auszuführenden Einheit zu signalisieren, ob ein Test fehlschlägt, oder ob der Durchlauf der Tests erfolgreich war.
Wenn ein Test für einen bestimmten Befehl fehlschlägt, wird eine Trap ausgelöst, was im Kontext von diesem Programm in einem sich wiederholenden Programmcounter resultiert.
Für den Fall dass alle Tests erfolgreich durchlaufen, wird jedoch ebenfalls dieser Mechanismus ausgelöst.
Je nach Einstellungen, welche Tests genau durchzuführen sind, passiert dies nach 87-97 Millionen Zyklen #footnote([Der Bereich von 87-97 Millionen Zyklen wurde durch Ausprobieren der verschiedenen Einstellungen ermittelt]).
Um also zu bewerten ob ein Test- und vorallem welcher Test fehlgeschlagen ist, muss also manuell analysiert werden, welche Befehle vor dem Detektieren der Trap ausgeführt wurden und in welchem Zyklus diese Trap erkannt wurde.

Mit dem entwickelten Emulator wird diese Testsuite Zyklus-für-Zyklus durchgegangen, bis eine Trap erkannt wird.
Hierbei hat sich ergeben, dass die alle funktionalen Tests ohne Fehler durchlaufen werden konnten.
Jedoch muss angemerkt werden, dass es sich dabei nicht um eine Validierung auf Zyklenebene handelt.
Mit diesem Programm wird nur das Verhalten von Befehlen als ganze Einheit getestet, mit einem Fokus auf dem richtigen Setzen von Statusflaggen.
/* TODO: Maybe some speed data here */

=== Taktgenaue Validierung <cycle_validation>
Bei der Taktzyklusvalidierung geht es darum, einen weiteren Emulator oder Simulator mit dem zu validierenden Emulator zu vergleichen. 
Der Simulator, welcher hier als Ground Truth benutzt wird, wird im folgenden Validator genannt. 
Im Gegensatz dazu steht der Generator, also der zu testende Emulator.

Die Validierungsarchitektue ist zwar generisch und modular aufgebaut, ein erster Validierungsdurchlauf geschieht jedoch nur mit dem Perfect6502 (siehe @visual6502).
Dieser Simulator wurde gewählt, da durch die Simulation auf Transistor-Ebene eine extreme Genauigkeit entsteht und der Perfect6502 als C-Reimplementierung des Visual6502 an Performanz gewinnt, gegenüber der originalen Implementierung in Javascript.

== Benchmarks