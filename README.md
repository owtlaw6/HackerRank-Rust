# HackerRank-Rust

### Easy Problems

1. [A Very Big Sum](https://www.hackerrank.com/challenges/a-very-big-sum/problem?isFullScreen=true)

    The function accepts a reference to an array of of 64-bit integers (&[i64]), which ensures that it can handle very large numbers. 

    I initialized a mutable variable `sum` of type i64 with 0. This variable will store the sum of the integers from the input array.

    The function iterates over each element in the input array ar using a for loop. During each iteration, the value of the current element is added to the sum.

    After processing all elements, the total sum is returned as an i64.

2. [Angry Professor](https://www.hackerrank.com/challenges/angry-professor/problem?isFullScreen=true)

    I initialized a mutable variable `onTime` to 0. This variable keeps track of the number of students who are on time.

    I iterate through the array `a` using a for loop. For each element in the array, if the value is less than or equal to 0, the `onTime` count is incremented.

    After counting the on-time students, the function compares `onTime` with `k` (the threshold).

    If `onTime` >= `k`, it means there are enough students on time, and the class is not canceled. The function returns the string "NO". Otherwise, the class is canceled, and the function returns the string "YES".

3. [Beautiful Days at the Movies](https://www.hackerrank.com/challenges/beautiful-days-at-the-movies/problem?isFullScreen=true)

    I initialized a mutable variable `days` to 0 to count the number of "beautiful days" within the range.

    I use a for loop to iterate through all integers from i to j (inclusive).

    For each day (element) in the range:
    1. A temporary variable `aux` is used to store the value of element (make a copy).
    2. A variable `rev` is initialized to 0 to compute the reverse of element.
    3. The reverse is computed by extracting the digits of `aux` one by one (using modulo 10) and constructing the reversed number (by multiplying `rev` by 10 and adding the extracted digit). This process continues until `aux` becomes 0.
    4. The function computes the absolute difference between the original day (element) and its reverse (rev) using `(element - rev).abs()`.
    5. Check if this difference is divisible by `k` using the modulo operator (%).
    If the condition is satisfied, the `days` counter is incremented.

    After processing all days in the range, the function returns the value of `days`, representing the total number of "beautiful days."


4. [Bill Division](https://www.hackerrank.com/challenges/bon-appetit/problem?isFullScreen=true)

    The total cost of all dishes in the bill array is computed using `bill.iter().sum()` which is stored in a mutable variable called `sum`.

    The cost of the dish at index `k` (which Anna did not eat) is subtracted from the total which gives the sum of the dishes that Anna and Brian actually shared.

    Since the remaining total represents the cost shared by both Anna and Brian, Anna's share is calculated by dividing the amount by 2 (`sum / 2`).

    The difference (`dif`) between the amount Anna was charged (`b`) and her actual share is calculated: `dif = b - sum / 2`.

    If `dif` is 0, it means Anna was charged correctly, and the function prints "Bon Appetit". Otherwise, it prints the value of `dif`, which represents the amount Brian owes Anna as a refund.
 
5. [Birthday Cake Candles](https://www.hackerrank.com/challenges/birthday-cake-candles/problem?isFullScreen=true)

    I initialized a mutable variable `max` to 0 to store the height of the tallest candle.

    I iterate through each element in the candles array using a for loop. If the current candle height (`element`) is greater than the current max, it updates `max` to the value of `element`.

    At the end of the loop, `max` contains the tallest candle height.

    I initialized a mutable variable `noCandles` to 0 to count the number of candles with the tallest height.

    The function iterates through the candles array again. For each candle, if its height matches the max value, `noCandles` is incremented by 1.

    After completing both loops, the function returns the value of `noCandles`, representing the number of the tallest candles from the given array.

6. [Compare the Triplets](https://www.hackerrank.com/challenges/compare-the-triplets/problem?isFullScreen=true)

    Two mutable variables, `alice` and `bob`, are initialized to 0 to keep track of the scores for Alice and Bob.

    The function compares corresponding elements in array `a` and array `b` for all three categories:

    1. If a[i] > b[i], Alice earns a point (alice += 1).
    2. If a[i] < b[i], Bob earns a point (bob += 1).
    3. If a[i] == b[i], no points are awarded for that category.

    This comparison is performed using if-else statements for each index i = 0, 1, 2 (so there are 3 ifs that check which of the 2 - alice or boob - has the biggest score for each of the 3 categories).

    I create a new vector `result`. Alice's score (`alice`) is pushed into the vector first, followed by Bob's score (`bob`).

    The function returns the `result` vector, containing Alice's and Bob's scores in this order.

7. [Counting Valleys](https://www.hackerrank.com/challenges/counting-valleys/problem?isFullScreen=true)

    I initialized a mutable variable `pos` to 0 to track the hiker's current elevation. Positive values indicate that the hiker is above sea level, negative values indicate they are below sea level, and 0 represents sea level.

    I initialized a mutable variable `valleys` to 0 to count the number of valleys traversed during the hike.

    The function iterates through each character in the string path using a for loop.

    For each step:
    1. If the step is 'D' (downhill), the elevation (`pos`) is decreased by 1.
    2. If the step is 'U' (uphill), the elevation (`pos`) is increased by 1.

    A valley is defined as a sequence of consecutive steps below sea level (`pos < 0`) that ends with a return to sea level (`pos == 0`).

    During the iteration, if an uphill step ('U') brings the hiker back to sea level (`pos == 0`), the `valleys` counter is incremented by 1.

    After processing all steps, the function returns the value of `valleys`, which represents the total number of valleys traversed by the hiker.

8. [Divisible Sum Pairs](https://www.hackerrank.com/challenges/divisible-sum-pairs/problem?isFullScreen=true)

    I initialized a variable `pairs` to 0 to count the number of valid pairs.

    A nested loop is used to generate all unique pairs of elements from the array:
    1. The outer loop iterates through each index (`first`) from 0 to `n - 2`.
    2. The inner loop iterates through each subsequent index (`second`) from `first + 1` to `n - 1`.

    For each pair (`ar[first]`, `ar[second]`):
    1. The sum of the two elements is calculated.
    2. If the sum is divisible by `k` (`(ar[first] + ar[second]) % k == 0`), the `pairs` counter is incremented by 1.

    After iterating through all pairs, the function returns the value of `pairs`, representing the total number of divisible sum pairs.

9. [Grading Students](https://www.hackerrank.com/challenges/grading/problem?isFullScreen=true)

    I initialized a new mutable vector `vec` to store the final grades after applying the rounding rules.

    The function uses a for loop to iterate through each grade (element) in the input array grades.

    For each grade, the following rules are applied:
    1. Rule 1: If the grade is less than 38, it is not rounded (i.e., failing grades are not eligible for rounding). The original grade is added to the vector.
    2. Rule 2: If the grade is 38 or higher:
    The difference between the grade and the next multiple of 5 is calculated using `5 - element % 5`. If the difference is less than 3, the grade is rounded up to the next multiple of 5.
    3. Otherwise, the grade remains unchanged.

    The rounded (or unchanged - depending on the case) grade is pushed into the `vec` vector.

    After processing all grades, the function returns the `vec` vector containing the final grades of the student.

10. [Migratory Birds](https://www.hackerrank.com/challenges/migratory-birds/problem?isFullScreen=true)

    I initialized five counters (`bird1`, `bird2`, `bird3`, `bird4`, `bird5`) to 0 to track the number of sightings for each bird type (1 through 5).

    A for loop iterates through each element in the input array `arr`.

    Using a match statement, the corresponding counter is incremented based on the bird type (1, 2, 3, 4, or 5).

    The function compares the counts for each bird type (bird1, bird2, ..., bird5) to determine which bird type has the highest frequency.

    If two or more bird types have the same highest frequency, the function ensures the smallest bird type ID is returned. This is achieved by evaluating the conditions in ascending order of bird type IDs.

    The function returns the ID of the most frequently observed bird type as an integer.

11. [Mini-Max Sum](https://www.hackerrank.com/challenges/mini-max-sum/problem?isFullScreen=true)

    I initialized a mutable variable `total` to 0 (as a 64-bit integer i64 to handle potential overflow for large integers).

    A for loop iterates through the array, adding each element to `total`.

    I initialized two variables, `min` and `max`:
    1. `min` is set to the maximum possible value of i64 (i64::MAX) to ensure it is updated with the smallest calculated sum.
    2. `max` is set to the minimum possible value of i64 (i64::MIN) to ensure it is updated with the largest calculated sum.

    A second for loop iterates through the array. For each element:
    1. The sum of the other four elements is calculated by subtracting the current element from total (`current_sum = total - *number as i64`).
    2. If `current_sum` is smaller than `min`, the value of `min` is updated.
    3. If `current_sum` is larger than `max`, the value of `max` is updated.

    After processing all elements, the function prints `min` and `max` separated by a space.

12. [Number Line Jumps](https://www.hackerrank.com/challenges/kangaroo/problem?isFullScreen=true)

    I initialized two mutable variables, `posK1` and `posK2` with the starting positions of Kangaroo 1 and Kangaroo 2, respectively.

    I initialized a mutable variable `distance` with the absolute difference between the two starting positions. This tracks the initial distance between the kangaroos.

    A while loop runs as long as the two kangaroos have not landed on the same position (`posK1 != posK2`).
    
    At each step:
    1. `posK1` is incremented by `v1` (Kangaroo 1’s jump distance).
    2. `posK2` is incremented by `v2` (Kangaroo 2’s jump distance).

    After each jump, the new distance between the kangaroos is compared to the previous distance.

    If the new distance is greater than or equal to the previous distance, it indicates that the kangaroos are moving further apart or maintaining the same distance, meaning they will never meet. The function returns the string "NO".

    If the kangaroos eventually land on the same position (posK1 == posK2), the function returns the string "YES".

13. [Picking Numbers](https://www.hackerrank.com/challenges/picking-numbers/problem?isFullScreen=true)

    I initialized a mutable variable `maxLen` to 0. This variable will store the length of the longest subarray satisfying the condition.

    The outer loop iterates through each unique value in the array (`num`). For each `num`, the function checks how many elements are equal to `num` or `num + 1`.

    Two counters, `count0` and `count1`, are used:
    1. `count0`: Counts elements in the array equal to `num`.
    2. `count1`: Counts elements in the array equal to `num + 1`.

    An inner loop iterates through the array again to count occurrences of `num` and `num + 1`.

    For each `num`, the total length of the subarray is calculated as the sum of `count0` and `count1`.

    If the calculated length (`total`) is greater than the current `maxLen`, the function updates `maxLen` with the new value.

    After iterating through all elements in the array, the function returns `maxLen`, which is the length of the longest valid subarray.

14. [Sales by Match](https://www.hackerrank.com/challenges/sock-merchant/problem?isFullScreen=true)

    I initialized a mutable variable `pairs` to 0 and a mutable fixed-size array `vec` of length 105 (sock colors range from 1 to 100 and I added some extra space just to be safe) with zeros.

    Each index of the array represents a sock color, and its value tracks the count of socks for that color.

    A for loop iterates through the input array `ar`.

    For each sock color (element), the corresponding index in the vec array is incremented by 1, counting the number of socks of each color.

    A second loop iterates through the `vec` array.
    
    For each sock color's count (`element` in `vec`), the number of pairs is calculated as `element / 2` (integer division).

    The total number of pairs is accumulated in the `pairs` variable.

    After processing all sock colors, the function returns the total number of pairs, the `pairs` variable.

15. [Simple Array Sum](https://www.hackerrank.com/challenges/simple-array-sum/problem?isFullScreen=true)

    I initialized a mutable variable `sum` of type i32 to 0. This variable accumulates the sum of the elements in the array.

    A for loop iterates over each element in the array `ar`.

    For every element, its value is added to the `sum` variable.

    After processing all elements in the array, the final value of `sum` is returned as the result.

16. [Staircase](https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=true)

    The outer loop iterates from 1 to n (inclusive). The variable `number` represents the current row number, which determines how many # characters and spaces are printed.

    For each row, a loop iterates from 1 to `n - number + 1` to print spaces (" "). The spaces ensure that the # characters are right-aligned in the staircase.

    After the spaces, another loop iterates from 1 to `number + 1` to print # characters. The number of # characters increases with each row.

    After printing the spaces and # characters for a row, a newline (println!()) is added to start the next row.

17. [The Hurdle Race](https://www.hackerrank.com/challenges/the-hurdle-race/problem?isFullScreen=true)

    I initialized a mutable variable `max` to 0.
    
    A for loop iterates through each element in the `height` array. If the current hurdle height is greater than `max`, `max` is updated to that value.

    After the loop, `max` contains the height of the tallest hurdle.

    The difference between the tallest hurdle (`max`) and the character’s maximum natural jump height (`k`) is calculated and stored in the mutable variable `potions`.

    If the character's jump height (`k`) is greater than or equal to `max`, `potions` will be negative or zero. In this case, the function returns 0, indicating no potions are required.

    If `potions` is positive, it represents the exact number of potions needed to match the difference in height, and the function returns this value.

    The function returns the value of `potions`, which represents the minimum number of potions needed to clear all hurdles.

18. [Utopian Tree](https://www.hackerrank.com/challenges/utopian-tree/problem?isFullScreen=true)

    I initialized a mutable variable `meters` to 1, representing the tree's initial height in meters before any growth cycles.

    A for loop iterates from 0 to n - 1 (inclusive), where each iteration represents a growth cycle.

    Even Cycles: If the cycle number (`element`) is even, the tree height doubles (`meters *= 2`).
    
    Odd Cycles: If the cycle number is odd, the tree height increases by 1 meter (`meters += 1`).

    After completing all growth cycles, the function returns the value of `meters`, representing the tree's height after `n` cycles.

19. [Viral Advertising](https://www.hackerrank.com/challenges/strange-advertising/problem?isFullScreen=true)

    `shared`: Represents the number of people the advertisement is shared with at the start of each day. It is initialized to 5 since the advertisement is shared with 5 people on the first day.

    `liked`: Represents the number of people who like the advertisement each day. It is initialized to 2 because 50% of the 5 people (rounded down) like the advertisement on the first day.

    `cumuluative`: Represents the total number of likes accumulated over all days. It is initialized to 2, as 2 people like the advertisement on the first day.

    A for loop iterates from 1 to n - 1, simulating the campaign for each subsequent day:

    1. Calculate `shared`: The number of people the advertisement is shared with is updated. Each person who liked the advertisement the previous day shares it with 3 new people. This is done by multiplying `shared` by 3.
    2. Calculate `liked`: Half (rounded down) of the people it is shared with on the current day like the advertisement. This is calculated as `shared / 2`.
    3. Update cumulative likes: The liked value for the current day is added to `cumuluative`.

    After iterating through all `n` days, the function returns the value of `cumuluative`, which represents the total number of likes accumulated during the campaign.

### Medium Problems

1. [Extra Long Factorials](https://www.hackerrank.com/challenges/extra-long-factorials/problem?isFullScreen=true)

    Factorials grow extremely large, exceeding the maximum size of standard integer types (e.g., i32 or i64) for relatively small values of `n`. To handle these large values, the function uses the BigUint type from the num-bigint crate, which is designed for arbitrarily large unsigned integers.
    
    I initialized a variable `sol` to 1 using BigUint::from(1u32). This variable will store the computed factorial as the loop progresses.

    A for loop iterates from 1 to n (inclusive).

    On each iteration, `sol` is multiplied by the current value of `number`, which is converted to BigUint using BigUint::from_i32(number).unwrap().

    The unwrap() ensures the conversion succeeds, as all number values are valid for this operation.

    After the loop completes, `sol` contains the factorial of `n`. The function prints the result using println!.