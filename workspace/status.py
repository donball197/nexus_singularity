import datetime
import os

def main():
    now = datetime.datetime.now()
    print("Current date and time:", now)
    print("\nFiles in workspace:")
    for filename in os.listdir("./workspace/"):
        print(filename)

if __name__ == "__main__":
    main()