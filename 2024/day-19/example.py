import sys
import re
import heapq
from collections import defaultdict, Counter, deque

sys.setrecursionlimit(10**6)


p1 = 0
p2 = 0
D = open(0).read().strip()
words, targets = D.split("\n\n")
words = words.split(", ")

DP = {}


def ways(words, target):
    if target in DP:
        return DP[target]
    ans = 0
    if not target:
        ans = 1
    for word in words:
        if target.startswith(word):
            ans += ways(words, target[len(word) :])
    DP[target] = ans
    return ans


for target in targets.split("\n"):
    target_ways = ways(words, target)
    if target_ways > 0:
        p1 += 1
    p2 += target_ways
