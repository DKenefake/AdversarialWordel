# AdversarialWordle
This is a solution method for the word game, Wordle, where the computer is acting as your adversary, also works on normal Wordle (for every possible correct answer).


## Best opening word?

The method we are using, is a min-max type approach where we want to minimize the largest set of possible words over the entire input range. More directly, this to say that we are trying to reduce the worst case outcome of a guess over all possible correct words. 

The following words are all equally best for this objective, as no possible correct word can make a set of possible correct words greater then 183 words from these guesses. This is the global minimum from the allowable guess words.

1. ARISE
2. RAISE
3. SERAI
4. AESIR

## What about after the first guess?
We essentually repeate the above stratigy with the reduce set of possible correct words and find the guess that minimizes the largest possible set of valid words.
