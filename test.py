import subprocess
import random
import argparse

file_name = input("trace file name: ")

# you can change test times
test_num = 5

for i in range(test_num):
    s = random.randint(1,10)
    E = random.randint(1,10)
    b = random.randint(1,10)

    ref = subprocess.check_output('./sim-ref -s {} -E {} -b {} -t traces/{}'.format(s, E, b, file_name), shell=True).decode('ascii')
#     if you need to compile every time, use cargo run
#     mine = subprocess.check_output('cargo run --manifest-path sim/Cargo.toml -- -s {} -E {} -b {} -t traces/{}'.format(s, E, b, file_name), shell=True).decode('ascii')
    mine = subprocess.check_output('./sim-exe -s {} -E {} -b {} -t traces/{}'.format(s, E, b, file_name), shell=True).decode('ascii')


    if ref == mine:
        print(" ==> Passed! \n")

    else:
        print("failed following ")
        print("params s={}, E={}, b={}".format(s,E,b))
        print(str(ref).strip())
        print(str(mine).strip())
        quit()

