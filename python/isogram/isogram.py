def is_isogram(string):
    table = dict.fromkeys(map(ord, ' -'), None)
    trimmed = string.translate(table).lower()
    return len(trimmed) == len(set(trimmed))
