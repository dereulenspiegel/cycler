# Cycler (name subject to change)

Dies ist ein kleines Projekt welches zum Ziel hat das Verhältnis von Autoinfrastruktur zu
Fahrradinfrastruktur in Städten zu analysieren. Es analysiert dazu OSM-Daten im Protobuf-Format
und kalkuliert die Länge aller Straßen. Dabei werden nur Straßen berücksichtigt auf denen Autos
potentiell Vorrang haben. Es werden also "Spielstraßen", aber auch Servicewege etc. ignoriert.
Zusätzlich wird zu jeder Straße das Vorhandensein von Fahrradinfrastruktur analysiert. Am Ende
erhält man die Länge aller Straßen, die Länge aller Fahrradspuren, die Länge aller baulich getrennter
Fahrradwege und die Länge aller Fahrradstraßen.

# TODO

* Direkten Download von .osm.pbf-Dateien unterstürten inkl. Auswertung des Last-Modified Headers
* Alte Ergebnisse in einer Liste speichern (inkl. Datum, Quelle etc.)
* Ausbau zum Twitter/Mastodon-Bot der regelmäßig die aktuellen Statistiken und Veränderungen berichtet.

# Aktuelle Ergebnisse (21.09.2018) für Dortmund
Quelle: https://download.bbbike.org/osm/bbbike/Dortmund/Dortmund.osm.pbf

| Typ | Länge |
| --- | ----- | 
| Öffentlichen Straßen | 1747.88 km |
| Fahrradspuren | 35.52 km |
| Baulich abgegrenzte Fahrradwege | 24.39 km |
| Fahrradstraßen | 0 km |
