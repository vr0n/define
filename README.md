# The Why
This is an extremely simple project to get acquainted with Rust and to solve a simple problem I have:
I very frequently (at least a couple of times a day) find myself running to Google to type
```
define: <some word>
```

If you don't know, when you type this into Google, it just spits up the Merriam Webster 
definition of the word instead of a matching search result. This is incredibly convenient,
but not as convenient as doing it in a terminal.

# The What
The logic of the code is extremely simple:
1. Take a single word to define as a command line argument
2. Load the GCIDE (GNU Collaborative International Dictionary of English)
3. Iterate over the dictionary line-by-line until we find the matching word
4. Print lines from the dictionary until the next blank line (In GCIDE formatting, this just means print the first definition given)

That really is all this does.

There is a file named `define.sh` in the root directory which is a shell script using `sed`
to demonstrate what the Rust code is intended to do (if you don't feel like compiling).

If everything is working as expected, `./define.sh apple` and `./define apple` should have
the same output (with one minor exception, which is that the shell script prints a blank
line after its definition, while the Rust code should not).

# Limitations
The GCIDE is poorly formatted for this task (you can look at the bizarre regex logic in the
code to see what I mean); however, I couldn't find any other text-based, open source dictionary
that was consolidated into a single file. If there are any I overlooked, I would be happy to
stand corrected on this.

The GCIDE is incredibly thorough in its definitions as well, so a self imposed limitation
is that only the first definition given is printed to the console.

To see what I mean here, open the `gcide.dict` file and perform a search for "^Apple" (The caret
is needed as the GCIDE starts word definitions at the beginning of the line. If you don't search
this way, you will find multiple hits for "Apple"). You will see that the definitions for "Apple"
span about 95 lines. Obviously, it isn't very useful for a CLI tool to print out 95 lines
of varying value simply to define what the word "Apple" means; so, the self imposed limitation
here is to just get the first definition and hope for the best. Ironically, the GCIDE's 
thoroughness is a limiting factor in using it effectively in a tool such as this.

Another self imposed limitation is that definitions for "words" that are made up of two or more
words separated by a space are not supported. As an example, the GCIDE contains the definition
for the "word" "black market". You can not use `define` to search for "black market" because
of the space, and the GCIDE does not include a full definition for the compound word
"black-market" (it does appear in the GCIDE, but it is considered an adjective while "black market"
is the noun which, presumably, would be the definition most people cared about).

I figured this was an acceptable limitation, though, it is an odd "feature" of the GCIDE to
count two words as one word. This may be how all dictionaries do it -- I haven't thought to
check prior to this project; but this is a limitation all the same.

# Other Notes
This code is currently broken. There is a limitation (an intentional limitation, of course) in Rust
concerning using Strings with non-utf8 values. Since the GCIDE handles more than utf8, you will
frequently find this code errors out. There are a couple of ways to solve this, and I am 
currently evaluating which is best. So, this problem should be gone fairly soon.
