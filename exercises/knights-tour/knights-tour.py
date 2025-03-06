import sys
import time

# Setup
knight_moves = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)]
visited = [(0, 0)]
width = int(sys.argv[1]) if len(sys.argv) > 1 else 8
required = int(sys.argv[2]) if len(sys.argv) > 2 else width ** 2

# Find path
def find_path(visited):
    #draw_path(visited)
    if len(visited) >= required:
        return visited
    next_places = [(visited[-1][0] + move[0], visited[-1][1] + move[1]) for move in knight_moves]
    legal_next_places = list(filter(lambda p: p[0] in range(width) and p[1] in range(width) and p not in visited, next_places))
    if len(legal_next_places) == 0:
        return None
    for place in legal_next_places:
        path = find_path(visited + [place])
        if path is not None:
            return path
    return None

# Draw path
def draw_path(visited):
    for x in range(width):
        for y in range(width):
            try:
                pos = visited.index((x, y))
                s = str(pos).rjust(2, " ")
            except ValueError:
                s = " ."
            print(s, end=" ")
        print()
    print(len(visited), "of", width ** 2)
    print()

visited = find_path(visited)
draw_path(visited)
