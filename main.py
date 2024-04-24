import random

words = []

with open("words.txt", "r") as f:
    lines = f.readlines()
    for line in lines:
        words.append(line.strip())

def info(word, guess):
    hints = ""
    for i in range(len(guess)):
        if word[i] == guess[i]:
            hints += "0"
        elif guess[i] in word:
            hints += "1"
        else:
            hints += "2"
    return hints

word = random.choice(words)
while True:
    guess = input("Enter a word: ").strip()
    print(info(word, guess))