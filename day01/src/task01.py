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
for i,le in enumerate(left): 
    sum = sum + abs(le-right[i])
    
print(f"Result is: {sum}")