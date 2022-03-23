# Rusty Random Pattern Generator


Generates a random pattern and saves it as `image.png` in root. Run:

```
cargo run [seed]
```

Max seed is `U64_MAX=18_446_744_073_709_551_615u64`. FFS it's unsigned don't give it a negative integer, or a float!

Built to be used with an NFT.