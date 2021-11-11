import numpy as np

x = np.loadtxt("data4.txt", dtype = str, comments = "$", delimiter = "\n\n")


y = open("data4.txt", "r").read()


def solve(x):

    count = 0
    y_split = y.split("\n\n")

    for i in range(len(y_split)):
        z = y_split[i].replace("\n", " ")
        z = z.split(" ")

        if "" in z:

            z.remove("")

        entries_valid = 0
        cid_missing = 1

        for j in z:

            if (j[0:3] == "byr") and (1920 <= int(j[4:]) <= 2002) and (len(j[4:])==4):
                entries_valid += 1

            if (j[0:3] == "iyr") and (2010 <= int(j[4:]) <= 2020) and (len(j[4:])==4):
                entries_valid += 1

            if (j[0:3] == "eyr") and (2020 <= int(j[4:]) <= 2030) and (len(j[4:])==4):
                entries_valid += 1

            if j[0:3] == "hgt":
                if (j[-2:] == "cm") and (150 <= int(j[4:-2]) <= 193):
                    entries_valid += 1

                if (j[-2:] == "in") and (59 <= int(j[4:-2]) <= 76):
                    entries_valid += 1

            if j[0:3] == "hcl":

                v = ["a","b","c","d","e","f","0","1","2","3","4","5","6","7","8","9"]

                if j[4] == "#":
                    if len(j[5:]) == 6:
                        code = j[5:]

                        c = 1

                        for k in code:

                            if k not in v:
                                c = 0


                        entries_valid += c

            if j[0:3] == "ecl":

                v = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]

                if j[4:] in v: 
                    entries_valid += 1
            if (j[0:3] == "pid") and (len(j[4:]) == 9):

                v = ["0","1","2","3","4","5","6","7","8","9"]

                c = 1

                for k in j[4:]:

                    if k not in v:

                        c = 0
                entries_valid += c
        

        if entries_valid == 7:
            count +=1
    
    print(count)

solve(y)
