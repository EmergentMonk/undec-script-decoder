# 8d_hat_quest.py — Octonion hat door (Synchronet hook)
import random, mido, json, math
from golden_multi_trit import GoldenMultiTritALU
from golden_ollama_bridge import ollama_lore
alu = GoldenMultiTritALU()
phi = (1 + math.sqrt(5)) / 2
hats = ["White", "Red", "Black", "Yellow", "Green", "Blue"]
pmi_labels = ["Plus", "Minus", "Interesting"]
mock_ideals = [[[1,-1,0,1,0,0,1,-1] for _ in range(3)], [[-1,0,1,-1,0,1,-1,0] for _ in range(3)]]  # 8D quark/lepton tease
def octonion_hat_quest(seed="E8 Triality"):
    hat = random.choice(hats)
    knight_ideal = random.choice(mock_ideals)
    dragon_ideal = random.choice(mock_ideals)
    clash = [[(k + d) % 3 - 1 for k, d in zip(row_k, row_d)] for row_k, row_d in zip(knight_ideal, dragon_ideal)]
    trace = sum(sum(row) for row in clash) / 8  # 8D normalize
    gen_leap = trace * phi
    pmi = {}
    for label in pmi_labels:
        if label == "Plus":
            pmi[label] = f"Golden gen leap {gen_leap:.3f} eternal!"
        elif label == "Minus":
            pmi[label] = "Swiss chiral leak!"
        elif label == "Interesting":
            pmi[label] = "Blue triality twist!"
    lore = ollama_lore(gen_leap, f"{hat} Hat {seed}")
    mid = mido.MidiFile()
    track = mido.MidiTrack()
    mid.tracks.append(track)
    notes = [60, 64, 67, 71, 75]  # 8D arpeggio
    for note in notes:
        track.append(mido.Message('note_on', note=note, velocity=100, time=0))
        track.append(mido.Message('note_off', note=note, velocity=0, time=240))
    mid.save('8d_hat_arpeggio.mid')
    return json.dumps({"hat": hat, "PMI": pmi, "gen_leap": gen_leap, "lore": lore, "seed": seed})
if __name__ == "__main__":
    print(octonion_hat_quest())