import re

file1 = open('input.txt', 'r')
lines = file1.readlines()

allData = []

for line in lines:
  number = re.sub(r'[^\d]+', '', line)
  value = int(number)
  allData.append(value)

iter = 0

for num1 in allData:
  for num2 in allData:
    if num1 + num2 > 2020:
      continue

    for num3 in allData:
      iter += 1
      if num1 + num2 + num3 == 2020:
        print('Match Found! %s %s %s -- %s', num1, num2, num3, iter)
        print(num1 * num2 * num3)
        exit
