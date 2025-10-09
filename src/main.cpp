/**
 * Blink
 *
 * Turns on an LED on for one second,
 * then off for one second, repeatedly.
 */
#include "Arduino.h"

// Set LED_BUILTIN if it is not defined by Arduino framework
// #define LED_BUILTIN 13

void setup()
{
  // initialize LED digital pin as an output.
  pinMode(PIND6, OUTPUT);
}

const int NOTE_C3 = (int)(1000000.0 / (440.0 * pow(2.0, -21.0 / 12.0)));
const int NOTE_D3 = (int)(1000000.0 / (440.0 * pow(2.0, -19.0 / 12.0)));
const int NOTE_E3 = (int)(1000000.0 / (440.0 * pow(2.0, -17.0 / 12.0)));
const int NOTE_F3 = (int)(1000000.0 / (440.0 * pow(2.0, -16.0 / 12.0)));
const int NOTE_G3 = (int)(1000000.0 / (440.0 * pow(2.0, -14.0 / 12.0)));
const int NOTE_A3 = (int)(1000000.0 / (440.0 * pow(2.0, -12.0 / 12.0)));
const int NOTE_H3 = (int)(1000000.0 / (440.0 * pow(2.0, -10.0 / 12.0)));
const int NOTE_C4 = (int)(1000000.0 / (440.0 * pow(2.0, -9.0 / 12.0)));
const int NOTE_D4 = (int)(1000000.0 / (440.0 * pow(2.0, -7.0 / 12.0)));
const int NOTE_E4 = (int)(1000000.0 / (440.0 * pow(2.0, -5.0 / 12.0)));
const int NOTE_F4 = (int)(1000000.0 / (440.0 * pow(2.0, -4.0 / 12.0)));
const int NOTE_G4 = (int)(1000000.0 / (440.0 * pow(2.0, -2.0 / 12.0)));
const int NOTE_A4 = (int)(1000000.0 / (440.0 * pow(2.0, 0.0 / 12.0)));
const int NOTE_H4 = (int)(1000000.0 / (440.0 * pow(2.0, 2.0 / 12.0)));
const int NOTE_C5 = (int)(1000000.0 / (440.0 * pow(2.0, 3.0 / 12.0)));

void spieleG4Fuer815ms()
{
  int anzahl = 815000 / NOTE_G4;
  for (int i = 0; i < anzahl; i++)
  {
    // Spiele Note
    const int NOTE = NOTE_G4 / 2;

    digitalWrite(PIND6, HIGH);
    delayMicroseconds(NOTE);
    digitalWrite(PIND6, LOW);
    delayMicroseconds(NOTE);
  }
}
void spieleE4Fuer815ms()
{
  int anzahl = 815000 / NOTE_E4;
  for (int i = 0; i < anzahl; i++)
  {
    // Spiele Note
    const int NOTE = NOTE_E4 / 2;

    digitalWrite(PIND6, HIGH);
    delayMicroseconds(NOTE);
    digitalWrite(PIND6, LOW);
    delayMicroseconds(NOTE);
  }
}
void spieleC4Fuer815ms()
{
  int anzahl = 815000 / NOTE_C4;
  for (int i = 0; i < anzahl; i++)
  {
    // Spiele Note
    const int NOTE = NOTE_C4 / 2;

    digitalWrite(PIND6, HIGH);
    delayMicroseconds(NOTE);
    digitalWrite(PIND6, LOW);
    delayMicroseconds(NOTE);
  }
}
void spieleG3Fuer815ms()
{
  int anzahl = 2 * 815000 / NOTE_G3;
  for (int i = 0; i < anzahl; i++)
  {
    // Spiele Note
    const int NOTE = NOTE_G3 / 2;

    digitalWrite(PIND6, HIGH);
    delayMicroseconds(NOTE);
    digitalWrite(PIND6, LOW);
    delayMicroseconds(NOTE);
  }
}

void loop()
{
  // Spiele G4 für ca. 815ms
  spieleG4Fuer815ms();

  // Spiele E4 für ca. 794ms
  spieleE4Fuer815ms();
  // Spiele C4 für ca. 841ms
  spieleC4Fuer815ms();
  // Spiele G3 für ca. 1.633s
  spieleG3Fuer815ms();

  delay(2000);
}
