//! Dieses Modul beinhaltet alles, was man für die Aufgabe der Töne über den angeschlossenen
//! Lautsprecher benötigt.

use arduino_hal::{
    delay_us,
    port::{mode::Output, Pin, PinOps},
};

/// Der Lautsprecher zum Abspielen der Tonfolge der Schulglocke.
pub(crate) struct Lautsprecher<PinLinks> {
    /// Der Pin, an welchem der linke Lautsprecher angeschlossen ist.
    pin_links: Pin<Output, PinLinks>,
}

impl<PinLinks: PinOps> Lautsprecher<PinLinks> {
    /// Erstellt ein neues Lautsprecher-Objekt.
    ///
    /// - `pin` der I/O-Pin, an welchem der linke Lautsprecher angeschlossen ist.
    pub fn neu(pin: Pin<Output, PinLinks>) -> Self {
        Self { pin_links: pin }
    }

    /// Brief Spielt die gesamte Tonfolge der Schulglocke ab.
    ///
    /// Die Funktion blockiert so lange, bis die Tonfolge abgeschlossen ist
    pub fn spiele_tonfolge(&mut self) {
        self.spiele_ton(Note::G.frequenz(4), 820);
        self.spiele_ton(Note::E.frequenz(4), 820);
        self.spiele_ton(Note::C.frequenz(4), 820);
        self.spiele_ton(Note::G.frequenz(3), 1640);
    }

    /// Spielt einen Ton als Rechteckwelle auf dem Lautsprecher ab.
    ///
    /// Die Funktion blockiert so lange, bis der Ton abgeschlossen ist.
    ///
    /// - `frequenz` die Tonfrequenz in Hertz (Hz)
    /// - `dauer_ms` die Dauer des Tons in Millisekunden
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        reason = "die gerundeten Werte sind genau genug und das Vorzeichen ist immer positiv"
    )]
    fn spiele_ton(&mut self, frequenz: f32, dauer_ms: u32) {
        assert!(frequenz > 0.0, "Die Frequenz muss ein positiver Wert sein");

        // die Dauer einer einzelnen Periode in Mikrosekunden (µs)
        let periodendauer_us = ((1_000_000.0 / frequenz) + 0.5) as u32;

        // die Anzahl der abzuspielenden Perioden, um die gewünschte Dauer zu erreichen
        let anzahl = ((dauer_ms as f32 * frequenz / 1000.0) + 0.5) as u32;

        // Rechteckwelle mit der richtigen Frequenz und Dauer auf dem Lautsprecher-Pin ausgeben
        for _ in 0..anzahl {
            self.pin_links.set_high();
            delay_us(periodendauer_us / 2);
            self.pin_links.set_low();
            delay_us(periodendauer_us / 2);
        }
    }
}

/// Die verfügbaren Töne, die sich abspielen lassen
#[derive(Clone, Copy)]
#[expect(unused, reason = "Nicht jede Note wird immer benutzt")]
enum Note {
    C,
    Cis,
    D,
    Dis,
    E,
    F,
    Fis,
    G,
    Gis,
    A,
    Ais,
    H,
}

impl Note {
    /// Berechnet die Frequenz der Note für die angegebene Oktave in Hertz.
    ///
    /// Die eingestrichene Oktave hat die Nummer 4. Siehe
    /// <https://de.wikipedia.org/wiki/Oktave#Untergliederung_des_Tonraumes_in_Oktavr%C3%A4ume>
    ///
    /// Die Frequenzen stammen aus der gleichstufigen Stimmung:
    /// <https://de.wikipedia.org/wiki/Gleichstufige_Stimmung#Frequenzberechnung>
    const fn frequenz(self, oktave: u8) -> f32 {
        // Die Frequenzen der Note in der eingestrichenen Oktave (4).
        // Die Frequenzen dieser Oktave sind leicht zu recherchieren
        // Die Frequenzen ließen auch leicht berechnen. Da der Arduino aber keine
        // Fließkommaberechnungen durchführen kann, arbeiten wir mit vorberechneten Werten.
        let notenfrequenz_oktave_4 = match self {
            Note::C => 261.6,
            Note::Cis => 277.2,
            Note::D => 293.7,
            Note::Dis => 311.1,
            Note::E => 329.6,
            Note::F => 349.2,
            Note::Fis => 370.0,
            Note::G => 392.0,
            Note::Gis => 415.3,
            Note::A => 440.0, // Kammerton A4
            Note::Ais => 466.2,
            Note::H => 493.9,
        };

        // die Frequenz der Note in die Subkontra-Oktave (0) umrechnen
        // Mit dem Ergebnis kann man alle anderen Oktaven durch eine einfach Multiplikation mit
        // einer Zweierpotenz erreichen.
        let notenfrequenz_oktave_0 = notenfrequenz_oktave_4 / (1_u16 << 4) as f32;

        // die Notenfrequenz auf die gewünschte Oktave anheben
        notenfrequenz_oktave_0 * (1_u16 << oktave) as f32
    }
}
