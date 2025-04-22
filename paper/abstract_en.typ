//Diese Arbeit beschäftigt sich mit der Entwicklung eines hochpräzisen und performanten Emulators für das Nintendo Entertainment System (NES).
//Hierfür muss eine Balance zwischen der genauen Abbildung der Hardware und der Performanz des Systems gefunden werden.
//Die Emulation der Central Processing Unit (CPU) wird hierbei in den Vordergrund gestellt.
//Hierfür wird ein microcodebasierten Ausführungsmodell entworfen, welches es dem Emulator ermöglicht die Ausführung von Befehlen in einzelnen Zyklen durchzuführen.
//Die Korrektheit der Emulation wird anschließend mit Testroms und einer Hardwaresimulation des 6502 validiert. 
//Abschließend wird durch Benchmarks gezeigt, dass diese Emulation zusätzlich echtzeitfähig ist.

This thesis concerns itself with the development of a highly precise and performant emulator for the Nintende Entertainment System (NES).
To achieve this, a balance between the exact digital depiction of the hardware and the performance of the system must be found.
The focus of this work lies in the emulation of the Central Processing Unit (CPU).
A microcode-based execution model is developed, which enables the emulator to execute instructions at a cycle-level granularity.
The correctness of the emulation will then be validated with test roms and a hardware simulation of the 6502.
Finally, the real-time capabilities of the emulator will be demonstrated through a series of benchmarks.