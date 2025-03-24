
### ```if let``` and ```let else```

Essentially for when we want to do matches on an ```Option<T>``` for example but only care about a certain arm (i.e dont do anything if it doesnt match on said arm).

However - this is means less exhaustive checking but shorter code.

We can replicate the behaviour of ```_``` / ```other``` with ```else``` ->

``` let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
```

