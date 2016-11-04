Übungsaufgaben zu „Programmieren in Rust“
=========================================

**Gruppe**:

- *Susi Sorglos*
- *Willi Wacker*

---

In diesem Repository werden die Lösungen der oben genannten Gruppe zu den
Programmieraufgaben in der Vorlesung „Programmieren in Rust“ gesammlt. Dazu
muss mindestens ein Gruppenmitglied diese Repository forken. Von dem Fork
wird später dann pro Aufgabenblatt ein *Pull Request* (PR) geöffnet.

Die Lösungen müssen in einer bestimmten Ordnerstruktur liegen, die wie folgt
aussieht:

```
README.md
sheet1/
    task1/
    task2/
sheet2/
    task1/
    task2/
```

In den `taskN`-Unterordnern muss dann eure Lösung für die entsprechende Aufgabe
liegen.

### Neue Aufgaben herunterladen

Neue Übungsaufgaben werden in [diesem Repository][1] veröffentlicht. Es kann
vorkommen, dass mit der Aufgabenstellung schon ein paar Quelldateien vorgegeben
werden; diese müsst ihr dann von dem genannten Repository in euren Fork
kopieren. Dies geht am besten, indem ihr beide Repositories (euren Fork des 
Gruppenrepositories *und* [oben genanntes Repository][1]) in den selben Ordner 
klont (sodass sie nebeneinander liegen). Das ganze sieht dann in etwa so aus:

```
$ git clone git@github.com:SusiSorglos/pir-ssorglos-wwacker.git
$ git clone https://github.com/LukasKalbertodt/programmieren-in-rust.git
$ ls
pir-ssorglos-wwacker/         programmieren-in-rust/
```

Wenn eine neue Aufgabe veröffentlicht wurde, muss zunächst 
`programmieren-in-rust` aktualisiert werden:

```
$ cd programmieren-in-rust
$ git pull
$ cd ..
```

Nun kann die Aufgabenstellung (mit Quelldateien) des `N`ten Arbeitsblatts 
in euren Fork kopiert werden:

```
$ cp -r programmieren-in-rust/sheetN pir-ssorglos-wwacker/
```

### Travis-CI

Um das Arbeiten zu erleichtern, ist für alle Lösungsrepositories ein Continuous
Integration Service aufgesetzt worden. Jedes mal, wenn ein PR geöffnet
oder aktualisiert wird, laufen eine Reihe von Tests durch, die den Codestil
prüfen, alle Rust Dateien kompilieren und alle Unit-Tests ausführen. Manuell
könnt ihr all diese Tests durchlaufen lassen, indem ihr im Wurzelverzeichnes
des Repositories folgenden Befehl ausführt (nur Linux und Verwandte!):

```
./ci/run-all.sh
```

Jeder PR hat also einen Status: *passed*, *failed* oder *pending*. Euer PR zum
Einreichen der Aufgaben muss bis Sonntag Abend den Status *passed* erreicht
haben, also plant genug Zeit zum Verbessern von kleinen Fehlern ein und öffnet
den PR nicht erst kurz vor Schluss.


[1]: https://github.com/LukasKalbertodt/programmieren-in-rust
