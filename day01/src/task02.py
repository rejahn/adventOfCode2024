file = open("input")

list = file.read().splitlines()

left = []
right =  []

for l in list: 
    str = l.split("   ")
    left.append(int(str[0]))
    right.append(int(str[1]))

left.sort()
right.sort()

sum = 0
found = 0

for indexLeft,numberLeft in enumerate(left):
    for indexRight, numberRight in enumerate(right):
        
        if numberLeft == numberRight:
            found += 1
        
    sum = sum + numberLeft * found
    found = 0 

print(f"Result is: {sum}")