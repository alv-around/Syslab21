# Systemlab21

Systemlab21 Programming Challenge

## Commands

- Run the usual cargo commands like: `cargo build/test/run`
- For a history of the changes added to this repo run: `git log`

## Remarks

This are some of the things I have noticed while developing (the check fields
are implemented):

- [ ] The Station is a directed graph
- [ ] A valid Route should be valid path from start note to end node
- [? ] A point should be preceded by a Entry Signal and followed by an exit
  signal
- [ ] Looking at the request, in the body the state is given, so there is no
      need to store any info in the server. So the http server can be stateless.
- [ ] Since a path is also given in the request to do not need to generate a
      search algorithms (like bfs or dfs) rather just validate the path in the
      request which can be naively done in O(n).
- [ ] Although we have a nix-shell which help us with the local environment. we
      should look to containarize the application, and possible provide a helm
      chart.

## Environment setup

- If cargo is not installed in your machine and you have nix, you create a
  virtual environment by running: `nix develop`
