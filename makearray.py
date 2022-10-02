
jump_frames = 120
jump_height = 50
jumps = [None ] * jump_frames
counter = 0
gravity = 9.8
jumpforce = 69973
jumps[0] = jumpforce
for i in range(0, jump_frames):
    if i != 0:
        jumps[i] = jumps[i-1] - i*9.8
normalizer = (max(jumps)/jump_height)
norm = [float(i)/normalizer for i in jumps]
norm = list(map(int, norm))
#norm = norm.reverse()
#print(jumps)

for i in range(0, jump_frames-1):
    norm[i] = norm[i] - norm[i+1]
    
norm.reverse()
print(norm)