import random

words = []

with open("src/words.txt") as f:
    for line in f.readlines():
        words.append(line.strip())

def get_info(word, correct_word):
    used = [False, False, False, False, False]
    for i in range(5):
        if word[i] == correct_word[i]:
            used[i] = True
    ret = ""
    for i in range(5):
        if word[i] == correct_word[i]:
            ret += "0"
        else:
            found = False
            for j in range(5):
                if word[i] == correct_word[j] and not used[j]:
                    found = True
                    used[j] = True
            if found:
                ret += "1"
            else:
                ret += "2"
    return ret

correct_word = random.choice(words)
while True:
    guess = input("Guess: ")
    print(get_info(guess, correct_word))