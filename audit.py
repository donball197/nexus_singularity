import onnxruntime as ort
import numpy as np

def softmax(x):
    e_x = np.exp(x - np.max(x))
    return e_x / e_x.sum()

# Load the seated quantized model
sess = ort.InferenceSession('models/model.onnx')

# Five patterns to stress-test the attractor
patterns = {
    "All Start (101)": [101] * 128,
    "All SEP (102)": [102] * 128,
    "Alternating": [101, 102] * 64,
    "All Padding (0)": [0] * 128,
    "Descending": list(range(127, -1, -1))
}

print(f"{'Pattern':<20} | {'Top Index':<10} | {'Confidence':<12} | {'Logit'}")
print("-" * 60)

for name, tokens in patterns.items():
    ids = np.array([tokens], dtype=np.int64)
    mask = np.ones((1, 128), dtype=np.int64)
    type_ids = np.zeros((1, 128), dtype=np.int64)
    
    out = sess.run(None, {
        'input_ids': ids, 
        'attention_mask': mask, 
        'token_type_ids': type_ids
    })
    
    logits = out[0][0]
    probs = softmax(logits)
    idx = np.argmax(probs)
    
    print(f"{name:<20} | {idx:<10} | {np.max(probs):.4%} | {logits[idx]:.4f}")
