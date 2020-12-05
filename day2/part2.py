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

  first_match = word[start - 1] == letter
  second_match = word[end - 1] == letter

  print("start: % 2d - end: % 2d - letter: % s - word: % s - first match: % s - second match: % s" %(start, end, letter, word, first_match, second_match))

  if first_match ^ second_match:
    num_words += 1
    print('Only One Match Is True!')


print("Number of Words that Match: % d" %(num_words))
