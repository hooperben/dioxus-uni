# Uni V2 in Dioxus/Rust Server

trying some uni v2 stuff in dioxus (dioxus is like if nextjs and rust had a baby).

i'm using bun for dev because it's fast - you can use whatever package you'd like.

### Development

```
# install tailwind/js components
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
```

### Deployment

I deployed the server to an EC2 instance. ssh:

```
ssh -i server-key.pem -o StrictHostKeyChecking=no ubuntu@3.92.1.71
```

### Testing

TODO
