# vault_loader_door.py — Corpus loader door (Synchronet hook, Ollama + MIDI remix)
import random
import json
import os
# Optional: pip install ollama mido  # For AI lore + arpeggio burst
# import ollama  # Uncomment for GPU remix
# import mido  # Uncomment for MIDI thunder

def load_corpus(file_path='corpus.json'):
    """Load corpus JSON (strict fallback to embedded mini if missing)."""
    if os.path.exists(file_path):
        with open(file_path, 'r') as f:
            return json.load(f)
    # Embedded mini-corpus (repo fallback—no file needed)
    return {
        "fish-sign": {"patterns": ["linear-flow", "trade-river"], "meaning": "Aquatic commerce or deity"},
        "unicorn-seal": {"patterns": ["horn-spiral", "mythic-guard"], "meaning": "Sacred protector or elite mark"},
        "jar-symbol": {"patterns": ["storage-loop", "grain-hoard"], "meaning": "Resource tally or ritual vessel"}
        # Scale: Add 400+ from Harappa CSV loader here
    }

def midi_arpeggio_burst(confidence):
    """MIDI tease on high-confidence dawn (retro BBS sound—optional)."""
    if confidence <= 0.7 or 'mido' not in globals():
        return " (MIDI thunder awaits—arpeggio on dawn!)"
    mid = mido.MidiFile()
    track = mido.MidiTrack()
    mid.tracks.append(track)
    notes = [60, 64, 67, 72]  # C major arpeggio for wyrm epic
    for note in notes:
        track.append(mido.Message('note_on', note=note, velocity=100, time=0))
        track.append(mido.Message('note_off', note=note, velocity=0, time=480))
    mid.save(f'vault_arpeggio_{int(confidence*100)}.mid')
    return f" (MIDI burst saved: vault_arpeggio_{int(confidence*100)}.mid—play for cheddar song! 🧀)"

def vault_glyph_loader(glyph="fish-sign", seed=None, ollama_host="http://localhost:11434"):
    """
    Synchronet door core: Load corpus, RNG decode, Ollama remix on high confidence.
    Input: Glyph (BBS user), seed (user ID for repro).
    Output: JSON + MIDI tease.
    """
    if seed:
        random.seed(seed)  # Repro for quest fairness
    corpus = load_corpus('corpus.json')  # Strict load/fallback
    base = corpus.get(glyph, {"patterns": ["unknown-emergent"], "meaning": "Mystery twist from the void"})
    confidence = random.uniform(0.1, 0.8)
    remixed = f"{base['meaning']} + {random.choice(['cosmic alignment', 'trade ledger', 'ritual chant'])}"
    output = {
        "glyph": glyph,
        "decoded_patterns": base['patterns'],
        "remixed_meaning": remixed,
        "confidence": f"{confidence:.2f}",
        "flavor": random.choice([
            "Indus wheel spins—secret decode whispers!",
            "Undeciphered rind cracks: Cheddar for ancient bytes.",
            "Harappan harmony leaps eternal—glyph guards the hoard!"
        ])
    }
    if confidence > 0.7:
        output["dawn"] = "Cheddar hoard revealed! 🧀—Wyrm's riddle: What's the BBS eternal? (Hint: Procedural cheese!)"
        # Ollama remix (optional; GPU thunder)
        # try:
        #     resp = ollama.chat(model='llama3', messages=[{'role': 'user', 'content': f"Remix Indus glyph {glyph}: {remixed} with emergent lore."}])
        #     output["ai_remix"] = resp['message']['content'][:200] + "..."  # Truncate for JSON
        # except:
        #     output["ai_remix"] = "Ollama lore awaits GPU upgrade—emergent patterns suggest river-trade thunder!"
        output["ai_remix"] = "Ollama lore: Emergent patterns suggest Harappan harmony—decipher with curiosity eternal!"
    
    # MIDI burst tease (retro door sound)
    midi_note = midi_arpeggio_burst(confidence)
    
    # Full output: JSON + MIDI note (for BBS text/parse)
    full_output = json.dumps(output, indent=2) + "\n\n" + midi_note
    return full_output

if __name__ == "__main__":
    import sys
    glyph_input = sys.argv[1] if len(sys.argv) > 1 else "unicorn-seal"
    print("=== Synchronet Vault Door: Load the Glyph Corpus! ===")
    print(vault_glyph_loader(glyph_input, seed=42))  # Repro seed for demo
    print("\nQuest Tease: High confidence? Solve the riddle for hoard glory! 🧀🚪")
