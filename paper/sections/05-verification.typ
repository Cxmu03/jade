#import "../util.typ": fn-name

= Verifikation und Validierung <verification_validation>
Die Verifikation und Validierung des Emulators für den Prozessor gliedert sich in zwei Teile auf.
Zum einen werden nach #link(<req-cpu-4>, [Anforderung 4]) Performanz-Tests durchgeführt.
Diese sollen überprüfen, ob der Emulator echtzeitfähig ist und in verschiedenen Situationen stabil bleibt. 
Zum anderen soll der Emulator nach #link(<req-cpu-3>, [Anforderung 3]) auf Korrektheit überprüft werden.

== Methodik <validation_methods>
Die Validierung auf Korrektheit geschieht auf mehreren Ebenen.
Dies geschieht aus dem Grund, dass es bei der gewünschten Granularität der Emulation Anforderungen an die Korrektheit gibt, welche unterschiedlich schwer zu testen sind.
Insbesondere muss hierbei die Zeit für einen Validierungsdurchlauf berücksichtigt werden, da engmaschige Validierung sehr zeitaufwändig sein kann.

=== Testroms <test_roms>
Als Testroms werden Programme beschrieben, mit welchen versucht wird einen Aspekt des Systems so vollständig wie möglich zu testen.
Dies können unterschiedlichste Aspekte sein, wie beispielsweise Befehlsausgaben, Befehlstimings, Interrupt-timings oder Speicherinhalte #cite(<6502testroms>).
Da diese Tests versuchen so umfangreich wie möglich zu sein, werden dementsprechent viele Taktzyklen benötigt, was zu einem Bottleneck für die taktgenaue Validierung sein kann (siehe @cycle_validation).

==== Klaus Dormann Test Suite <sec:dormann>

Die Testsuite von Klaus Dormann umfasst eine Reihe von Testroms für den 6502, welche Befehle, den Dezimalmodus, Interrupts und inoffizielle Befehle testen.
Aufgrund des nicht-vorhandenen Dezimalmodus in der NES-Variante der 6502 wird der Dezimaltest ignoriert.
Auch der Test für inoffizielle Opcodes wird aufgrund der Anforderungen nicht weiter behandelt.
Der wichtigste Test ist der Befehlstest, welcher überprüfen soll, ob alle Befehle korrekt implementiert sind.

Da dieser Test ein simples 6502-Programm ist, gibt es keinen simplen Mechanismus um der auszuführenden Einheit zu signalisieren, ob ein Test fehlschlägt, oder ob der Durchlauf der Tests erfolgreich war.
Wenn ein Test für einen bestimmten Befehl fehlschlägt, wird eine Trap ausgelöst, was im Kontext von diesem Programm in einem sich wiederholenden Programmzähler resultiert.
Für den Fall dass alle Tests erfolgreich durchlaufen, wird jedoch ebenfalls dieser Mechanismus ausgelöst.
Je nach Einstellungen, welche Tests genau durchzuführen sind, passiert dies nach 87-97 Millionen Zyklen #footnote([Der Bereich von 87-97 Millionen Zyklen wurde durch Ausprobieren der verschiedenen Einstellungen ermittelt]).
Um also zu bewerten ob ein Test- und vorallem welcher Test fehlgeschlagen ist, muss also manuell analysiert werden, welche Befehle vor dem Detektieren der Trap ausgeführt wurden und in welchem Zyklus diese Trap erkannt wurde.

Mit dem entwickelten Emulator wird diese Testsuite Zyklus-für-Zyklus durchgegangen, bis eine Trap erkannt wird.

==== MD5 <jade_programs_md5>
Ein weiteres Programm, welches für Test- und Validierungszwecke in der `jade-validate` Crate implementiert wurde, ist eine MD5-Implementierung für den 6502.
Die Implementierung des Algorithmus in 6502-Assembly stammt vom Github-Benutzer "lobzega" #footnote(link("https://github.com/laubzega/md5_6502")).
Hierfür wurde dann ein Wrapper-Programm in C geschrieben, welches mit dem `cc65`-Compiler kompiliert wurde.
Dieses Programm führt einen kompletten Hashvorgang eines vordefinierten Wertes aus und überprüft anschließend, ob der errechnete Hashwert mit dem erwarteten Hashwert übereinstimmt.
Ist diese Überprüfung erfolgreich, so wird der Wert `0xBE` in den Akkumulator geladen, ansonsten der Wert `0xFB`. 
Anschließend wird durch den BRK-Befehl ein Interrupt ausgelöst, welcher als Erkennungsmechanismus für die Terminierung des Programms dient.

Das Ziel dieses Programms ist die Demonstration von Korrektheit des Emulators bei Programmen, welche einen großen Lawineneffekt vorweisen.
Der Lawineneffekt ist eine Eigenschaft von kryptographischen Algorithmen, dass eine kleine Änderung in der Eingabe eine größtmögliche Änderung in der Ausgabe erzeugt #cite(<upadhyay2022investigating>).
Existieren also schon kleine Ungenauigkeiten in der Implementierung der Befehle, so ändert sich das Ergebnis maßgeblich.
Im Falle einer korrekten Ausführung kann demnach begründet werden, dass die ausgeführten Befehle funktional korrekt implementiert wurden.
 
=== Taktgenaue Validierung <cycle_validation>
Bei der Taktzyklusvalidierung geht es darum, einen weiteren Emulator oder Simulator mit dem zu validierenden Emulator zu vergleichen. 
Der Simulator, welcher hier als Ground Truth benutzt wird, wird im folgenden Validator genannt. 
Im Gegensatz dazu steht der Generator, also der zu testende Emulator.

Das Prinzip dieser Validierung ist folgendes: der Validator und der Generator werden zeitgleich in einer Schleife laufen gelassen und in jeder Iteration führen beide Emulatoren einen Zyklus, oder nach gewünschter Granularität einen Befehl, aus.
Nach der Befehlsausführung wird aus jedem Emulator der aktuelle Zustand abgespeichert, wie nach #link(<req-cpu-3>, "Anforderung 3") gefordert.
Der Prozessorzustand des Generators wird dann mit dem Zustand des Validators verglichen und die gefundenen Fehler werden gezählt.
Hierbei findet eine Kategorisierung der Fehler in vier Kategorien statt:
#list(
  indent: 2em,
  [*Kontrollfluss*: Es liegt eine Abweichung des Programmzählers zwischen Generator und Validator vor. Dies ist der gravierendste Fehler, welcher in den meisten Fällen eine Fehlerlawine auslöst, da möglicherweise ein völlig unterschiedlicher Pfad durch das Programm genommen wird.],
  [*Register*: Der Generator und Validator unterscheiden sich in einem oder mehreren Registern. Die überprüften Register sind der Akkumulator, die Indexregister X und Y, sowie der Stackpointer. Falls es mehrere Fehler innerhalb eines Zyklus gibt, so werden diese akkumuliert.],
  [*IO*: Ein IO-Fehler besteht dann, wenn Werte auf dem Datenbus, dem Adressbus oder dem Read/Write-Pin in einem Zyklus nicht übereinstimmen. Mit der Überprüfung dieser Fehler kann direkt sichergestellt werden, dass keine Diskrepanzen zwischen den Speichern der beiden Emulatoren entstehen. Somit muss kein zusätzlicher Vergleich der Arbeitsspeicher durchgeführt werden.],
  [*Status*: Der Statusfehler wird durch einen Unterschied im Prozessorstatuswort charakterisiert. Es kann sich um eine oder mehrere Flaggen handeln, jedoch wird dies stets als ein einzelner Fehler gewertet. Unter Umständen könnte noch eine weitere Aufteilung angedacht werden für Flaggen, welche Kontrollflussrelevant sind (Carry, Overflow, Zero, Negative) und Flaggen, welche darauf keinen Einfluss nehmen können (Break, Bit 5, Interrupt Disable, Decimal). Eine Notwendigkeit hierfür ergibt sich jedoch nicht, da die kontrollflussrelevanten Flaggen bei einem Fehlverhalten zu einem abgeänderten Kontrollfluss führen, welcher bereits durch einen bestehenden Fehlertyp abgedeckt wird.]
) <validation_error_types>
Anhand hiervon kann dann eine Fehlerquote von jedem Fehler für den Validierungsdurchlauf berechnet werden.

Obwohl die Validierungsarchitektur zwar generisch und modular aufgebaut ist, geschieht ein erster Validierungsdurchlauf jedoch nur mit dem Perfect6502 (siehe @visual6502).
Dieser Simulator wurde gewählt, da durch die Simulation auf Transistor-Ebene eine extreme Genauigkeit entsteht und der Perfect6502 als C-Reimplementierung des Visual6502 an Performanz gewinnt, gegenüber der originalen Implementierung in Javascript.
Des weiteren kann die Validierung so auf Zyklenebene durchgeführt werden.

=== Testrom-Validierung
Die Testrom-Validierung ist das zweite Validierungsverfahren, welches für den Emulator implementiert wird.
Für diese Art der Validierung wird im Regelfall nur ein Emulator gebraucht, welcher getestet werden soll, es ist jedoch nicht ausgeschlossen, dieses Verfahren mit der taktgenauen Validierung zu kombinieren.

In diesem Verfahren wird der Emulator mit einer Testrom, wie sie in @test_roms beschrieben wird, ausgeführt. 
Dies wird so lange fortgeführt bis eine Abbruchbedingung erreicht wird, die einen Fehler oder den Erfolg des Programms signalisieren könnte.
Jede Testrom kann eine eigene spezielle Abbruchbedingung haben, im Fall der Klaus-Dormann-Testsuite ist dies eine Trap des Programmzählers (siehe @sec:dormann).
Solch eine Bedingung kann zwar automatisiert erkannt werden, jedoch ist bei der Auswertung eine manuelle Analyse möglich, ob- und was für ein Fehler erkannt wurde, oder ob die Tests erfolgreich durchgelaufen sind.


== Architektur <verification_architecture>
Für die Verifikation und Validierung wurde ein generisches Framework entworfen, um verschiedenste Emulatoren miteinander vergleichen zu können und verschiedene vordefinierte Programme auf diesen laufen lassen zu können.
Ein Fokus liegt hierbei auch darauf, dass Emulatoren, welche in verschiedensten Sprachen geschrieben wurden, eingebunden werden können. 
Somit kann die größte Flexibilität für eine Überprüfung der Funktionsweise erreicht werden.
Die allgemeine Architektur dieser Infrastruktur kann in @validation_framework gesehen werden.

#figure(
  image("../resources/jade_validate.svg", width: 100%),
  caption: "Architektur der Validierungsinfrastruktur"
) <validation_framework>

Die Funktionalität wird hierbei auf verschiedene Crate aufgeteilt,um eine logische Trennung zu erhalten und die Wiederverwendbarkeit von Komponenten zu maximieren.
Crates sind logische Kapselungen von Modulen, Dependencies und Build-Logik, welche von der Rust Programmiersprache benutzt werden #cite(<crates>).
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

=== `jade_validate`
Die `jade_validate` Crate definiert die allgemeine Validierungsinfrastruktur.
Darunter fallen Schnittstellen für Emulatoren, Wrapper für Emulatoren welche diese Schnittstellen definieren, Funktionen für die Validierung und ein Command-Line-Interface (CLI), mit dem sich diese Crate bedienen lässt.

==== Traits <jade_validate_traits>
Den Kern dieser Crate bilden die beiden Traits für den Generator und Validator, wie sie bereits in @cycle_validation angeschnitten wurden.
Wie in @generator_validator_traits gesehen werden kann, sind dies Supertraits, welche verschiedene andere Traits benötigen.

Für die tatsächliche Ausführung gibt es den `StepCycle` Trait, welcher eine Methode bereitstellt, um die Ausführung eines Emulators um einen Zyklus voranzutreiben. 
Der `LoadExecutable` Trait ist dafür verantwortlich, ein ausführbares Programm aus einem Buffer oder einer Datei in den Arbeitsspeicher eines Emulators einzulesen.
Mit dem `SnapshotLog` Trait wird ermöglicht, eine Momentaufnahme des Prozessorzustands aufzunehmen.
Diese enthält alle Register und Busse, welche für eine Überprüfung auf Korrektheit nach #link(<req-cpu-3>, "Anforderung 3") gefordert werden..
Der `HasName` Trait stellt sicher, dass ein Emulator über den Namen in einer generischen Funktion identifiziert werden kann.
Ein letzter Subtrait, welcher zwischen diesen Traits geteilt wird, ist `std::fmt::Debug`, welcher benutzt wird um Backtraces der letzten Zyklen im Fall eines Fehlers zu zeigen. 
Der `HasInitialCpuStatus` Trait wird jedoch nur vom einem Validator implementiert.
Dies hat den Grund, dass ein Validator einen Anfangszustand für die Emulation bereitstellen muss, an dem sich der Generator richtet.
So wird sichergestellt, dass nicht aufgrund von unterschiedlichen Startzuständen fälschlicherweise Fehler in der Emulation erkannt werden.
Den bereitgestellten Startzustand kann durch den Generator geladen werden, indem dieser den `InitializeWithCpuStatus` Trait implementieren muss.

#figure(
  ```rust
  pub trait Generator:
      InitializeWithCpuStatus + SnapshotLog + StepCycle + 
      LoadExecutable + HasName + fmt::Debug
  {
  }

  pub trait Validator:
      HasInitialCpuStatus + SnapshotLog + StepCycle + 
      LoadExecutable + HasName + fmt::Debug
  {
  }
  ```,
  placement: top,
  caption: [Traits für Generatoren und Validatoren]
) <generator_validator_traits>

==== Emulatoren <jade_validate_emulators>
Die `jade-validate` Crate verfügt über verschiedene vorimplementierte Emulatoren, welche für die Validierung und Benchmarks benutzt können werden.
Hierunter fällt ein implementierter Generator, ein Validator und ein weiterer Emulator, welcher keinen der beiden Traits implementiert und somit nur für Benchmarks benutzt wird.
Weitere Emulatoren können jederzeit über Implementierung der `Generator` und `Validator` Traits hinzugefügt werden.

Als primärer Validator ist der perfect6502 angedacht, da dieser eine hohe Genauigkeit bietet.
Ein Hindernis in der Implementierung eines Wrappers ist jedoch, dass der perfect6502 in der Programmiersprache C geschrieben ist.
Dieses Problem kann jedoch umgangen werden, da Rust zur Compile-Zeit ein flexibles Build-Skript-System anbietet.
Der erste Schritt beim Kompilieren dieser Crate besteht daraus, Bindings für die perfect6502-Library mithilfe von `bindgen`#footnote("https://rust-lang.github.io/rust-bindgen/") zu generieren.
Hierbei werden in Rust-Funktionsköpfe gebildet, welche mit den Symbolen aus C zusammen gelinked werden können.

#figure(
  ```rust
  // Originaler Funktionskopf in C
  extern unsigned short readPC(state_t *state);

  // Automatisch generierter Funktionskopf von rust-bindgen
  unsafe extern "C" {
      pub fn readPC(state: *mut ::std::os::raw::c_void) 
          -> ::std::os::raw::c_ushort;
  }
  ```,
  caption: "Beispielhaft generiertes Rust-Binding"
)

Anschließend wird die perfect6502-Library aus dem Build-Skript heraus vom Quellcode kompiliert, wofür ein C-Kompilierer auf dem System vorhanden sein muss.
Sobald die Kompilierung abgeschlossen ist, kann die gebaute Bibliothek in das `jade-validate`-Executable statisch gelinked werden.
Damit ist dann ein Foreign Function Interface (FFI) nach C hergestellt.
Um diesen C-Code kann dann eine sichere Rust-Abstraktion gebaut werden, welche alle benötigten Traits implementiert.

Da der Jade-Emulator im Zuge dieser Arbeit entwickelt wird, ist dies auch der erste implementierte Generator, welcher getestet werden soll.
Um die bereits vorhandene `jade`-Crate wird hierfür ein dünner Wrapper geschrieben, welcher den `Generator`-Trait implementiert.

Ein dritter Emulator, für den ein Wrapper in dieser Crate erstellt wird, ist der `emulator_6502`#footnote("https://docs.rs/emulator_6502/latest/emulator_6502/"), welcher ein frei verfügbarer 6502-Emulator im offiziellen Repository für Rust-Crates #footnote("https://crates.io/") ist.
Dieser Emulator ist jedoch nicht grundsätzlich zyklengenau, obwohl er eine Funktion anbietet, der die Ausführung um einen Zyklus voranschreitet.
Dies geschieht, indem der Emulator einen Befehl im letzten Zyklus vollständig ausführt, während die Zyklen davor nichts getan wird. 
Der `emulator_6502` ist primär als Vergleichsemulator für die Performanz gedacht, da er aufgrund dieser Implementierung weder ein guter Generator- , noch ein guter Validator ist.

// TODO: Should probable rewrite this section

== Benchmarks <benchmarks>
Benchmarks sind eine weitere wichtige Komponente um die Anforderungen an der Emulator zu validieren und einen Vergleich mit anderen Emulatoren durchzuführen.
Die Anforderung welche hiermit überprüft wird ist #link(<req-cpu-4>, "Anforderung 4"), welche die Echtzeitfähigkeit des Emulators fordert.
Dieser muss in der Ausführung eine Mindestgeschwindigkeit von 1.8MHz aufweisen.

In @benchmark_method wird erklärt, wie die durchgeführten Benchmarks aufgebaut sind und welche Ziele mit diesem Aufbau verfolgt werden.
@benchmark_implementation geht dann näher auf die technische Implementierung dieser Benchmarks ein.

=== Methodik <benchmark_method>
Die gewählte Methodik für die Durchführung gliedert sich in 3 Teile auf, nämlich die getesteten Emulatoren, die ausgewählten Programme und das Format der Benchmarks.

Für die Benchmarks werden die 3 bereits genannten Emulatoren aus @jade_validate_emulators benutzt.
Dies wurde so gewählt, da all diese Emulatoren grundlegend verschiedene Ziele verfolgen, und anhand von diesen Ergebnissen somit bestimmte Anforderungen validiert- und Designentscheidungen begründet werden können.
Da der perfect6502 die Genauigkeit als oberstes Ziel hat, leidet die Performanz darunter voraussichtlich deutlich.
Der `emulator_6502` ist das absolute Gegenstück hierzu, da Geschwindigkeit priorisiert wird, jedoch die Genauigkeit auf Zyklenebene darunter leidet.
`Jade` versucht hierzu den perfekten Kompromiss zu finden mit einer hohen Genauigkeit, welche mit den Methoden aus @validation_methods validiert wird, sowie einer vergleichbaren Performanz zum 'emulator_6502', um echtzeitfähig zu bleiben.

Die Auswahl der Programme, welche im Benchmark ausgeführt werden, wurde so getroffen, um grundlegend verschiedene Anwendungsfälle darzustellen um somit die Performanz eines Emulators zwischen diesen Anwendungsfällen und Komplexitäten vergleichen zu können.

Das erste und einfachste Programm ist das Standardprogramm des Visual6502-Simulators, welches in @visual6502_default_program dargestellt ist.
Hierbei handelt es sich um ein simples Programm mit wenigen Befehlen, welches in einer engen Schleife mit einem Funktionsaufruf das Indexregister X inkrementiert, das Indexregister Y dekrementiert und den Wert des Akkumulators um 3 erhöht.

Bei dem zweiten Programm handelt es sich um die Implementierung des MD5-Hashes, welche in `jade_programs` implementiert ist und in @jade_programs_md5 vorgestellt wurde.
Hierbei handelt es sich zwar auch um eine repetitive Rechnung, jedoch werden mehr verschiedene Befehle durch die höhere Komplexität abgedeckt.
Durch den Benchmark gilt es zu entscheiden, ob sich diese erhöhte Komplexität auf die Ausführungsgeschwindigkeit des Emulators auswirkt.

Das letzte Programm ist die Testsuite von Klaus Dormann (siehe @sec:dormann), welche eine große Breite von verschiedenen Befehlen abdeckt und somit die größte Variation dieser Programme aufweist.
Da dieses Programm bereits für die Validierung in `jade_programs` integriert wird, gestaltet sich die Integration in die Benchmarks sehr einfach.  

#figure(
  placement: top,
  ```asm
  0000    LDA #$00
  0002    JSR 0010
  0005    JMP 0002
  000F    RTI
  0010    INX
  0011    DEY
  0012    INC $0F
  0014    SEC
  0015    ADC #$02
  0017    RTS
  ```,
  caption: [Standardprogramm des Visual6502]
) <visual6502_default_program>

Der dritte Parameter eines Benchmarks ist die Anzahl der Zyklen, welche in dem aktuellen Durchlauf ausgeführt wird.
Hiermit soll getestet werden, wie sich die Performanz eines Emulatoren mit unterschiedlicher Ausführungsdauer verhält, da Aspekte wie Branch Prediction (Sprungvorhersage) hier ins Spiel kommen könnten.
Diese Zyklenanzahlen sind für alle drei Emulatoren gleich und reichen auf einer logarithmischen Skala von $1 dot 10^2$ bis $1 dot 10^6$, womit eine große Reichweite abgedeckt werden kann.

Für jede Kombination aus diesen Merkmalen wird dann ein Benchmark durchgeführt, was $3 dot 3 dot 5=45$ Benchmarks entspricht.
Die genaue technische Durchführung dieser Benchmarks wird im folgenden Kapitel näher erläutert.

=== Durchführung <benchmark_implementation>
Die Durchführung der Benchmarks geschieht mit der Rust-Bibliothek `Criterion.rs`#footnote("https://docs.rs/criterion/latest/criterion/"), welche eine Portierung der Haskell-Bibliothek `Criterion`#footnote("https://hackage.haskell.org/package/criterion") ist.
Das Hauptziel dieser Bibliotheken ist die Durchführung und eine ausführliche statistische Auswertung der Benchmarks und deren Ergebnisse.

Die Benchmarks gliedern sich in der Durchführung in verschiedene Teile auf.
Im folgenden bezeichnet ein *Durchlauf* einen vollständigen Benchmark einer Parameterkombination, wie sie in @benchmark_method vorgestellt wurden.
Criterion sammelt dann pro Benchmark eine bestimmte Anzahl an *Samples*, welche für die statistische Auswertung dienen.
Jedes Sample wird Charakterisiert als der Durchschnitt aus einer variablen Anzahl an *Iterationen*, welche von Criterion während der Laufzeit bestimmt wird.
In jeder Iteration findet dann ein tatsächlicher Durchlauf von n Zyklen eines bestimmten Programms statt.

Die Bestimmung der Anzahl von Iterationen pro Sample und die Entwicklung dieser Anzahl innerhalb eines Durchlaufs wird von Criterion in der Warmup-Phase bestimmt.
Diese Phase führt bereits wenige Iterationen der zu benchmarkenden Funktion aus, um den Prozessor und die Runtime auf den Benchmark vorzubereiten, also Caches zu füllen und im Fall eines JIT-Compilers diesen bereits aufzuwärmen #cite(<criterion>).
Anhand von den gesammelten Daten im Warmup entscheidet sich Criterion dann für einen von zwei Sampling-Modes, *Flat* und *Linear*, welche bestimmen wie sich die Anzahl der Iterationen pro Sample verändert.
Mit einem Sampling-Mode von Flat verändert sich die Anzahl der Iterationen nicht und bleibt für jedes gesammelte Sample konstant.
Dies wird benutzt wenn die Laufzeit eines bestimmten Benchmarks zu groß für den Linear-Modus wäre. 
Der Linear-Modus lässt die Iterationen in jedem Sample linear anwachsen.