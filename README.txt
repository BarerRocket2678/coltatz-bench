Inputting values:

Threads: The number of threads the program will use. The program will report very inaccurate values if the search size is not much higher than the number of threads, so set it much higher than the number of threads.

Search size: The number of numbers the program will calculate the collatz-conjecture up to. A value of around 10,000,000 seems to yield consistent results

Repeats: How many times the program will repeat calculation the conjecture up to the search size. A value of at least 3 is needed to see the median, as well as other data like throttling detection. If the difference between the mean and the median is large, you should increase this value. If you increase this value and the difference between the median and mode is still large, it may signify processor instability. (I'm looking at you Alex Clark!)


Output:

Data: The benchmark scores your cpu based on how many times 

All output values are based on all of the Repeats.
