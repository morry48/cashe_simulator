import subprocess
import random
import argparse

file_name = input("trace file name: ")

s = 4
E = 1
b = 4

ref = subprocess.check_output('./sim-ref -v -s {} -E {} -b {} -t traces/{}'.format(s, E, b, file_name), shell=True).decode('ascii')
# if you need to compile every time, use cargo run
# mine = subprocess.check_output('cargo run --manifest-path sim/Cargo.toml -- -v -s {} -E {} -b {} -t traces/{}'.format(s, E, b, file_name), shell=True).decode('ascii')
mine = subprocess.check_output('./sim-exe -v -s {} -E {} -b {} -t traces/{}'.format(s, E, b, file_name), shell=True).decode('ascii')


if ref == mine:
    print(" ==> Passed! \n")

else:
    print("failed following ")
    print("params s={}, E={}, b={}".format(s,E,b))
    print("sim-ref: ")
    print(str(ref).strip())
    print("your code: ")
    print(str(mine).strip())
    quit()

