#!/usr/bin/env python3
from argparse import ArgumentParser
from random import shuffle
from secrets import choice
from string import ascii_letters, digits

alphanum = ascii_letters + digits
special_chars = "!@#$%^&*()_-+="
magic_num = 7
min_length = 5

def has_lower(word):
    return any(c.islower() for c in word)

def has_upper(word):
    return any(c.isupper() for c in word)

def has_digit(word):
    return any(c.isdigit() for c in word)

def has_special(word):
    return any(c for c in word if c in special_chars)

parser = ArgumentParser(description="Generate a complex password")
parser.add_argument("-l", default=10, metavar="length", type=int, nargs='?', help=f"password length (default: 10, minimum: {min_length})")
parser.add_argument("-s", help="require special characters (default: false)", action="store_true")

args = parser.parse_args()

if args.l < min_length:
    print(f"error: cannot generate a password below the minimum length: {min_length}\n")
    parser.print_help()
    exit(1)

charset = list(alphanum)

if args.s:
    charset += list(special_chars)
    
for _ in range(magic_num):
	shuffle(charset)

while True:
    password = ''.join(choice(charset) for _ in range(args.l))

    # restart loop if password does not have a upper, lower and digit
    if not (has_lower(password) and has_upper(password) and has_digit(password)):
        continue

    if args.s and not has_special(password):
        continue

    break

print(f"Generated: {password}")
