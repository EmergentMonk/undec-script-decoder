"""
Core decode function for undeciphered scripts.
Wraps the blackbox_indus_decoder from the parent module.
"""
import json
import sys
import os

# Add parent directory to path to import the blackbox decoder
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

try:
    from undec_script_blackbox import blackbox_indus_decoder
except ImportError:
    # Fallback if import fails
    def blackbox_indus_decoder(quest_glyph="base-symbol"):
        return json.dumps({"error": "Decoder not available", "input": quest_glyph})


def decode(input_str):
    """
    Decode undeciphered script input.
    
    Args:
        input_str: The script text or glyph identifier to decode
        
    Returns:
        The decoded output as a string (JSON format)
    """
    return blackbox_indus_decoder(input_str)
