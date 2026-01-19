import math
from collections import Counter

def calculate_shannon_entropy(data):
    """
    Calculates the Shannon entropy of the given data (string or list of items).
    """
    if not data:
        return 0.0

    # Count frequencies
    counts = Counter(data)
    total_items = len(data)
    entropy = 0.0

    # Calculate probabilities and sum -p * log2(p)
    for count in counts.values():
        probability = count / total_items
        if probability > 0:
            entropy -= probability * math.log2(probability)

    return entropy

if __name__ == '__main__':
    test_string_low = "AAAAABBBBB"
    test_string_high = "ABCDEFGHJI"
    test_string_uniform = "0101010101"

    print(f"Entropy of '{test_string_low}': {calculate_shannon_entropy(test_string_low):.4f}")
    print(f"Entropy of '{test_string_high}': {calculate_shannon_entropy(test_string_high):.4f}")
    print(f"Entropy of '{test_string_uniform}': {calculate_shannon_entropy(test_string_uniform):.4f}")