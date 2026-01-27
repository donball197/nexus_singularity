import onnxruntime as ort
import numpy as np

sess = ort.InferenceSession('models/model.onnx')
# Testing common BERT start/stop tokens
test_tokens = [101, 102, 0, 27] 

for t in test_tokens:
    ids = np.array([[t]*4], dtype=np.int64)
    mask = np.ones((1, 4), dtype=np.int64)
    type_ids = np.zeros((1, 4), dtype=np.int64)
    out = sess.run(None, {'input_ids': ids, 'attention_mask': mask, 'token_type_ids': type_ids})
    print(f"Token {t} -> Top Index: {np.argmax(out[0][0])} (Prob: {np.max(out[0][0]):.4f})")
