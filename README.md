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

# start the web app
dx serve
```

### Deployment

I spun up an EC2 instance, to SSH:

ssh -i server-key.pem -o StrictHostKeyChecking=no ubuntu@3.92.1.71

### Testing

To test the functions needed for uni v2, you can run cargo test command

from root:

```
cargo test
```

I
