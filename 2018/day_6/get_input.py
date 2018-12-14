from requests import get
import datetime, time

go = False
while not go:
    seconds = (datetime.datetime(2018, 12, 5) - datetime.datetime.now()).seconds
    print("Seconds left: " + str(seconds))

    if seconds > 80000:
        go = True
    elif seconds > 20:
        time.sleep(5)
    else:
        time.sleep(0.1)


lines = get(f'https://adventofcode.com/2018/day/6/input', cookies={'session': '53616c7465645f5f3fd84e203bf8a12b6e02c7b6709a4552faf9b03b9820f658d02d5f6b002a654dcd5c1c2694801312'}).text.split('\n')
if lines[-1] == '':
    lines = lines[:-1]

with open('input.txt', 'w') as f:
    for item in lines:
        f.write("%s\n" % item)