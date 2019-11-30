"""A simple checker for types of functions in tweet.py."""

import unittest
import checker_generic
import tweet


class CheckTest(unittest.TestCase):
    """Sanity checker for assignment functions."""

    def testIsValidTweet(self):
        """Function is_valid_tweet."""

        self._check(tweet.is_valid_tweet, ['Hello!'], bool)
        self.assertTrue(tweet.is_valid_tweet("hello"))
        self.assertFalse(tweet.is_valid_tweet(2 * 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'))

    def testCompareTweetLengths(self):
        """ Function compare_tweet_lengths."""

        self._check(tweet.compare_tweet_lengths, ['abc', 'ab'], int)
        self.assertEqual(tweet.compare_tweet_lengths('abc', 'ab'), 1)
        self.assertEqual(tweet.compare_tweet_lengths('ab', 'abc'), -1)
        self.assertEqual(tweet.compare_tweet_lengths('abc', 'abc'), 0)

    def testAddHashtag(self):
        """Function add_hashtag."""

        self._check(tweet.add_hashtag, ['Hello', 'greeting'], str)
        self.assertEqual(tweet.add_hashtag('I like', 'cs108'), 'I like #cs108')
        self.assertEqual(tweet.add_hashtag(2 * 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'csc108'),
                         2 * 'ABCDEFGHIJKLMNOPQRSTUVWXYZ')

    def testContainsHashtag(self):
        """Function contains_hashtag."""

        self._check(tweet.contains_hashtag, ['#hash', 'hash'], bool)
        self.assertTrue(tweet.contains_hashtag('I like #csc108', 'csc108'))
        self.assertFalse(tweet.contains_hashtag('I like #csc108', 'csc'))
        self.assertTrue(tweet.contains_hashtag('I like #csc108, #mat137, and #phl101', 'mat137'))

    def testIsMentioned(self):
        """Function is_mentioned."""

        self._check(tweet.is_mentioned, ['@mention', 'mention'], bool)
        self.assertTrue(tweet.is_mentioned('no@spaces#whatsoever?!', 'spaces'))
        self.assertFalse(tweet.is_mentioned('no@spaces#whatsoever?!', 'whatsoever'))


    def testAddMentionExclusive(self):
        """"Function add_mention_exclusive."""
        self._check(tweet.add_mention_exclusive, [
                    'Hello, World', 'World'], str)
        self.assertEqual(tweet.add_mention_exclusive('no@spaces#whatsoever?!', 'spaces'),
                         'no@spaces#whatsoever?!')
        self.assertEqual(tweet.add_mention_exclusive('Go Raptors!', 'Raptors'),
                         'Go Raptors! @Raptors')

    def testNumTweetsRequired(self):
        """Function num_tweets_required."""
        self._check(tweet.num_tweets_required, ['hello'], int)
        text = 'Standing ovation as Setsuko Thurlow is awarded a' \
               ' Doctor of Laws degree, honoris causa, by the ' \
               'University of Toronto @UofT for her tireless nuclear' \
               ' disarmament work and contributions to the Treaty on ' \
               'the Prohibition of Nuclear Weapons with @nuclearban ICAN'
        self.assertEqual(tweet.num_tweets_required(text), 6)
        self.assertEqual(tweet.num_tweets_required('hello'), 1)

    def testGetNthTweet(self):
        """Function get_nth_tweet."""
        text = 'Standing ovation as Setsuko Thurlow is awarded a' \
               ' Doctor of Laws degree, honoris causa, by the ' \
               'University of Toronto @UofT for her tireless nuclear' \
               ' disarmament work and contributions to the Treaty on ' \
               'the Prohibition of Nuclear Weapons with @nuclearban ICAN'
        self._check(tweet.get_nth_tweet, ['abcdef', 1], str)
        self.assertEqual(tweet.get_nth_tweet(text, 0), 'Standing ovation '
                                                       'as Setsuko Thurlow is awarded a D')
        self.assertEqual(tweet.get_nth_tweet(text, 5), ' ICAN')
        self.assertEqual(tweet.get_nth_tweet(text, 6), '')

    def testCheckConstants(self):
        """Checking constants"""
        print('\nChecking that constants refer to their original values')
        self.assertEqual(tweet.MAX_TWEET_LENGTH, 50,
                         'Set MAX_TWEET_LENGTH to its original value: 50')
        self.assertEqual(tweet.HASHTAG_SYMBOL, '#',
                         'Set HASHTAG_SYMBOL to its original value: \'#\'')
        self.assertEqual(tweet.MENTION_SYMBOL, '@',
                         'Set MENTION_SYMBOL to its original value: \'@\'')
        self.assertEqual(tweet.UNDERSCORE, '_',
                         'Set UNDERSCORE to its original value: \'_\'')
        self.assertEqual(tweet.SPACE, ' ',
                         'Set SPACE to its original value: \' \'')
        print('  check complete')

    def _check(self, func: callable, args: list,
               ret_type: type) -> None:
        """Check that func called with arguments args returns a value of type
        ret_type. Display the progress and the result of the check.

        """

        print('\nChecking {}...'.format(func.__name__))
        result = checker_generic.check(func, args, ret_type)
        self.assertTrue(result[0], result[1])
        print('  check complete')


TARGET_LEN = 79
print(''.center(TARGET_LEN, "="))
print(' Start: checking coding style '.center(TARGET_LEN, "="))
checker_generic.run_pyta('tweet.py', 'pyta/a1_pyta.txt')
print(' End checking coding style '.center(TARGET_LEN, "="))

print(' Start: checking type contracts '.center(TARGET_LEN, "="))
unittest.main(exit=False)
print(' End checking type contracts '.center(TARGET_LEN, "="))

print('\nScroll up to see ALL RESULTS:')
print('  - checking coding style')
print('  - checking type contract\n')
