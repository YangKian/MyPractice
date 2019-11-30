"""Assignment 1.
"""

import math

# Maximum number of characters in a valid tweet.
MAX_TWEET_LENGTH = 50

# The first character in a hashtag.
HASHTAG_SYMBOL = '#'

# The first character in a mention.
MENTION_SYMBOL = '@'

# Underscore is the only non-alphanumeric character that can be part
# of a word (or username) in a tweet.
UNDERSCORE = '_'

SPACE = ' '


def is_valid_tweet(text: str) -> bool:
    """Return True if and only if text contains between 1 and
    MAX_TWEET_LENGTH characters (inclusive).

    >>> is_valid_tweet('Hello Twitter!')
    True
    >>> is_valid_tweet('')
    False
    >>> is_valid_tweet(2 * 'ABCDEFGHIJKLMNOPQRSTUVWXYZ')
    False

    """

    length = len(text)
    return bool(0 < length <= MAX_TWEET_LENGTH)


# Now define the other functions described in the handout.
def compare_tweet_lengths(text1: str, text2: str) -> int:
    """Compare the lengths of text1 and text2, if text1 is longer than text2,
    return 1, if text1 is shorter than text2, return -1 and if two texts have
    the same length, return 0.

    >>> compare_tweet_lengths("hello", "hi")
    1
    >>> compare_tweet_lengths("hi", "hello")
    -1
    >>> compare_tweet_lengths("hello", "world")
    0

    """

    length1 = len(text1)
    length2 = len(text2)
    if length1 > length2:
        return 1
    elif length1 < length2:
        return -1
    else:
        return 0


def add_hashtag(text: str, word: str) -> str:
    """Return the text append with a space, a hash symbol and the word, if the
    text is a valid tweet. Otherwise return the original text.

    >>> add_hashtag("I learned", "course108")
    'I learned #course108'
    >>> add_hashtag("Standing ovation as Tom is awarded a Doctor of Laws degree", "doctor")
    'Standing ovation as Tom is awarded a Doctor of Laws degree'

    """

    result = text + SPACE + HASHTAG_SYMBOL + word
    if is_valid_tweet(result):
        return result
    else:
        return text


def check_tweet_with_notation(text: str, word: str, notation: str) -> bool:
    """A helper function, return true if the word with notation as prefixion
    is in the text, otherwise return false

    >>> check_tweet_with_notation("University of Toronto @UofT", "UofT", MENTION_SYMBOL)
    True
    >>> check_tweet_with_notation("University of Toronto @UofT", "UofT", HASHTAG_SYMBOL)
    False

    """

    str_cleaned = clean(text)
    str_with_notation = notation + word
    for item in str_cleaned.split(" "):
        if item == str_with_notation:
            return True
    return False


def contains_hashtag(text: str, word: str) -> bool:
    """Return true if the word with a hashtag as prefixion is in the text,
    otherwise return false

    >>> contains_hashtag("the course is #cs108", "cs108")
    True
    >>> contains_hashtag("the course is #cs108", "cs")
    False

    """

    return check_tweet_with_notation(text, word, HASHTAG_SYMBOL)


def is_mentioned(text: str, word: str) -> bool:
    """Return true if the word with a mention symbol as prefixion is in
    the text, otherwise return false

    >>> is_mentioned("Go @Python!", "Python")
    True
    >>> is_mentioned("Go @Python", "Java")
    False

    """

    return check_tweet_with_notation(text, word, MENTION_SYMBOL)


def add_mention_exclusive(text: str, word: str) -> str:
    """Appending a word with a mention symbol as prefixion to the text, if the
    text is a valid tweet, return the modified text,otherwise return the
    original text.

    >>> add_mention_exclusive("Go @Python!", "Python")
    'Go @Python!'
    >>> add_mention_exclusive("Go @Python!", "Java")
    'Go @Python! @Java'

    """

    if is_valid_tweet(text) and is_mentioned(text, word):
        return text
    else:
        tweet_temp = text + SPACE + MENTION_SYMBOL + word
        if is_valid_tweet(tweet_temp):
            return tweet_temp
        else:
            return text


def num_tweets_required(message: str) -> int:
    """Return the minimum number of tweets that would be required to communicate
    the entire message.

    >>> num_tweets_required("hello")
    1
    >>> num_tweets_required("Standing ovation as Setsuko is awarded a Doctor of Laws degree")
    2

    """

    return math.ceil(len(message) / MAX_TWEET_LENGTH)


def get_nth_tweet(message: str, idx: int) -> str:
    """Return the ith valid tweet in the sequence of message. If n is too large
    and there is no index n tweet in the sequence, return an empty string.

    >>> get_nth_tweet("Standing ovation as Setsuko Thurlow is awarded a Doctor of Laws degree", 0)
    'Standing ovation as Setsuko Thurlow is awarded a D'
    >>> get_nth_tweet("Standing ovation as Setsuko Thurlow is awarded a Doctor of Laws degree", 1)
    'octor of Laws degree'
    >>> get_nth_tweet("Standing ovation as Setsuko Thurlow is awarded a Doctor of Laws degree", 2)
    ''

    """

    split_num = num_tweets_required(message)
    if split_num < idx:
        return ""
    if idx == split_num - 1:
        return message[MAX_TWEET_LENGTH * idx:]
    else:
        return message[MAX_TWEET_LENGTH * idx:MAX_TWEET_LENGTH * (idx + 1)]


# A helper function.  Do not modify this function, but you are welcome
# to call it.
def clean(text: str) -> str:
    """Return text with every non-alphanumeric character, except for
    HASHTAG_SYMBOL, MENTION_SYMBOL, and UNDERSCORE, replaced with a
    SPACE, and each HASHTAG_SYMBOL replaced with a SPACE followed by
    the HASHTAG_SYMBOL, and each MENTION_SYMBOL replaced with a SPACE
    followed by a MENTION_SYMBOL.

    >>> clean('A! lot,of punctuation?!!')
    'A  lot of punctuation   '
    >>> clean('With#hash#tags? and@mentions?in#twe_et #end')
    'With #hash #tags  and @mentions in #twe_et  #end'
    """

    clean_str = ''
    for char in text:
        if char.isalnum() or char == UNDERSCORE:
            clean_str = clean_str + char
        elif char == HASHTAG_SYMBOL or char == MENTION_SYMBOL:
            clean_str = clean_str + SPACE + char
        else:
            clean_str = clean_str + SPACE
    return clean_str