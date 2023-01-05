# What's this 
A minimal Re-implementation (every thing from scratch) of almigthy bitcoin in rust. 



# Todo 
- [x] public/private keys crypto + ECDSA implementation.
- [x] serialization/deserilization of signatures and keys.
- [ ] Transactions
- ...


# Resources


## Addresses

- [Why we specify eveness of y^2=f(x) in compressed SEC format](https://bitcoin.stackexchange.com/questions/41662/on-public-keys-compression-why-an-even-or-odd-y-coordinate-corresponds-to-the-p)
DRTL: because the left side of the equation is y2, the solution for y is a square root, which can have a positive or negative value. Visually, this means that the resulting y coordinate can be above or below the x-axis. As you can see from the graph of the elliptic curve in Figure 4-2, the curve is symmetric, meaning it is reflected like a mirror by the x- axis. So, while we can omit the y coordinate we have to store the sign of y (positive or negative); or in other words, we have to remember if it was above or below the x-axis because each of those options represents a different point and a different public key.

- [How a bitcoin address is generated](https://bitcointalk.org/index.php?topic=5223167.0)
- [How a bitcoin address is generated (image)](https://en.bitcoinwiki.org/upload/en/images/thumb/a/a6/BitcoinAddress.png/700px-BitcoinAddress.png)
