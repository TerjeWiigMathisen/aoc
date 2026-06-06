from re import compile
from operator import mul, itemgetter
from functools import reduce
from itertools import tee, filterfalse, chain

value = compile('value (\d+) goes to (bot \d+)')
botcmd = compile('(bot \d+) gives low to ((?:output|bot) \d+) and high to ((?:output|bot) \d+)')

def parseLines(lines, regex):
    return [regex.match(line).groups() for line in lines]

def partition(pred, iterable):
    t1, t2 = tee(iterable)
    return filterfalse(pred, t1), filter(pred, t2)

def defineBot(bot, low, high, bins):
    def dist_a(a):
        def dist_b(b):
            h, l        = max(a, b), min(a, b)
            bins[high]  = bins[high](h)
            bins[low]   = bins[low](l)          
            return (h, l)
        return dist_b
    return dist_a

def botEval(inputs, cmds, bins):
    for bot, low, high in cmds:
        bins[bot] = defineBot(bot, low, high, bins) 

    for val, bot in inputs:
        bins[bot] = bins[bot](int(val))

def getOutputs(bins):
    outputBins = ((k, v) for k, v in bins.items() if k.startswith("output"))
    return [v for k, v in sorted(outputBins, key=lambda x: int(x[0].split(" ")[-1]))]

def day10(input):
    inputs, cmds = partition(lambda s: s.startswith("bot"), input)
    inputs, cmds = parseLines(inputs, value), parseLines(cmds, botcmd)

    bins = {x:lambda y: y for x in chain.from_iterable(cmds)} 
    botEval(inputs, cmds, bins)

    outputs = getOutputs(bins)
    return {v:k for k, v in bins.items()}[(61, 17)], reduce(mul, outputs[:3], 1)

input = open("input10.txt").read()
input = [x.strip() for x in input.split("\n")]
print(day10(input))
