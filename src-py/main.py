#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# AUTHOR:       RLogik
# URL:          https://github.com/RLogik/wordle
# DATE CREATED: 17.02.2022
# LAST EDITED:  24.02.2022
# VERSION:      (see dist/VERSION)
# DESCRIPTION:  Wordle aided solver
#   NOTE:
#   The strategies here are not as optimal as the rust version.
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# IMPORTS
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

import os;
import sys;

os.chdir(os.path.join(os.path.dirname(__file__), '..'));
sys.path.insert(0, os.getcwd());

import re;
import math;
from textwrap import dedent;
from math import inf;

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# GLOBAL CONSTANTS/VARIABLES
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

WORDLE_LENGTH = 5;
MAX_DISPLAY_LENGTH = 10;
MAX_LENGTH_FOR_BEST_OPTIMISATION = 100;
SKIP_PROMPTS = True;

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# MAIN METHOD
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

def enter(fname: str, code_green: str, code_yellow: str, code_red: str, *_):
    words = getData(fname);
    correct = {index: (a, True) for index, a in enumerate(code_green) if not(a == '-')} \
        | {index: (a, False) for index, a in enumerate(code_yellow) if not(a == '-')};
    incorrect = convertStringToList(code_red);
    words = reduceOptionsBasic(words, correct=correct, incorrect=incorrect);
    words = tacticalReductionSortByThenUniqueness(words);
    # words = tacticalReductionSortByReductionThenEntropyThenUniqueness(words);
    DisplayWords(words);
    return;

def enter_interactive(fname: str, *_):
    words = getData(fname);
    correct = {};
    incorrect = [];

    while True:
        if len(words) <= MAX_LENGTH_FOR_BEST_OPTIMISATION:
            words = tacticalReductionSortByReductionThenEntropyThenUniqueness(words);
        else:
            words = tacticalReductionSortByThenUniqueness(words);
        if len(words) <= 1:
            break;
        print('\033[4mCurrent best options:\033[0m');
        words_unique = tacticalReductionNoDuplicates(words);
        if len(words_unique) > 0:
            DisplayWords(words_unique);
        else:
            DisplayWords(words);
        print('');
        guess, feedback = NewGuess();
        UpdateState(guess=guess, feedback=feedback, correct=correct, incorrect=incorrect);
        words = reduceOptionsBasic(words, correct=correct, incorrect=incorrect);

    if len(words) == 1:
        word = wordToString(words[0]);
        print(dedentIgnoreFirstLast('''
            Only one word left. The solution must be:
                {}.
        ''').format(exprToBoxedFormatWithFeedback(guess=word.upper(), feedback='1'*len(word))));
    else:
        print('Something went wrong! No words remaining!');
    return;

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# INPUT METHODS
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

def getData(fname: str) -> list[list[str]]:
    with open(fname, 'r') as fp:
        words = fp.readlines();
        words = map(lambda x: [a for a in x.strip()], words);
        words = list(words);
        return words;

def convertStringToList(expr: str) -> list[str]:
    return [a for a in expr.lower()];

def convertStringToIndexedList(expr: str) -> dict[int,str]:
    letters = convertStringToList(expr);
    return { k: a for k, a in enumerate(letters) if re.match(r'\w', a) };

def NewGuess() -> tuple[str, str]:
    while True:
        guess = input('Enter your guess: ');
        guess = guess.strip().lower();
        if not ValidateGuess(guess):
            print(dedentIgnoreFirstLast('''
                [\033[91mERROR\033[0m] Invalid Guess!
                - Must consist of letters.
                - Length of guess must be {}.
            ''').format(WORDLE_LENGTH));
            continue;
        if SKIP_PROMPTS or PromptFeedback('Confirm that your guess was \033[1m{}\033[0m (y/n): '.format(exprToBoxedFormat(guess.upper()))):
            break;

    while True:
        feedback = input(dedentIgnoreFirstLast('''
            Enter the feedback to your guess \033[1m{guess}\033[0m:

                E.g. if the guess+feedback were
                    {example}
                    x  x  ~  x  √
                enter \033[1mxx-x1\033[0m.

            {prompt} ''').format(
            guess = exprToBoxedFormat(guess.upper()),
            example = exprToBoxedFormat('ALERT'),
            prompt = '>> ',
        ));
        if not ValidateFeedback(guess, feedback):
            print(dedentIgnoreFirstLast('''
                [\033[91mERROR\033[0m] Invalid Feedback option!
                - Format must only contain the symbols: 0, 1, -
                - Length of feedback must match length of guess.
                - Letters marked (correct|partially correct|incorrect) must be disjoint!
            '''));
            continue;
        if SKIP_PROMPTS or PromptFeedback('Confirm that the feedback was \033[1m{}\033[0m (y/n): '.format(feedback.lower())):
            break;

    print(dedentIgnoreFirstLast('''
        Your guess and its feedback were:
            {}.
    ''').format(exprToBoxedFormatWithFeedback(guess=guess.upper(), feedback=feedback)));
    return guess, feedback;

def UpdateState(
    guess:     str,
    feedback:  str,
    correct:   dict[int,str],
    incorrect: list[str],
):
    for k, a in enumerate(guess):
        match feedback[k]:
            case '0' | 'x':
                incorrect[:] = list(set(incorrect + [a]));
            case '1':
                correct[k] = (a, True);
            case '-':
                correct[k] = (a, False);
    return;

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# OUTPUT METHODS
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

def DisplayWords(words: list[list[str]]):
    print('\n'.join(map(wordToString, words[:MAX_DISPLAY_LENGTH])));
    if len(words) > MAX_DISPLAY_LENGTH:
        print('...');
    return;

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# LOGICAL METHODS
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

def ValidateGuess(guess: str) -> bool:
    if not (len(guess) == WORDLE_LENGTH):
        return False;
    if not re.match(r'^\w+$', guess):
        return False;
    return True;

def ValidateFeedback(guess: str, feedback: str) -> bool:
    if not (len(guess) == len(feedback)):
        return False;
    if re.match(r'[^0x1-]', feedback):
        return False;
    letters_correct = [a for k, a in enumerate(guess) if feedback[k] == '1'];
    letters_incorrect = [a for k, a in enumerate(guess) if feedback[k] in ['0', 'x']];
    letters_pcorrect = [a for k, a in enumerate(guess) if feedback[k] == '-'];
    if any(x in letters_incorrect for x in letters_correct):
        return False;
    if any(x in letters_pcorrect for x in letters_correct):
        return False;
    if any(x in letters_pcorrect for x in letters_incorrect):
        return False;
    return True;

def reduceOptionsBasic(
    words:            list[str],
    correct:          dict[int,tuple[str, bool]],
    incorrect:        list[str],
) -> list[list[str]]:
    words = filter(lambda w: all(w[k] == a for k, (a, placing) in correct.items() if placing), words);
    words = filter(lambda w: all(w[k] != a for k, (a, placing) in correct.items() if not placing), words);
    words = filter(lambda w: all((a in w) for a, placing in correct.values() if not placing), words);
    words = filter(lambda w: not any((a in w) for a in incorrect), words);
    words = list(words);
    return words;

def tacticalReductionSortBySizeOfReduction(words: list[str]) -> list[str]:
    '''
    Sorts by sizes of average remaining word list (lowest to highest),
    assuming all current words are equally likely to be the correct wordle.
    '''
    sizes = getAverageSizesOfRemainingWords(words);
    words = sorted(words, key = lambda w: (sizes[str(w)],));
    return words;

def tacticalReductionSortByEntropy(words: list[str]) -> list[str]:
    '''
    Sorts by entropy (highest to lowest).
    '''
    entropy = getEntropy(words);
    words = sorted(words, key = lambda w: (entropy[str(w)],));
    return words;

def tacticalReductionSortByThenUniqueness(words: list[str]) -> list[str]:
    '''
    Sorts by entropy (highest to lowest),
    then by number of unique letters (highest to lowest).
    '''
    entropy = getEntropy(words);
    words = sorted(words, key = lambda w: (-entropy[str(w)], -numberOfUniqueLettersInWord(w)));
    return words;

def tacticalReductionSortByReductionThenEntropyThenUniqueness(words: list[str]) -> list[str]:
    '''
    Sorts by remainig word list size (lowest to highest),
    then by entropy (highest to lowest),
    then by number of unique letters (highest to lowest).
    '''
    sizes = getAverageSizesOfRemainingWords(words);
    entropy = getEntropy(words);
    words = sorted(words, key = lambda w: (sizes[str(w)],-entropy[str(w)], -numberOfUniqueLettersInWord(w)));
    return words;

def tacticalSortByUniqueness(words: list[str]) -> list[str]:
    return sorted(words, key = lambda w: -numberOfUniqueLettersInWord(w));

def tacticalReductionNoDuplicates(words: list[str]) -> list[str]:
    words = list(filter(lambda w: numberOfUniqueLettersInWord(w) == len(w), words));
    return words;

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# EMPIRICAL METHODS
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

def getEntropy(words: list[list[str]]) -> dict[str, float]:
    letters = sorted(uniqueLettersInWord([ a for w in words for a in w ]));
    probs = { a: sum(len([x for x in w if x == a]) for w in words) for a in letters };
    entropy = { str(w): sum([-probs[a]*math.log(probs[a]) for a in w]) for w in words };
    return entropy;

def getAverageSizesOfRemainingWords(words: list[list[str]]) -> list[float]:
    n = len(words);
    average_size_of_remaining_words = {};
    for word in words:
        size_of_remaining_words = 0;
        for word_correct in words:
            correct = { k: (a, True) for k, a in enumerate(word) if word_correct[k] == a };
            word_correct_remaining = [ a for k, a in enumerate(word_correct) if not (k in correct.keys()) ];
            correct = correct | { k: (a, False) for k, a in enumerate(word) if a in word_correct_remaining };
            incorrect = [ word[k] for k in range(WORDLE_LENGTH) if not (k in correct.keys()) ];
            words_remaining = reduceOptionsBasic(words, correct=correct, incorrect=incorrect);
            size_of_remaining_words += len(words_remaining);
        average_size_of_remaining_words[str(word)] = size_of_remaining_words / n;
    return average_size_of_remaining_words;

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# MISCELLANEOUS
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

def wordToString(word: list[str]):
    return ''.join(word);

def uniqueLettersInWord(w: list[str]) -> list[str]:
    return list(set(w));

def numberOfUniqueLettersInWord(w: list[str]) -> list[str]:
    return len(set(w));

def exprToBoxedFormat(expr: str) -> str:
    return re.sub(r'(.)', repl=r'[\1]', string=expr);

def exprToBoxedFormatWithFeedback(guess: str, feedback: str) -> str:
    block = '';
    for k, a in enumerate(guess):
        match feedback[k]:
            case '0' | 'x':
                block += '[\033[91;1m{}\033[0m]'.format(a);
            case '1':
                block += '[\033[92;1m{}\033[0m]'.format(a);
            case '-':
                block += '[\033[93;1m{}\033[0m]'.format(a);
    return block;

def dedentIgnoreFirstLast(expr: str) -> str:
    expr = expr.lstrip('\n').rstrip('\n');
    expr = dedent(expr);
    return expr;

def removeFromArray(X: list, el) -> list:
    if el in X:
        index = X.index(el);
        X = X[:index] + X[(index+1):];
    return X;

def PromptFeedback(message: str) -> bool:
    confirm = input(message);
    if re.match(r'y|j|ja|yes', confirm):
        return True;
    return False;

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# EXECUTION
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

if __name__ == '__main__':
    args = list(sys.argv[1:]) + ['', '', '', ''];
    if args[0] == '-it':
        enter_interactive(*args[1:]);
    else:
        enter(*args);
