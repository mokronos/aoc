data = open("data11.txt", "r").read()

def solve(data):
    data = data.split("\n\n")
    monkeys = []
    for monkey in data:
        monkeys.append(parse(monkey))

    for _ in range(20):
        monkeys = round(monkeys)

    max = [0,0]
    for monkey in monkeys:
        if monkey["inspected"] > max[0]:
            max[0] = monkey["inspected"]
        max.sort()
    print(max[0]*max[1])
def round(monkeys):

    for number, monkey in enumerate(monkeys):

        for old in monkey["items"]:
            new = eval(f"{old}{monkey['op']}{monkey['op_val']}")
            new = new//3
            if new % monkey["test"] == 0:
                monkeys[monkey["true"]]["items"].append(new)
            else:
                monkeys[monkey["false"]]["items"].append(new)
        
            monkey["inspected"] += 1
        monkey["items"] = []

    return monkeys

def parse(monkey):
    
    lines = monkey.splitlines()
    result = {}
    result["name"] = lines[0][-2]
    result["items"] = [int(x) for x in lines[1].split("s: ")[-1].split(", ")]
    result["op"] = lines[2].split(" ")[-2]
    result["op_val"] = lines[2].split(" ")[-1]
    result["test"] = int(lines[3].split(" ")[-1])
    result["true"] = int(lines[4].split(" ")[-1])
    result["false"] = int(lines[5].split(" ")[-1])
    result["inspected"] = 0

    return result


solve(data)

