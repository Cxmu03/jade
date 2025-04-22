= Fazit und Ausblick
Die ursprüngliche Zielsetzung dieser Arbeit war die Entwicklung eines NES-Emulators, welcher gleichzeitig eine sehr hohe Präzision und eine hohe Performanz aufweist. 
Diese Zielsetzung konnte in der vorgegebenen Zeit nur teilweise erreicht werden, da die Validierung auf Zyklenebene, Entwicklung der Infrastruktur hierfür und der manuelle Abgleich mit hardwarenahen Quellen viel Zeit in Anspruch nahm.

Für die Hauptkomponente des NES, den 6502, wurde ein vollständiger Emulator fertiggestellt, welcher alle gesetzten Ziele und Anforderungen erfüllt.
Im Vordergrund steht für diesen Emulator eine sehr feine Granularität der Ausführung auf Zyklenebene, so dass eine möglichst hohe Genauigkeit erreicht werden kann.
Die Zyklen werden durch einen Mikrocodeansatz implementiert, so dass die einzelnen Zyklen eines Befehls zeitlich völlig isoliert voneinander ausgeführt werden können, so lange der interne Zustandsautomat den richtigen Zustand besitzt.
Die Korrektheit dieser Emulation wurde mit dem Simulator `Perfect6502` an verschiedenen Testprogrammen validiert.
Dabei zeigte sich, dass eine hohe, echtzeitfähige Performanz auch mit einer sehr präzisen Emulation zu vereinen ist.
Der Emulator kann Taktraten von bis zu $69$MHz erreichen.

Die restlichen Komponenten des NES, also die PPU und die APU, konnten zeitlich begründet nicht implementiert werden.
In einer zukünftigen Arbeit könnten diese Teilkomponenten und eine Integration zu einem Gesamtsystem näher beleuchtet werden.
Mit dem Fertigstellen eines CPU-Emulators, welcher die Performanz- und Präzisionsanforderungen erfüllt ist jedoch bereits ein Meilenstein geschafft.
Da die CPU der komplizierteste Hardwarebaustein des NES ist, beweist dies, dass eine ähnlich gute Emulation der anderen Teilkomponenten und somit auch des Gesamtsystems möglich ist.