from common import main
from day05.input import Database, parse_input



def solve_part1(db: Database):
    fresh_ingredients = [
        ingredient
        for ingredient in db.available_ingredients
        if any(start <= ingredient <= end for start, end in db.fresh_ranges)
    ]
    return len(fresh_ingredients)

if __name__ == "__main__":
    main(parse_input, solve_part1)
