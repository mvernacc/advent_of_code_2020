import day12

print('Example: ')
dist = day12.nav_instructions_manhattan_distance("F10\nN3\nF7\nR90\nF11", verbose=True)
assert dist == 25
print('Example OK\n')


with open('input.txt', 'r') as input_file:
    nav_instructions_text = input_file.read()

# Part 1.
dist = day12.nav_instructions_manhattan_distance(nav_instructions_text)
print(f'Part 1: Manhattan distance  = {dist:d}')
