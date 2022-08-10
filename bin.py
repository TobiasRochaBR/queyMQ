baite = [0,0,0,0,1,1,0,0]

value = 0

for i in range(len(baite)-1,-1,-1):
    index = len(baite)-i-1
    #print(index,i)
    value += baite[index]*2**i


print("Byte size:",len(baite),"(bits) with",value,"value")
