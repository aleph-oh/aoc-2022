OPP_ROCK = "A"
OPP_PAPER = "B"
OPP_SCISSORS = "C"

MY_ROCK = "X"
MY_PAPER = "Y"
MY_SCISSORS = "Z"

MY_LOSS = "X"
MY_DRAW = "Y"
MY_WIN = "Z"


ROCK = 1
PAPER = 2
SCISSORS = 3


def parse_choice(s: str) -> int:
    if s == OPP_ROCK or s == MY_ROCK:
        return ROCK
    elif s == OPP_PAPER or s == MY_PAPER:
        return PAPER
    elif s == OPP_SCISSORS or s == MY_SCISSORS:
        return SCISSORS
    else:
        raise ValueError


LOSE_SCORE = 0
DRAW_SCORE = 3
WIN_SCORE = 6


def p1_score_round(opp: str, my: str) -> int:
    opp = parse_choice(opp)
    my = parse_choice(my)
    return p1_score_round_parsed(opp, my)


def p1_score_round_parsed(opp: int, my: int) -> int:
    if opp == my:
        return my + DRAW_SCORE
    elif (my - opp) % 3 == 1:
        return my + WIN_SCORE
    else:
        return my + LOSE_SCORE


DRAW = 0
WIN = 1
LOSS = 2


def parse_outcome(s: str) -> int:
    if s == MY_WIN:
        return WIN
    elif s == MY_LOSS:
        return LOSS
    elif s == MY_DRAW:
        return DRAW
    else:
        raise ValueError


def cause_outcome(opp: int, desired: int) -> int:
    if desired == DRAW:
        return opp
    elif desired == WIN:
        return max((opp + 1) % 4, 1)
    else:
        return SCISSORS if opp == ROCK else (opp + 2) % 3


def p2_score_round(opp: str, desired: str) -> int:
    opp_choice = parse_choice(opp)
    desired_out = parse_outcome(desired)
    my_choice = cause_outcome(opp_choice, desired_out)
    assert 1 <= my_choice <= 3, (opp, desired, my_choice)
    return p1_score_round_parsed(opp_choice, my_choice)


if __name__ == '__main__':
    with open("../day2_input.txt") as f:
        p1_sum = 0
        p2_sum = 0
        for line in f:
            first, second = line.strip().split()
            p1_sum += p1_score_round(first, second)
            p2_sum += p2_score_round(first, second)

        print(p1_sum, p2_sum)
