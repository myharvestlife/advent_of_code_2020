import re

file1 = open('input1.txt', 'r')
lines = file1.readlines()

allData = []

pattern = re.compile(r"^(\d+)-(\d+) ([a-z]{1}): ([a-z]+)$")

num_words = 0

for line in lines:
  result = pattern.match(line)
  start = int(result.group(1))
  end = int(result.group(2))
  letter = result.group(3)
  word = result.group(4)

  num_chars = 0
  for idx in range(0, len(word)):
    char = word[idx]
    print("char: % s == letter: %s" %(char, letter))
    if char == letter:
      num_chars += 1

  print("start: % 2d - end: % 2d - letter: % s - word: % s - num chars: % d" %(start, end, letter, word, num_chars))

  if start <= num_chars and num_chars <= end:
    num_words += 1
    print('Word Matches')


print("Number of Words that Match: % d" %(num_words))
