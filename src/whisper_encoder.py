import time

def send_whisper(message):
    print(f">> [FORGE] ENCODING: {message}")
    binary_msg = ''.join(format(ord(i), '08b') for i in message)
    for bit in binary_msg:
        if bit == '1':
            start = time.perf_counter()
            while time.perf_counter() - start < 0.1:
                _ = 2 ** 100 
        else:
            time.sleep(0.1)
    print("\n>> [FORGE] TRANSMISSION COMPLETE.")

if __name__ == "__main__":
    msg = input("Enter message: ")
    send_whisper(msg)
