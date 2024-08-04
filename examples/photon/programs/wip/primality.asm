; https://stackoverflow.com/questions/42732225/fast-integer-sqrt-upper-bound-approximation

function isPrime(n){
    if n <= 1 {return false;}
    if n <= 3 {return true;}
    if n%2 == 0 || n%3 == 0 {return false;}

    // all primes are of the form 6k ± 1, with the exception of 2 and 3
    // where k is some integer, so only multiples of 6 need to be checked 
    // (twice: once for 6k − 1, once for 6k + 1)
    // that is the reason why we increase i by 6 (i=i+6) in the for loop.

    // the loop only has to test until the square root of n so (i*i<=n)
    // This is because if the square root of n is not a prime number,
    // n is not a prime number by mathematical definition. 

    for (var i=6; i*i<=n; i=i+6){
        //Divisibility test by 6k+1 and 6k-1
        if (n%(i+1) == 0 || n%(i-1) == 0) return false;
    }
        
        return true;
    }




        ; use two i's incrementing by 6
        ; instead of one and doing plus/minus

        ; can we compute the sqrt more efficiently by looking at how the value increases over time
        ; and just increment variables on each iteration accordingly?
        ; ^ that's only useful for generating list of primes though....


check_prime:
        geq n, 2        ; if n <= 1, n is not prime
        bf no
        geq n, 4        ; if n <= 3, n is prime
        bf yes

        tst n, 1        ; if n is even, n is not prime
        bf no

        sqrt z, n

loop:   b = n % i+1
        bt b, no
        b = n % i-1
        bt b, no


        add i, 6
        btd z, loop

yes:    return true

no:     return false