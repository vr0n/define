# The Why
This is an extremely simple project to get acquainted with Rust and to solve a simple problem I have:
I very frequently (at least a couple times a day) find myself running to Google to type
```
define: <some word>
```

If you don't know, when you type this into Google, it just spits up the Merriam Webster 
definition of the word instead of a matching search result. This is incredibly convenient,
but not as convenient as doing it in a terminal.

# The What
The logic of the code is extremely simple:
1. Take a single word to define as command line argument
2. Load the GCIDE (GNU Collaborative International Dictionary of English)
3. Iterate over the dictionary line-by-line until we find the matching word
4. Print the output from the dictionary to the next newline

That really is all this does.

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

# Other Notes
This code is currently broken. There is a limitation (an intentional limitation, of course) in Rust
concerning using Strings with non-utf8 values. Since the GCIDE handles more than utf8, you will
frequently find this code errors out. There are a couple of ways to solve this, and I am 
currently evaluating which is best. So, this problem should be gone fairly soon.
