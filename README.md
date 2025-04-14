# Uni V2 in Dioxus/Rust

trying some uni v2 stuff in dioxus (dioxus is apparently like if nextjs and rust had a baby).

i'm using bun for dev (for tailwind deps) because it's fast - you can use whatever package you'd like.

### Development

```
# install tailwind/js components for web deployment

cd client
bun install

## NOTE ##
# you need 2 terminals to run this thing in dev (well, at the moment anyway)

# start tailwind (in client/)
bun run dev

# start the web app (client/)
dx serve

# start the rust server
cd ../server && cargo run

# alternatively if your computer can't handle all of this at once use
# https://uni-v2.hooper.link as the server
```

### Deployment

I deployed the server to an EC2 instance. To ssh in:

```
ssh -i server-key.pem -o StrictHostKeyChecking=no ubuntu@uni-v2.hooper.link

# view processes
screen -ls

```

to deploy the server:

```
# in server
cargo run

# with a different port number
cargo run -- --port 8080
```

### Testing

Functions are written after the trade function described in the uni v3 cook book: https://uniswapv3book.com/milestone_0/constant-function-market-maker.html

the trade function:

$$ (x+r\Delta x)(y-\Delta y)=k $$

is tested with differing decimal amounts each way, and seems to match the current price I get in the uniswap UI.

The errors caught (neatly) so far are:

- incorrect parameters (when calling via REST)
- pool address correct, but src or dst token address incorrect

but most other errors thrown will probably return a potentially ugly error to the user.

to run tests:

```
# in server/

cargo test

# with printlns showing
cargo test -- --nocapture
```

### TODO

- make the UI better
- build the other form of the function (what amount in for amount out)
- build that into the UI too
