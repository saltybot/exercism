def recite(start_verse, end_verse):
    days = [
        'first',
        'second',
        'third',
        'fourth',
        'fifth',
        'sixth',
        'seventh',
        'eighth',
        'ninth',
        'tenth',
        'eleventh',
        'twelfth'
    ]
    gifts = [
        'a Partridge in a Pear Tree',
        'two Turtle Doves',
        'three French Hens',
        'four Calling Birds',
        'five Gold Rings',
        'six Geese-a-Laying',
        'seven Swans-a-Swimming',
        'eight Maids-a-Milking',
        'nine Ladies Dancing',
        'ten Lords-a-Leaping',
        'eleven Pipers Piping',
        'twelve Drummers Drumming'
    ]

    verses = []
    verse = ''

    for i in range(start_verse - 1, end_verse):
        if i == 0:
            verse += 'On the first day of Christmas my true love gave to me: a Partridge in a Pear Tree.'
        else:
            verse += 'On the ' + days[i] + ' day of Christmas my true love gave to me: '
            for j in range(i, 0, -1):
                verse += gifts[j] + ', '
            verse += 'and a Partridge in a Pear Tree.'
        verses.append(verse)
        verse = ''

    return verses
