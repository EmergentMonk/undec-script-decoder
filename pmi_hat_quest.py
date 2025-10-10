# pmi_hat_quest.py — Gamified PMI & Six Hats (Synchronet/TUI)
import random, mido, json

hats = ["White", "Red", "Black", "Yellow", "Green", "Blue"]
pmi_labels = ["Plus", "Minus", "Interesting"]
def pmi_quest(seed="Quantum Cheese"):
    hat = random.choice(hats)
    pmi = {label: f"{label} for {seed}: ..." for label in pmi_labels}
    # MIDI: C60 for Plus, C48 for Minus, C67 for Interesting
    mid = mido.MidiFile()
    track = mido.MidiTrack()
    mid.tracks.append(track)
    notes = [60, 48, 67]
    for note in notes:
        track.append(mido.Message('note_on', note=note, velocity=100, time=0))
        track.append(mido.Message('note_off', note=note, velocity=0, time=240))
    mid.save('pmi_hat_arpeggio.mid')
    return json.dumps({"hat": hat, "PMI": pmi, "seed": seed})

if __name__ == "__main__":
    print(pmi_quest())