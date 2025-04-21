= Einleitung
== Motivation
In den frühen Tagen der kommerziellen Videospielindustrie, etwa ab 1970, war der Markt für Videospiele und Videospielkonsolen von einer deutlichen Instabilität gekennzeichnet #cite(<Ernkvist2008>).
In den USA brach dieser Markt 1983 mit dem Fall von Arcade-Systemen ein.
Die Veröffentlichung des Nintendo Entertainment System (NES) änderte diesen Trend.
Das NES ist eine Videospielkonsole, welche von  Nintendo 1983 auf den japanischen- , und 1985 auf den amerikanischen Markt gebracht wurde. 
Trotz anfänglicher Bedenken #cite(<Keizer1989>) leistete einen fundamentalen Beitrag zur Videospiel- und allgemeinen Popkultur, indem es zahlreiche bedeutende Franchises einführte, darunter Super Mario Bros., The Legend of Zelda und Tetris.

Da alte Videospielkonsolen nicht mehr produziert werden, gibt es nur begrenzte Möglichkeiten, diese Videospiele in ihrer originalen Form zu spielen.
Die beiden Alternativen sind Nachbildung der Hardware und Emulation, also das Simulieren der Hardware in Software.

== Problemstellung
In dieser Studienarbeit wird ein Emulator für das Nintendo Entertainment System entwickelt, welcher aus Gründen der historischen Präservation eine möglichst große Genauigkeit aufweisen soll.

Die Implementierung eines Emulators ist eine Herausforderung in der Softwareendwicklung, abhängig von der Komplexität der emulierten Hardware und der gewünschten Granularität.
Zwar ist die verwendete Hardware im NES recht simpel im Vergleich zu modernen Konsolen und Computern, durch die Hardwarelimitationen entstanden jedoch clevere und knifflige Verhalten des Systems.
Dazu zählen beispielsweise besonderen Mapper-Hardware in den Kassetten, unterschiedliches Hardwareverhalten in verschiedenen Regionen oder Rendern des Bildes durch die PPU.
== Ziel der Arbeit
Die Entwicklung und Modellierung von Emulatoren ist oft geprägt von einem Speed-Accuracy-Tradeoff #cite(<Mallach1975>).
Eine höhere Genauigkeit der Emulation fordert immer eine genauere Modellierung der Hardware und der Interaktion zwischen verschiedenen Komponenten.
In einem komplexen Hardwaresystem führt dies unausweichlich zu einer niedrigeren Performanz.
Ein Emulator welcher auf einer sehr hohen Abstraktionsebene arbeitet um eine hohe Performanz zu erreichen kann die Interaktionen auf Hardwareebene nicht fein genug abbilden.

Da das NES jedoch im Vergleich zu moderner Hardware simpler aufgebaut ist, wird in dieser Arbeit untersucht, ob es möglich ist, einen sehr präzisen NES-Emulator zu schreiben, welcher zusätzlich eine hohe Performanz aufweist.
Hierfür müssen Abstraktionen gefunden werden, welche das Hardwareverhalten der NES abbilden können, ohne an Performanz zu stark einzubußen.
Um dies dann bewerten zu können, werden Kriterien für die Genauigkeit und Performanz entwickelt.

== Arbeitsschritte
In diesem Kapitel werden die benötigten Arbeitsschritte vorgestellt, um das Ziel der Arbeit erreichen zu können.

== Anmerkungen an Leser