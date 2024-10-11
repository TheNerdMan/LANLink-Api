# LANLink
Connecting players to LAN game servers

## Live Reload
Make sure you have cargo-watch and systemfds installed
`cargo install cargo-watch systemfd`
Then you can run 
`systemfd --no-pid -s http::3000 -- cargo watch -x run`