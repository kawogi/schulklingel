#include "Arduino.h"

// Der Pin an welchem der Lautsprecher angeschlossen ist
#define LAUTSPRECHER_PIN PIND6

// Nummern der Halbtöne
#define NOTE_C 0
#define NOTE_CIS 1
#define NOTE_D 2
#define NOTE_DIS 3
#define NOTE_E 4
#define NOTE_F 5
#define NOTE_FIS 6
#define NOTE_G 7
#define NOTE_GIS 8
#define NOTE_A 9
#define NOTE_AIS 10
#define NOTE_H 11

void setup()
{
  // Den Pin des Lautsprechers auf "Ausgabe" umschalten
  pinMode(PIND6, OUTPUT);
}

/// @brief Berechnet die Frequenz einer Note
/// @param note Nummer des Halbtons innerhalb einer Oktave (C = 0)
/// @param oktave Nummer der Oktave
/// @return die berechnete Frequenz in Hz
float frequenz(int note, int oktave)
{
  // setze Kammerton A4 = 440 Hz als Referenz für alle anderen Noten
  const int BASISFREQUENZ = 440;
  const int BASISOKTAVE = 4;
  // A ist der 9. Halbton in einer Oktave
  const int BASISNOTE = 9;
  // Gleichstufige Stimmung: https://de.wikipedia.org/wiki/Gleichstufige_Stimmung#Frequenzberechnung
  return BASISFREQUENZ * pow(2.0, (oktave - BASISOKTAVE) + (note - BASISNOTE) / 12.0);
}

// einige vorberechnete Tonfrequenzen für übliche Melodien
const int FREQUENZ_C3 = frequenz(NOTE_C, 3);
const int FREQUENZ_D3 = frequenz(NOTE_D, 3);
const int FREQUENZ_E3 = frequenz(NOTE_E, 3);
const int FREQUENZ_F3 = frequenz(NOTE_F, 3);
const int FREQUENZ_G3 = frequenz(NOTE_G, 3);
const int FREQUENZ_A3 = frequenz(NOTE_A, 3);
const int FREQUENZ_H3 = frequenz(NOTE_H, 3);
const int FREQUENZ_C4 = frequenz(NOTE_C, 4);
const int FREQUENZ_D4 = frequenz(NOTE_D, 4);
const int FREQUENZ_E4 = frequenz(NOTE_E, 4);
const int FREQUENZ_F4 = frequenz(NOTE_F, 4);
const int FREQUENZ_G4 = frequenz(NOTE_G, 4);
const int FREQUENZ_A4 = frequenz(NOTE_A, 4);
const int FREQUENZ_H4 = frequenz(NOTE_H, 4);
const int FREQUENZ_C5 = frequenz(NOTE_C, 4);

/// @brief Spielt den Ton als Rechteckwelle auf dem Lautsprecher ab.
///
/// Die Funktion blockiert so lange, bis der Ton abgeschlossen ist
///
/// @param frequenz die Tonfrequenz in Hertz (Hz)
/// @param dauer_ms die Dauer des Tons in Millisekunden
void spieleTon(float frequenz, int dauer_ms)
{
  // die Dauer einer einzelnen Periode in Mikrosekunden (µs)
  int periodendauer_us = (int)round(1000000.0 / frequenz);

  // die Anzahl der abzuspielenden Perioden, um die gewünschte Dauer zu erreichen
  int anzahl = dauer_ms * frequenz / 1000;
  for (int i = 0; i < anzahl; i++)
  {
    digitalWrite(LAUTSPRECHER_PIN, HIGH);
    delayMicroseconds(periodendauer_us / 2);
    digitalWrite(LAUTSPRECHER_PIN, LOW);
    delayMicroseconds(periodendauer_us / 2);
  }
}

/// @brief Spielt die gesamte Tonfolge der Schulglocke ab
///
/// Die Funktion blockiert so lange, bis die Tonfolge abgeschlossen ist
void spieleTonfolge()
{
  spieleTon(FREQUENZ_G4, 820);
  spieleTon(FREQUENZ_E4, 820);
  spieleTon(FREQUENZ_C4, 820);
  spieleTon(FREQUENZ_G3, 1640);
}

void loop()
{
  spieleTonfolge();

  delay(2000);
}
