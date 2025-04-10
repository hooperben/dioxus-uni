# Uni V2 in Dioxus

trying some uni v2 stuff in dioxus (dioxus is like if nextjs and rust had a baby).

i'm using bun for dev because it's fast - you can use whatever package you'd like.

### Development

```
# install deps
bun install

## NOTE ##
# you need 2 terminals to run this thing in dev (well, at the moment anyway)

# start tailwind
bun run dev

# start the front end
dx serve
```

### Deployment

I've currently been deploying dioxus to vercel - but full disclaimer I have absolutely no idea what it's security is. This is just testing full stack dev in rust more than anything.

### Testing

To test the functions needed for uni v2, you can run cargo test command

from root:

```
cargo test
```
