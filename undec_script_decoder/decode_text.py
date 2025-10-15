from .undec import decode

def undec_decode(input_str):
    """
    A simple wrapper for the undec.decode() function.
    """
    return decode(input_str)

if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        input_text = sys.argv[1]
        decoded_text = undec_decode(input_text)
        print(decoded_text)
    else:
        print("Usage: python -m undec_script_decoder.decode_text \"<your_script_text>\"")
