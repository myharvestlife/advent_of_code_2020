import re

file1 = open('input.txt', 'r')
lines = file1.readlines()

allData = []

for line in lines:
  number = re.sub(r'[^\d]+', '', line)
  value = int(number)
  allData.append(value)

for num1 in allData:
  for num2 in allData:
    if num1 + num2 == 2020:
      print('Match Found! %s %s', num1, num2)
      print(num1 * num2)
